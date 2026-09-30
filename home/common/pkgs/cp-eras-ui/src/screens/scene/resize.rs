//! A temporary, bounded cached-material preview during a window resize.
//! Only a resting dashboard opts in; the ordinary backdrop stays exact.

use super::*;
use crate::palette::Palette;
use iced::window;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Condvar;
use std::time::Instant;

// Headless compositor delivery can space resize events about 150ms apart.
// Leave room for that cadence before preparing the final exact raster.
const QUIET: Duration = Duration::from_millis(200);
const MIN_SIZE: (u32, u32) = (2560, 1440);
const MIN_RATIO: f32 = 0.90;
const MAX_RATIO: f32 = 1.10;
const PREPARE_POLL: Duration = Duration::from_millis(33);
static PREVIEW_DRAWS: AtomicUsize = AtomicUsize::new(0);
static EXACT_DRAWS: AtomicUsize = AtomicUsize::new(0);
static NEXT_OWNER: AtomicU64 = AtomicU64::new(1);
static PREPARE_WORKER: OnceLock<Option<Arc<PrepareWorker>>> = OnceLock::new();

fn trace_enabled() -> bool {
    std::env::var_os("CP_ERAS_UI_RESIZE_TRACE").is_some()
}

fn trace(message: &str, size: (u32, u32)) {
    if trace_enabled() {
        eprintln!("resize-backdrop {message} {}x{}", size.0, size.1);
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Context {
    prims: PrimKey,
    palette: Palette,
}

impl Context {
    fn of(backdrop: &Backdrop) -> Self {
        Self { prims: PrimKey::new(backdrop.prims), palette: backdrop.style.palette }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct PrepareKey {
    context: Context,
    size: (u32, u32),
    scale: u32,
}

impl PrepareKey {
    fn new(context: Context, size: (u32, u32), k: f32) -> Self {
        Self { context, size, scale: k.to_bits() }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct JobId {
    owner: u64,
    generation: u64,
    key: PrepareKey,
}

struct PrepareJob {
    id: JobId,
    layers: Vec<(&'static [Prim], bool)>,
}

#[derive(Default)]
struct PrepareQueue {
    desired: Option<JobId>,
    pending: Option<PrepareJob>,
    running: Option<JobId>,
    published: Option<JobId>,
    generation: u64,
    failed: bool,
}

impl PrepareQueue {
    fn submit(&mut self, owner: u64, key: PrepareKey,
        layers: Vec<(&'static [Prim], bool)>) -> bool
    {
        if self.failed { return false; }
        if self.desired.is_some_and(|id| id.owner == owner && id.key == key)
            && (self.running == self.desired
                || self.pending.as_ref().is_some_and(|job| Some(job.id) == self.desired))
        {
            return true;
        }
        if self.desired.is_some() && self.desired == self.published
            && self.desired.is_some_and(|id| id.owner == owner && id.key == key)
        {
            return false; // Published then evicted: do not poll/rebuild forever.
        }
        self.generation = self.generation.wrapping_add(1);
        let id = JobId { owner, generation: self.generation, key };
        self.desired = Some(id);
        self.pending = Some(PrepareJob { id, layers });
        self.published = None;
        true
    }

    fn invalidate_owner(&mut self, owner: u64) {
        if self.desired.is_some_and(|id| id.owner == owner) {
            self.desired = None;
            self.pending = None;
            self.published = None;
        }
    }

    fn waiting_for(&self, owner: u64, key: PrepareKey) -> bool {
        self.desired.is_some_and(|id| id.owner == owner && id.key == key
            && (self.running == Some(id)
                || self.pending.as_ref().is_some_and(|job| job.id == id)))
    }

    fn take(&mut self) -> Option<PrepareJob> {
        let job = self.pending.take()?;
        self.running = Some(job.id);
        Some(job)
    }

    fn finish(&mut self, id: JobId) {
        if self.running == Some(id) { self.running = None; }
    }

    fn fail(&mut self) {
        self.failed = true;
        self.desired = None;
        self.pending = None;
        self.running = None;
        self.published = None;
    }
}

struct PrepareWorker {
    queue: Mutex<PrepareQueue>,
    ready: Condvar,
}

#[derive(Clone, Copy)]
enum WorkerSource<'a> {
    Shared,
    // Deterministic tests inject a worker without starting the global thread.
    #[allow(dead_code)]
    Borrowed(&'a PrepareWorker),
}

impl WorkerSource<'_> {
    fn get(&self, start: bool) -> Option<&PrepareWorker> {
        match self {
            Self::Borrowed(worker) => Some(worker),
            Self::Shared if start => PrepareWorker::shared().map(Arc::as_ref),
            Self::Shared => PrepareWorker::existing().map(Arc::as_ref),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JobResult { Published, Stale, Rejected }

impl PrepareWorker {
    fn shared() -> Option<&'static Arc<Self>> {
        PREPARE_WORKER.get_or_init(|| {
            let worker = Arc::new(Self {
                queue: Mutex::new(PrepareQueue::default()),
                ready: Condvar::new(),
            });
            let background = Arc::clone(&worker);
            std::thread::Builder::new().name("neomil-resize-prepare".into())
                .spawn(move || background.run()).ok().map(|_| worker)
        }).as_ref()
    }

    fn existing() -> Option<&'static Arc<Self>> {
        PREPARE_WORKER.get().and_then(Option::as_ref)
    }

    fn submit(&self, owner: u64, key: PrepareKey,
        layers: Vec<(&'static [Prim], bool)>) -> bool
    {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        let old = queue.generation;
        let accepted = queue.submit(owner, key, layers);
        if accepted && queue.generation != old { self.ready.notify_one(); }
        accepted
    }

    fn invalidate_owner(&self, owner: u64) {
        self.queue.lock().unwrap_or_else(|e| e.into_inner()).invalidate_owner(owner);
    }

    fn waiting_for(&self, owner: u64, key: PrepareKey) -> bool {
        self.queue.lock().unwrap_or_else(|e| e.into_inner()).waiting_for(owner, key)
    }

    fn execute_job(&self, job: &PrepareJob, shared: &SoftCache,
        build: impl FnOnce(&PrepareJob) -> SoftCache) -> bool
    {
        let tracing = trace_enabled();
        let started = tracing.then(Instant::now);
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let scratch = build(job);
            let local_bytes = if tracing {
                scratch.state.lock().unwrap_or_else(|e| e.into_inner()).bytes
            } else { 0 };
            let keys = layer_keys(&job.layers, &job.id.key.context.palette,
                job.id.key.size, f32::from_bits(job.id.key.scale));
            let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
            let result = if queue.desired != Some(job.id) {
                JobResult::Stale
            } else if shared.publish_complete(&scratch, &keys) {
                queue.published = Some(job.id);
                JobResult::Published
            } else {
                JobResult::Rejected
            };
            queue.finish(job.id);
            let shared_bytes = if tracing {
                shared.state.lock().unwrap_or_else(|e| e.into_inner()).bytes
            } else { 0 };
            (result, local_bytes, shared_bytes)
        }));
        if let Some(started) = started {
            match &outcome {
                Ok((result, local_bytes, shared_bytes)) => eprintln!(
                    "resize-backdrop job={result:?} owner={} generation={} target={}x{} elapsed_ms={:.3} local_cache_bytes={} retained_cache_bytes={}",
                    job.id.owner, job.id.generation, job.id.key.size.0, job.id.key.size.1,
                    started.elapsed().as_secs_f64() * 1000.0, local_bytes, shared_bytes),
                Err(_) => eprintln!(
                    "resize-backdrop job=panic owner={} generation={} target={}x{} elapsed_ms={:.3}",
                    job.id.owner, job.id.generation, job.id.key.size.0, job.id.key.size.1,
                    started.elapsed().as_secs_f64() * 1000.0),
            }
        }
        match outcome {
            Ok((JobResult::Published | JobResult::Stale, _, _)) => true,
            Ok((JobResult::Rejected, _, _)) | Err(_) => {
                self.queue.lock().unwrap_or_else(|e| e.into_inner()).fail();
                false
            }
        }
    }

    fn run(&self) {
        loop {
            let job = {
                let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                while queue.pending.is_none() {
                    queue = self.ready.wait(queue).unwrap_or_else(|e| e.into_inner());
                }
                queue.take().expect("pending job checked")
            };
            let healthy = self.execute_job(&job, SoftCache::shared(), |job| {
                // The ordinary CPU compositor runs on a job-local cache. No
                // shared cache lock or UI-thread renderer is held here.
                let scratch = SoftCache::with_budget(SOFT_CACHE_BYTES);
                scratch.prepare(&job.layers, &job.id.key.context.palette,
                    job.id.key.size, f32::from_bits(job.id.key.scale));
                scratch
            });
            if !healthy { return; } // UI redraw takes the exact fallback.
        }
    }
}

/// A canvas owns only its matching worker generation. Dropping a route's
/// widget state cannot cancel another canvas that has become current.
pub(super) struct ResizeState {
    owner: u64,
    context: Option<Context>,
    deadline: Option<Instant>,
    target: Option<PrepareKey>,
}

impl Default for ResizeState {
    fn default() -> Self {
        Self {
            owner: NEXT_OWNER.fetch_add(1, Ordering::Relaxed),
            context: None,
            deadline: None,
            target: None,
        }
    }
}

impl Drop for ResizeState {
    fn drop(&mut self) {
        if let Some(worker) = PrepareWorker::existing() {
            worker.invalidate_owner(self.owner);
        }
    }
}

impl ResizeState {
    fn observed_target(&mut self, key: PrepareKey, now: Instant, context: Context) -> Option<Instant> {
        let previous = self.target.replace(key);
        (previous.is_some() && previous != Some(key)).then(|| self.resized(now, context))
    }

    fn resized(&mut self, now: Instant, context: Context) -> Instant {
        let deadline = now + QUIET;
        self.context = Some(context);
        self.deadline = Some(deadline);
        deadline
    }

    fn redraw(&mut self, now: Instant, context: Context) -> Option<Instant> {
        if self.context.is_some_and(|old| old != context) {
            self.clear();
            return None;
        }
        match self.deadline {
            Some(deadline) if now < deadline => Some(deadline),
            _ => {
                self.deadline = None;
                None
            }
        }
    }

    fn preview(&self, context: Context, key: PrepareKey) -> bool {
        // `update` clears expiry before draw. Its event timestamp may lead
        // draw's wall clock slightly, so the stored deadline is authoritative.
        self.context == Some(context) && self.target == Some(key) && self.deadline.is_some()
    }

    fn clear(&mut self) {
        self.context = None;
        self.deadline = None;
        self.target = None;
    }
}

pub(super) struct ResizeBackdrop {
    backdrop: Backdrop,
}

impl ResizeBackdrop {
    pub(super) fn new(backdrop: Backdrop) -> Self {
        debug_assert!(!backdrop.stretch);
        Self { backdrop }
    }

    fn update_with<M>(
        &self,
        state: &mut ResizeState,
        event: &iced::Event,
        bounds: Rectangle,
        soft: &SoftCache,
        workers: WorkerSource<'_>,
    ) -> Option<canvas::Action<M>> {
        let context = Context::of(&self.backdrop);
        let k = scale(bounds);
        let size = ((FRAME.0 * k).round().max(1.0) as u32,
            (FRAME.1 * k).round().max(1.0) as u32);
        let key = PrepareKey::new(context, size, k);
        if state.context.is_some_and(|old| old != context) {
            state.clear();
            if let Some(worker) = workers.get(false) {
                worker.invalidate_owner(state.owner);
            }
            return Some(canvas::Action::request_redraw());
        }
        match event {
            iced::Event::Window(window::Event::Resized(_)) => {
                state.target = Some(key);
                if let Some(worker) = workers.get(false) {
                    worker.invalidate_owner(state.owner);
                }
                let at = state.resized(Instant::now(), context);
                trace("event=resize deadline=renewed", size);
                Some(canvas::Action::request_redraw_at(at))
            }
            iced::Event::Window(window::Event::RedrawRequested(now)) => {
                // Iced canvas receives this with current layout bounds
                // before drawing; Window::Resized can arrive afterward.
                if let Some(at) = state.observed_target(key, *now, context) {
                    if let Some(worker) = workers.get(false) {
                        worker.invalidate_owner(state.owner);
                    }
                    trace("event=bounds deadline=renewed", size);
                    return Some(canvas::Action::request_redraw_at(at));
                }
                let active = state.deadline.is_some();
                let next = state.redraw(*now, context);
                if active && next.is_none() {
                    trace("event=redraw deadline=cleared", size);
                }
                if let Some(at) = next {
                    return Some(canvas::Action::request_redraw_at(at));
                }
                if state.context == Some(context)
                    && k > 0.0 && size.0 >= MIN_SIZE.0 && size.1 >= MIN_SIZE.1
                {
                    let layers = soft_layers(self.backdrop.prims);
                    if soft.can_prepare(layers.len(), size)
                        && !soft.complete(&layers, &self.backdrop.style.palette, size, k)
                        && soft.preview_snapshot(&layers, &self.backdrop.style.palette, size).is_some()
                    {
                        if workers.get(true).is_some_and(|worker|
                            worker.submit(state.owner, key, layers))
                        {
                            return Some(canvas::Action::request_redraw_at(*now + PREPARE_POLL));
                        }
                    }
                }
                None
            }
            iced::Event::Window(window::Event::Rescaled(_) | window::Event::Unfocused) => {
                let pending = workers.get(false).is_some_and(|worker|
                    worker.waiting_for(state.owner, key));
                let was_preview = state.deadline.is_some() || pending;
                state.clear();
                if let Some(worker) = workers.get(false) {
                    worker.invalidate_owner(state.owner);
                }
                was_preview.then(canvas::Action::request_redraw)
            }
            _ => None,
        }
    }

    fn draw_snapshot(&self, renderer: &Renderer, bounds: Rectangle,
        snapshot: PreviewSnapshot, target: (u32, u32), k: f32, exact: bool,
        started: Option<Instant>)
        -> Vec<canvas::Geometry>
    {
        debug_assert!(!exact || snapshot.size == target);
        let source = snapshot.size;
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let mut images = snapshot.layers.iter();
        for prim in leading_soft(self.backdrop.prims) {
            self.paint_preview(&mut frame, prim, &mut images,
                snapshot.size, target, k, None);
        }
        debug_assert!(images.next().is_none());
        let geometry = frame.into_geometry();
        if let Some(started) = started {
            if exact {
                let count = EXACT_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("resize-backdrop draw=exact_snapshot count={count} size={}x{} elapsed_ms={:.3}",
                    target.0, target.1, started.elapsed().as_secs_f64() * 1000.0);
            } else {
                let count = PREVIEW_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("resize-backdrop draw=preview count={count} source={}x{} target={}x{} elapsed_ms={:.3}",
                    source.0, source.1, target.0, target.1,
                    started.elapsed().as_secs_f64() * 1000.0);
            }
        }
        vec![geometry]
    }

    /// Same Soft/Motion traversal and clip as `Backdrop::paint`, using one
    /// coherent image set. Preview scales an old set; an exact snapshot uses
    /// source == target. Advance the layer cursor even behind an empty clip.
    fn paint_preview(
        &self,
        frame: &mut canvas::Frame,
        prim: &'static Prim,
        layers: &mut std::slice::Iter<'_, Bands>,
        source: (u32, u32),
        target: (u32, u32),
        k: f32,
        region: Option<Rectangle>,
    ) {
        match *prim {
            Prim::Soft { .. } => {
                let bands = layers.next().expect("preview has every ordered Soft layer");
                let sy = target.1 as f32 / source.1 as f32;
                let draw = |frame: &mut canvas::Frame| {
                    for (y, h, handle) in bands.iter() {
                        let image = canvas::Image::new(handle.clone())
                            .filter_method(iced::widget::image::FilterMethod::Linear);
                        let bounds = Rectangle {
                            x: 0.0,
                            y: *y as f32 * sy,
                            width: target.0 as f32,
                            height: *h as f32 * sy,
                        };
                        frame.draw_image(bounds, image);
                    }
                };
                match region {
                    Some(region) => frame.with_clip(region, draw),
                    None => draw(frame),
                }
            }
            Prim::Motion { motion, prims } => {
                let t = motion::progress(&motion, self.backdrop.at);
                let region = match motion.change {
                    Change::Clip { x, y, w, h } => {
                        let own = Rectangle {
                            x: x * k,
                            y: y * k,
                            width: Change::lerp(w, t) * k,
                            height: Change::lerp(h, t) * k,
                        };
                        match region {
                            Some(outer) => outer.intersection(&own)
                                .unwrap_or(Rectangle::new(Point::ORIGIN, Size::ZERO)),
                            None => own,
                        }
                    }
                    Change::Opacity { .. } => region.unwrap_or(Rectangle::new(
                        Point::ORIGIN,
                        Size::new(frame.width(), frame.height()),
                    )),
                };
                for prim in prims {
                    self.paint_preview(frame, prim, layers, source, target, k, Some(region));
                }
            }
            _ => {}
        }
    }
}

impl<M> canvas::Program<M, Style> for ResizeBackdrop {
    type State = ResizeState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<canvas::Action<M>> {
        self.update_with(state, event, bounds, SoftCache::shared(), WorkerSource::Shared)
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Style,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Include key construction, cache locking and snapshot acquisition,
        // not just submission of the already prepared image handles.
        let started = trace_enabled().then(Instant::now);
        let k = scale(bounds);
        if k > 0.0 {
            let target = ((FRAME.0 * k).round().max(1.0) as u32,
                          (FRAME.1 * k).round().max(1.0) as u32);
            if target.0 >= MIN_SIZE.0 && target.1 >= MIN_SIZE.1 {
                let soft = SoftCache::shared();
                let layers = soft_layers(self.backdrop.prims);
                if let Some(exact) = soft.exact_snapshot(&layers,
                    &self.backdrop.style.palette, target, k)
                {
                    return self.draw_snapshot(renderer, bounds, exact, target, k, true, started);
                }
                let context = Context::of(&self.backdrop);
                if state.context == Some(context) {
                    if let Some(preview) = soft.preview_snapshot(&layers,
                        &self.backdrop.style.palette, target)
                    {
                        let key = PrepareKey::new(context, target, k);
                        let quiet = state.preview(context, key);
                        if quiet || (soft.can_prepare(layers.len(), target)
                            && PrepareWorker::shared().is_some_and(|worker|
                                worker.submit(state.owner, key, layers)))
                        {
                            return self.draw_snapshot(renderer, bounds, preview, target, k, false, started);
                        }
                    }
                }
            }
        }
        let diagnostic = (trace_enabled() && k > 0.0).then(|| {
            let size = ((FRAME.0 * k).round().max(1.0) as u32,
                (FRAME.1 * k).round().max(1.0) as u32);
            let soft = SoftCache::shared();
            let complete = soft.complete(&soft_layers(self.backdrop.prims),
                &self.backdrop.style.palette, size, k);
            (size, complete, Instant::now())
        });
        let result = <Backdrop as canvas::Program<M, Style>>::draw(
            &self.backdrop, &(), renderer, theme, bounds, cursor,
        );
        if let Some((size, complete_before, start)) = diagnostic {
            let soft = SoftCache::shared();
            let retained = soft.state.lock().unwrap_or_else(|e| e.into_inner()).bytes;
            let count = EXACT_DRAWS.fetch_add(1, Ordering::Relaxed) + 1;
            eprintln!("resize-backdrop draw=exact count={count} size={}x{} cache_before={} elapsed_ms={:.3} retained_cache_bytes={retained}",
                size.0, size.1, if complete_before { "complete" } else { "missing" },
                start.elapsed().as_secs_f64() * 1000.0);
        }
        result
    }
}

struct PreviewSnapshot {
    size: (u32, u32),
    layers: Vec<Bands>,
}

/// Build the same ordered keys `prepare` uses, with each cut carrying all
/// preceding groups. A preview source is accepted only if every key exists.
fn layer_keys(
    layers: &[(&'static [Prim], bool)],
    palette: &Palette,
    size: (u32, u32),
    k: f32,
) -> Vec<SoftKey> {
    let mut under = Vec::with_capacity(layers.len());
    layers.iter().map(|&(prims, cut)| {
        let key = SoftKey::new(prims, palette, size, k, cut.then_some(under.as_slice()));
        under.push(prims);
        key
    }).collect()
}

impl SoftCache {
    /// Clone one coherent exact set under a single cache lock. Painting it
    /// does not invoke `Backdrop::draw` or any cache-miss renderer afterward.
    fn exact_snapshot(
        &self,
        layers: &[(&'static [Prim], bool)],
        palette: &Palette,
        size: (u32, u32),
        k: f32,
    ) -> Option<PreviewSnapshot> {
        let keys = layer_keys(layers, palette, size, k);
        if keys.is_empty() { return None; }
        let mut cache = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if !keys.iter().all(|key| cache.entries.iter().any(|entry| entry.key == *key)) {
            return None;
        }
        let bands = keys.iter().map(|key|
            cache.find(key).expect("complete exact set checked under lock")
        ).collect();
        Some(PreviewSnapshot { size, layers: bands })
    }

    /// Publish all ordered images from one job-local cache in one lock hold.
    /// Reject before mutation if the batch cannot fit; duplicate keys are
    /// upserted once so bytes and entry count remain exact.
    fn publish_complete(&self, scratch: &SoftCache, keys: &[SoftKey]) -> bool {
        let local = scratch.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut batch: Vec<SoftEntry> = Vec::with_capacity(keys.len());
        for key in keys {
            if batch.iter().any(|entry| entry.key == *key) { continue; }
            let Some(entry) = local.entries.iter().find(|entry| entry.key == *key) else {
                return false;
            };
            batch.push(SoftEntry {
                key: entry.key.clone(), bytes: entry.bytes,
                bands: Arc::clone(&entry.bands),
            });
        }
        let Some(bytes) = batch.iter().try_fold(0usize,
            |sum, entry| sum.checked_add(entry.bytes)) else {
            return false;
        };
        if batch.is_empty() || batch.len() > SOFT_CACHE_ENTRIES || bytes > self.budget {
            return false;
        }
        drop(local);

        let mut cache = self.state.lock().unwrap_or_else(|e| e.into_inner());
        // Upsert a batch atomically; do not count replaced entries twice.
        for entry in &batch {
            if let Some(index) = cache.entries.iter().position(|old| old.key == entry.key) {
                let old = cache.entries.remove(index);
                cache.bytes -= old.bytes;
            }
        }
        while cache.bytes > self.budget - bytes
            || cache.entries.len() > SOFT_CACHE_ENTRIES - batch.len()
        {
            let old = cache.entries.remove(0);
            cache.bytes -= old.bytes;
        }
        cache.bytes += bytes;
        cache.entries.extend(batch);
        true
    }

    fn complete(
        &self,
        layers: &[(&'static [Prim], bool)],
        palette: &Palette,
        size: (u32, u32),
        k: f32,
    ) -> bool {
        let keys = layer_keys(layers, palette, size, k);
        let cache = self.state.lock().unwrap_or_else(|e| e.into_inner());
        keys.iter().all(|key| cache.entries.iter().any(|entry| entry.key == *key))
    }

    fn preview_snapshot(
        &self,
        layers: &[(&'static [Prim], bool)],
        palette: &Palette,
        target: (u32, u32),
    ) -> Option<PreviewSnapshot> {
        let first = layers.first()?;
        let mut cache = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let source = cache.entries.iter().rev().find_map(|entry| {
            if entry.key.prims != PrimKey::new(first.0) || entry.key.palette != *palette {
                return None;
            }
            let size = entry.key.size;
            let rx = target.0 as f32 / size.0 as f32;
            let ry = target.1 as f32 / size.1 as f32;
            if !(MIN_RATIO..=MAX_RATIO).contains(&rx)
                || !(MIN_RATIO..=MAX_RATIO).contains(&ry)
            {
                return None;
            }
            let k = f32::from_bits(entry.key.scale);
            let keys = layer_keys(layers, palette, size, k);
            (keys[0] == entry.key && keys.iter().all(|key|
                cache.entries.iter().any(|candidate| candidate.key == *key)
            )).then_some((size, keys))
        })?;
        // Move the complete source set to the MRU end as one decision. The
        // only retained payload remains in SoftCache; this vector clones Arc
        // handles for the current draw, not RGBA or persistent entries.
        let bands = source.1.iter().map(|key|
            cache.find(key).expect("complete preview source was checked under this lock")
        ).collect();
        Some(PreviewSnapshot { size: source.0, layers: bands })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{fill_rect, Era, Ink};

    static GROUND: &[Prim] = &[fill_rect(0.0, 0.0, 4.0, 4.0, Ink::Bg)];
    static GHOST: &[Prim] = &[fill_rect(1.0, 1.0, 2.0, 2.0, Ink::Fg)];

    fn context() -> Context {
        let style = Era::Neomil.style().dashboard_style();
        Context { prims: PrimKey::new(style.dashboard), palette: style.palette }
    }

    fn key(size: (u32, u32)) -> PrepareKey {
        PrepareKey::new(context(), size, 1.0)
    }

    fn prepared_job_cache(job: &PrepareJob) -> SoftCache {
        let cache = SoftCache::with_budget(1024);
        let keys = layer_keys(&job.layers, &job.id.key.context.palette,
            job.id.key.size, f32::from_bits(job.id.key.scale));
        for key in keys {
            let width = key.size.0 as usize;
            cache.keep(key, vec![soft::Band {
                y: 0, h: 1, rgba: vec![255; width * 4],
            }]);
        }
        cache
    }

    #[test]
    fn initial_target_survives_redraw_without_resize_event() {
        let now = Instant::now();
        let context = context();
        let mut state = ResizeState::default();
        let initial = key((3840, 2160));
        assert_eq!(state.observed_target(initial, now, context), None);
        assert_eq!(state.redraw(now, context), None);
        assert!(state.target == Some(initial));
        let smaller = key((3600, 2025));
        assert_eq!(state.observed_target(smaller, now, context), Some(now + QUIET));
        assert!(state.preview(context, smaller));
    }

    #[test]
    fn blocked_worker_discards_stale_render_and_publishes_only_latest_complete_set() {
        use std::sync::mpsc;
        let worker = Arc::new(PrepareWorker {
            queue: Mutex::new(PrepareQueue::default()),
            ready: Condvar::new(),
        });
        let shared = Arc::new(SoftCache::with_budget(1024));
        let layers = vec![(GROUND, false), (GHOST, true)];
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (stale_tx, stale_rx) = mpsc::channel();
        let (finish_tx, finish_rx) = mpsc::channel();
        let old_state = ResizeState::default();
        let new_state = ResizeState::default();
        let old_owner = old_state.owner;
        let first = key((4, 4));
        assert!(worker.submit(old_state.owner, first, layers.clone()));
        let background = Arc::clone(&worker);
        let destination = Arc::clone(&shared);
        let thread = std::thread::spawn(move || {
            let active = background.queue.lock().unwrap().take().unwrap();
            assert!(background.execute_job(&active, &destination, |job| {
                started_tx.send(job.id).unwrap();
                release_rx.recv_timeout(Duration::from_secs(2)).unwrap();
                prepared_job_cache(job)
            }));
            stale_tx.send(()).unwrap();
            finish_rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let latest = background.queue.lock().unwrap().take().unwrap();
            assert!(background.execute_job(&latest, &destination, prepared_job_cache));
            latest.id
        });
        assert!(started_rx.recv_timeout(Duration::from_secs(2)).unwrap().key == first);
        let middle = key((5, 4));
        let final_key = key((6, 4));
        assert!(worker.submit(old_state.owner, middle, layers.clone()));
        assert!(worker.submit(new_state.owner, final_key, layers.clone()));
        worker.invalidate_owner(old_state.owner); // Same owner-scoped operation as Drop.
        drop(old_state);
        assert!(!worker.waiting_for(old_owner, middle));
        assert!(worker.waiting_for(new_state.owner, final_key));
        release_tx.send(()).unwrap();
        stale_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(shared.state.lock().unwrap().entries.is_empty(),
            "stale job must not publish even a partial prefix");
        finish_tx.send(()).unwrap();
        let next = thread.join().unwrap();
        assert_eq!(next.owner, new_state.owner);
        assert!(next.key == final_key);
        assert!(shared.exact_snapshot(&layers, &context().palette, (6, 4), 1.0).is_some());
        assert!(shared.exact_snapshot(&layers, &context().palette, (4, 4), 1.0).is_none());
        assert!(shared.exact_snapshot(&layers, &context().palette, (5, 4), 1.0).is_none());
        assert_eq!(shared.state.lock().unwrap().entries.len(), layers.len());
    }

    #[test]
    fn same_dimensions_do_not_reuse_changed_scale_or_palette() {
        let worker = PrepareWorker {
            queue: Mutex::new(PrepareQueue::default()),
            ready: Condvar::new(),
        };
        let shared = SoftCache::with_budget(1024);
        let layers = vec![(GROUND, false), (GHOST, true)];
        let old = key((4, 4));
        assert!(worker.submit(7, old, layers.clone()));
        let running = worker.queue.lock().unwrap().take().unwrap();
        let changed_scale = PrepareKey::new(context(), (4, 4), 1.000001);
        assert!(worker.submit(7, changed_scale, layers.clone()));
        let mut changed_context = context();
        changed_context.palette.fg = iced::Color::BLACK;
        let changed_palette = PrepareKey::new(changed_context, (4, 4), 1.000001);
        assert!(worker.submit(7, changed_palette, layers.clone()));
        assert!(!worker.waiting_for(7, old));
        assert!(!worker.waiting_for(7, changed_scale));
        assert!(worker.waiting_for(7, changed_palette));
        assert!(worker.execute_job(&running, &shared, prepared_job_cache));
        assert!(shared.state.lock().unwrap().entries.is_empty());
        let latest = worker.queue.lock().unwrap().take().unwrap();
        assert!(worker.execute_job(&latest, &shared, prepared_job_cache));
        assert!(shared.exact_snapshot(&layers, &changed_context.palette, (4, 4), 1.000001).is_some());
        assert!(shared.exact_snapshot(&layers, &context().palette, (4, 4), 1.0).is_none());
        assert!(shared.exact_snapshot(&layers, &context().palette, (4, 4), 1.000001).is_none());
    }

    #[test]
    fn backdrop_update_schedules_worker_and_redraw_after_quiet_expiry() {
        let style = Era::Neomil.style().dashboard_style();
        let backdrop = ResizeBackdrop::new(Backdrop {
            style, prims: style.dashboard, stretch: false, at: Duration::from_secs(3),
        });
        let worker = PrepareWorker {
            queue: Mutex::new(PrepareQueue::default()), ready: Condvar::new(),
        };
        let soft = SoftCache::with_budget(SOFT_CACHE_BYTES);
        let layers = soft_layers(style.dashboard);
        let source = layer_keys(&layers, &style.palette, (3840, 2160), 2.4);
        for key in source { put(&soft, key); }
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(3600.0, 2025.0));
        let context = Context::of(&backdrop.backdrop);
        let target = PrepareKey::new(context, (3600, 2025), 2.25);
        let mut state = ResizeState::default();
        let now = Instant::now();
        state.context = Some(context);
        state.target = Some(target);
        state.deadline = Some(now - Duration::from_millis(1));
        let event = iced::Event::Window(window::Event::RedrawRequested(now));
        let mut fresh = ResizeState::default();
        assert!(backdrop.update_with::<()>(&mut fresh, &event, bounds, &soft,
            WorkerSource::Borrowed(&worker)).is_none());
        assert!(!worker.waiting_for(fresh.owner, target));
        fresh.clear(); // A route switch, rescale or unfocus also loses context.
        assert!(backdrop.update_with::<()>(&mut fresh, &event, bounds, &soft,
            WorkerSource::Borrowed(&worker)).is_none());
        assert!(!worker.waiting_for(fresh.owner, target));
        let action = backdrop.update_with::<()>(&mut state, &event, bounds, &soft,
            WorkerSource::Borrowed(&worker));
        assert!(action.is_some(), "worker completion needs a future redraw");
        assert!(state.deadline.is_none());
        assert!(worker.waiting_for(state.owner, target));
        let (_, redraw, _) = action.unwrap().into_inner();
        assert!(matches!(redraw, window::RedrawRequest::At(_)));
    }

    #[test]
    fn failed_worker_stops_polling_and_refuses_further_jobs() {
        let mut queue = PrepareQueue::default();
        let target = key((3600, 2025));
        assert!(queue.submit(1, target, vec![(GROUND, false)]));
        assert!(queue.waiting_for(1, target));
        queue.fail(); // Spawn/render/publish failure takes the exact fallback.
        assert!(!queue.waiting_for(1, target));
        assert!(!queue.submit(1, target, vec![(GROUND, false)]));
    }

    #[test]
    fn published_then_evicted_target_uses_fallback_instead_of_endless_requeue() {
        let mut queue = PrepareQueue::default();
        let target = key((3600, 2025));
        assert!(queue.submit(1, target, vec![(GROUND, false)]));
        let job = queue.take().unwrap();
        queue.published = Some(job.id);
        queue.finish(job.id);
        assert!(!queue.waiting_for(1, target));
        assert!(!queue.submit(1, target, vec![(GROUND, false)]));
    }

    #[test]
    fn panicking_worker_transitions_to_exact_fallback_without_polling() {
        let worker = PrepareWorker {
            queue: Mutex::new(PrepareQueue::default()),
            ready: Condvar::new(),
        };
        let target = key((3600, 2025));
        assert!(worker.submit(1, target, vec![(GROUND, false)]));
        let job = worker.queue.lock().unwrap().take().unwrap();
        let shared = SoftCache::with_budget(1024);
        assert!(!worker.execute_job(&job, &shared, |_| panic!("scratch renderer failure")));
        assert!(!worker.waiting_for(1, target));
        assert!(!worker.submit(1, target, vec![(GROUND, false)]));
    }

    #[test]
    fn atomic_publish_upserts_duplicates_and_rejects_an_unretainable_batch() {
        let palette = context().palette;
        let layers = [(GROUND, false), (GROUND, false)];
        let keys = layer_keys(&layers, &palette, (4, 4), 1.0);
        let scratch = SoftCache::with_budget(64);
        scratch.keep(keys[0].clone(), vec![soft::Band {
            y: 0, h: 4, rgba: vec![255; 4 * 4 * 4],
        }]);
        let cache = SoftCache::with_budget(64);
        assert!(cache.publish_complete(&scratch, &keys));
        assert!(cache.publish_complete(&scratch, &keys));
        let entries = cache.state.lock().unwrap();
        assert_eq!(entries.entries.len(), 1);
        assert_eq!(entries.bytes, 64);
        drop(entries);
        let smaller = SoftCache::with_budget(32);
        let old_key = SoftKey::new(GHOST, &palette, (2, 2), 1.0, None);
        put(&smaller, old_key.clone());
        assert!(!smaller.publish_complete(&scratch, &keys));
        let entries = smaller.state.lock().unwrap();
        assert_eq!(entries.entries.len(), 1);
        assert_eq!(entries.entries[0].key, old_key);
        assert_eq!(entries.bytes, 8); // `put`'s 2×2 key stores one 2px row.
    }

    #[test]
    fn exact_snapshot_clones_all_ordered_cached_handles_under_one_lock() {
        let cache = SoftCache::with_budget(1024);
        let palette = context().palette;
        let layers = [(GROUND, false), (GHOST, true)];
        let keys = layer_keys(&layers, &palette, (4, 4), 1.0);
        put(&cache, keys[0].clone());
        assert!(cache.exact_snapshot(&layers, &palette, (4, 4), 1.0).is_none());
        put(&cache, keys[1].clone());
        let snapshot = cache.exact_snapshot(&layers, &palette, (4, 4), 1.0).unwrap();
        assert_eq!(snapshot.size, (4, 4));
        assert_eq!(snapshot.layers.len(), layers.len());
        assert!(Arc::ptr_eq(&snapshot.layers[0], &cache.find(&keys[0]).unwrap()));
        assert!(Arc::ptr_eq(&snapshot.layers[1], &cache.find(&keys[1]).unwrap()));
    }

    #[test]
    fn latest_resize_deadline_controls_the_exact_final_draw() {
        let start = Instant::now();
        let mut state = ResizeState::default();
        let context = context();
        let key = PrepareKey::new(context, (3840, 2160), 2.4);
        state.target = Some(key);
        assert!(!state.preview(context, key));
        assert_eq!(state.redraw(start, context), None);
        let first = state.resized(start, context);
        assert_eq!(first, start + QUIET);
        assert!(state.preview(context, key));
        assert_eq!(state.redraw(start + Duration::from_millis(90), context), Some(first));
        let last = state.resized(start + Duration::from_millis(100), context);
        assert_eq!(state.redraw(first, context), Some(last));
        assert!(state.preview(context, key));
        assert_eq!(state.redraw(last, context), None);
        assert!(!state.preview(context, key));
        assert_eq!(state.redraw(last + QUIET, context), None);
    }

    #[test]
    fn a_changed_context_cannot_continue_a_resize_preview() {
        let start = Instant::now();
        let mut state = ResizeState::default();
        let old = context();
        let key = PrepareKey::new(old, (3840, 2160), 2.4);
        state.target = Some(key);
        state.resized(start, old);
        let mut changed = old;
        changed.palette.fg = iced::Color::BLACK;
        assert!(!state.preview(changed, key));
        assert_eq!(state.redraw(start, changed), None);
        assert!(state.deadline.is_none());
    }

    fn put(cache: &SoftCache, key: SoftKey) {
        let (w, _) = key.size;
        let band = soft::Band { y: 0, h: 1, rgba: vec![255; w as usize * 4] };
        cache.keep(key, vec![band]);
    }

    #[test]
    fn preview_requires_a_complete_same_palette_scale_and_prefix_set() {
        let cache = SoftCache::with_budget(16 * 1024);
        let palette = context().palette;
        let size = (400, 225);
        let target = (420, 236);
        let layers = [(GROUND, false), (GHOST, true)];
        let keys = layer_keys(&layers, &palette, size, 0.25);
        put(&cache, layer_keys(&layers, &palette, size, 0.251)[1].clone());
        put(&cache, keys[0].clone());
        assert!(cache.preview_snapshot(&layers, &palette, target).is_none(),
            "a layer with a different source scale cannot complete the set");
        put(&cache, keys[1].clone());
        let snapshot = cache.preview_snapshot(&layers, &palette, target).expect("complete set");
        assert_eq!(snapshot.size, size);
        assert_eq!(snapshot.layers.len(), 2);
        assert_eq!(Arc::strong_count(&snapshot.layers[0]), 2,
            "only the cache and this draw-local snapshot own the bands");
        drop(snapshot);
        let entries = cache.state.lock().unwrap_or_else(|e| e.into_inner());
        assert!(entries.entries.iter().all(|entry| Arc::strong_count(&entry.bands) == 1));
        drop(entries);
        let mut another = palette;
        another.fg = iced::Color::BLACK;
        assert!(cache.preview_snapshot(&layers, &another, target).is_none());
        assert!(cache.preview_snapshot(&[(GROUND, false), (GHOST, false)], &palette, target).is_none(),
            "a cut is not an independent image");
        assert!(cache.preview_snapshot(&layers, &palette, (500, 281)).is_none(),
            "preview scaling is bounded");
        assert!(cache.preview_snapshot(&layers, &palette, (399, 260)).is_none(),
            "both source axes must stay within the scaling bound");
        assert!(cache.state.lock().unwrap().bytes <= cache.budget);
    }
}
