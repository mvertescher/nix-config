| `<clipPath>` with an animated `<rect>` | `Prim::Motion` + `Change::Clip` on the scene    |
| `begin="1.2s"`                        | `.delay(..)` -- the value holds at `from` until then |# Design pipeline: original image → SVG → iced

The crate's render chain is a three-stage pipeline, each stage the
verifiable input to the next:

```
original image       SVG reference          Rust iced implementation
   images/             docs/<era>/*.svg         src/
   (gitignored)        (committed)              screens/, widgets/, eras/, bar.rs
        │                    │                        │
        └─── G1 ─────────────┘                        │
             trace is                      ┌───────────┘
             faithful to the               │
             material                      └─── G2 ────
                    schematic and its implementation agree
```

## Stages

1. **Original image** — the Behance source (gallery `Cyberpunk 2077 User
   Interface (Part 1)`, `scripts/download_images.py`; the per-era runs are
   documented in `docs/<era>/README.md`). Lives in `images/`, which is
   gitignored: the artwork is not ours to redistribute. This is the source
   of truth for *shapes* and *palette*; nothing downstream may invent
   geometry the material does not show.
2. **SVG reference** — a hand-traced schematic that commits the sampled
   palette and the source's geometry to the repo. One trace per sourced
   screen — `login-trace.svg`, `dashboard-trace.svg`, `mailbox-trace.svg`,
   `store-trace.svg`, sixteen in all — plus `bar.svg`, the one original.
   The app-shaped `dashboard.svg` and `target-app.svg` composites that
   predated the traces were deleted 2026-09-03 (`docs/sources.md` keeps a
   row per deleted file saying what was wrong with it). Each era also has
   a `components.svg` widget sheet, rebuilt the same day from its four
   traces and `bar.svg` with every component cited back to a trace
   element, replacing the by-eye `target-components.svg`.
   This is what a render is compared against, by eye or by script.
3. **iced implementation** — `src/`. Screens, widgets and era tables
   compose from `Style`; the golden matrix renders them headless and locks
   the renders.

The bar is the one surface with no source: no Behance screen shows a
status bar, so `bar.svg` and `bar.rs` are original compositions wearing
each era's palette. `docs/sources.md` records which SVG traces which
source, and which are originals.

## Verification gates

- **G1i — source → svg, measured.** *The gate that matters, and the only
  one here with a pass/fail.* `scripts/extract_spec.py` measures an image
  into a spec — palette by k-means, connected components split by
  nearest-peak on the distance transform, each fitted against rect /
  diamond / chamfered-rect / rule templates, plus a per-ink-family 80x45
  occupancy grid — and `scripts/spec_diff.py` compares two specs. Run it
  as `scripts/fidelity_check.sh --inventory <era> <screen>`. Two verdict
  modes, picked per era in that script:
    - **shapes** (neomil, entropism): match the shape inventories piece
      by piece. Fails when the candidate draws none of a class the
      source has, or matches under 60% of the source's shape area.
      Right for axis-aligned design languages, where the templates fit
      whole widgets.
    - **inks** (kitsch, neokitsch): rotated, overlapping, translucent
      geometry — fan blades, onion cascades — fragments differently on
      a photo and on a clean render, so fragment identity is not a
      stable invariant there; where each colour sits on the canvas is.
      Families are paired by colour and their occupancy grids compared
      by IoU; fails under a weighted IoU of 0.45 or when a major source
      family has no counterpart. Calibrated: faithful traces score
      ~0.5, the known-wrong app composites (since deleted) 0.03–0.07.

- **G1 — source → svg.** Grid statistics:
  `scripts/compare_ref.py images/<src>.png <svg render> --regions`.
  Directional only, and *not sufficient*: a trace that drew three chart
  cards for a photo holding a six-diamond menu scored 0.560 layout here,
  against a "unrelated scenes" baseline of ~0.15. Mass in roughly the
  right place is not the same as drawing the right things. Read it
  alongside G1i and the overlays, never on its own.
- **G2i — svg → iced, measured.** *The gate an SVG→iced conversion
  iterates against, and the second one here with a pass/fail.* What G1i
  is to G1: the design SVG and a live headless capture of the matching
  binary, put through `extract_spec.py` and diffed by `spec_diff.py`
  design→implementation. Run it as
  `scripts/fidelity_check.sh --implementation <era> [screen]`, with
  `--bin-dir DIR` to name where the binaries are (default `target/debug`).
  Unlike G1i this uses the **shapes** verdict for every era, kitsch and
  neokitsch included: the `inks` exception there is about the *photo*,
  whose glow fragments rotated geometry differently from a clean render,
  and both sides here are clean renders of our own. Two knobs differ from
  G1i's defaults and the reasoning is in the script: shapes are matched at
  IoU 0.65 rather than 0.30, because two renders have no glow to excuse a
  loose box, and the pairs that have actually converged hold their score
  to 0.90. It also writes `compare_ref.py`'s overlays plus both inputs to
  `/tmp/g2i-<era>-<screen>/`, because "unmatched in source" names a
  bounding box and only the picture says what was in it.
  One thing G2i hides: a trace records how the material *photographs*,
  and part of that is residue the implementation is told not to draw —
  entropism's sharpening ring around every edge, neokitsch's blurred
  copy of its own content. The extractor bins such residue as an ink
  family of its own, and on the mailbox it was 48% (entropism) and 77%
  (neokitsch) of the design's shape area, so no faithful screen could
  clear the 60% bar. A trace marks those elements `class="photo"`; G2i
  renders the design with that class hidden and G1i renders it whole.
  The tag is a claim about the photo, so adding it is vision work like
  any other trace edit, and the header should say what was tagged.
  Two more things about the extractor worth knowing before reading a
  G2i number. Its colour budget is k=8 clusters and a haze or bloom
  takes about five of them, so on a hazed screen every ink family is
  competing for three bins: line art drawn dimmer than measured merges
  into a neighbouring family (the neokitsch mailbox wire scored as
  tape ink, RIFLES as foreground), and a screen drawn without its
  ground scores against a design whose ground *is* five of the eight
  families — backgrounds are not optional. And iced strokes cover
  ~15% more than rsvg's at the same width (.87 vs .75 coverage per
  edge pixel), which is enough to flip a hairline's bin; the pairs are
  a match by eye when that is all the diff says. Third, the two sides
  do not composite alike: rsvg applies `fill-opacity` to the encoded
  sRGB values and wgpu blends in linear light, and no single alpha
  reproduces a translucent layer over a *varying* backdrop in all three
  channels. On the kitsch dashboard's ghost stacks the gate flipped
  between four and six levels of drift (paste experiments: correcting
  every pixel off by more than 8 levels scored 49%, more than 6 levels
  67%). The scene resolves this on the app side, not the gate's: an
  era table wraps a translucent stack in `Prim::Soft` and
  `screens/soft.rs` composites it in sRGB, in software, so the capture
  carries the trace's own pixels (that screen: 45% → 68%). A new
  translucent stack that scores low is almost certainly missing its
  `Soft`, and a gate-side "linear-light design" mode would not help —
  rsvg has no such mode, and the drift is per channel. The same
  compositor is where the traces' gradient defs the canvas has no
  shape for go: a multi-stop `linearGradient` is `Prim::Ramp`, a
  luminance `mask` is `Prim::Masked` (rsvg's arithmetic, on the
  encoded values), both inside a `Soft` group. A ground that
  `triptych.sh --diff` lights edge to edge is one of these drawn from
  memory instead.
- **G2 — svg → iced.** The implementation matches the reference:
  rasterise the SVG (`rsvg-convert`) and
  `scripts/compare_ref.py <svg render> tests/golden/<screen>-<era>-...png`.
  The golden matrix already locks iced-vs-iced (G0: byte-identical run to
  run for one build — a toolkit bump can still move it, as iced 0.14 did
  by a few edge pixels on every screen); this
  gate locks iced-vs-reference. A schematic is expected to reach
  ~0.65+ layout / 0.85+ palette against its own golden; text rasterization
  and the hand-drawn gap keep edge correlation below the perfect-1.0 that
  G0 holds. G2 has no verdict and moves smoothly; where G2i can run,
  it is the one to iterate against and G2 is the sanity read beside it.
- Run any of them with `scripts/fidelity_check.sh [--source|--inventory|--implementation] [era [screen]]`.

## Rendering one screen by hand

`scripts/render.sh` is the golden matrix's recipe — headless weston,
pixman, a lavapipe ICD, `WGPU_BACKEND=vulkan`, the era's palette
published into a scratch `HOME` — pointed at a binary already on disk
instead of at a nix build of the crate:

```
scripts/render.sh --era neomil --size 1600x220 --out /tmp/bar.png cp-eras-ui-bar-window
scripts/render.sh --era kitsch --out /tmp/login.png cp-eras-ui-login   # 1600x900 default
scripts/render.sh --era none  --bin /path/to/cp-eras-ui-login ...      # compiled fallback
```

It is what G2i captures the implementation with, and it takes about
eight seconds against a warm nix store: a 4s settle, with
`ICED_PRESENT_MODE=mailbox` so the app's second frame is not held back
by the headless compositor's frame callback — under the default FIFO
presentation that frame took 3-5s to land and a 3s capture missed the
login washes, which was misread for a day as a first-paint cost.
`render.sh`'s `DEFAULT_SETTLE` note has the measurements. It does not
build anything: use `nix-shell shell.nix --run 'cargo build --bin
<name>'` first, or pass `--bin`. The capture is faithful — 19 of the 20
golden cells come out byte-identical to `tests/golden/` — but it is not
hermetic (the 20th, store/neomil, draws kanji the sandbox has no font
for), so `scripts/run_test_matrix.sh` remains what gates a change.

`scripts/triptych.sh [era [screen]]` stacks the three stages of a screen
into one image — source photo, trace render, `render.sh` capture, top to
bottom, captioned — for every era x sourced screen by default, into
`images/triptych/`. It is the review view, not a gate: the gates say how
far apart two stages are, this shows *where*, and it is the fastest way
to see whether a trace or an implementation lost something the photo has.
Unlike G2i the trace is rendered with its `class="photo"` elements, since
it is being read against the photo above it. `--diff` adds a fourth row
that points at what rows 2 and 3 disagree on — |trace − iced| per pixel
on a black → yellow → red ramp over a dimmed copy of the trace, the
trace this time *without* its `photo` elements so the expected halo
stays dark — with the share of pixels off by more than 8 levels in the
caption. Text always lights a little (two rasterisers, two AAs); a
filled shape lit solid is a colour miss, an outline lit is a placement
miss, and a whole frame lit dimly is a ground drawn from memory.

## Motion

The traces are still pictures of a running interface, and the material
is stiller than they are: a photo cannot say how the caret blinks or
how a panel opens. Motion is the one layer of the design with no source
to be faithful to, so the convention is built around making it cost
the still frame nothing — the trace at rest stays the design the gates
measure against the photo, and everything that moves is written *on
top* of it.

**Where it lives.** SMIL, in the trace, on the element it moves: an
`<animate>`, `<animateTransform>` or `<set>` as a child of the `<rect>`
or `<g>` whose attribute changes, never a CSS animation and never a
script. SMIL because the timeline can be seeked from outside (below);
CSS animations cannot be, and rsvg draws neither. Each `<animate>`
carries an `id` — `caret-blink`, `panel-open` — which is how the iced
side and the review tools cite it, exactly as an element's `id` cites
its geometry.

**The rest frame is the trace.** rsvg ignores SMIL, so every static
gate, the goldens and `triptych.sh` see the trace with nothing played,
and see no difference whether it is annotated or not. The rule that
keeps that true: the element's own attribute must equal the value its
animation has *at rest* — where it freezes (`fill="freeze"`, its `to`
or the last of its `values`) for a transition, and its frame-0 value
for a cycle. Rest is `motion::REST`, 2.4 s: after every boot-in the
traces annotate and a whole number of caret blinks, so the caret is
lit there as at 0. A boot-in is therefore written *backwards* from the
drawing: the element as traced is the `to`, and the `from` is where it
comes in from. An annotation that moves the rest frame has changed the
design, and the G1 gates will say so.

**Vocabulary.** Only what the iced side can play back --
`iced::animation::Animation` (lilt 0.8) for the eased transitions,
`src/motion.rs` for the discrete cycles it has no notion of:

| SMIL                                  | iced `Animation`                              |
|---------------------------------------|-----------------------------------------------|
| `dur="400ms"`                         | `.duration(Duration::from_millis(400))`       |
| `begin="1.2s"`                        | `.delay(..)`, or a later `.go(.., at)`        |
| `repeatCount="indefinite"` / `"3"`    | `.repeat_forever()` / `.repeat(3)`            |
| `values="a;b;a"` with `keyTimes`      | `.auto_reverse()` (only the symmetric case)   |
| `calcMode="discrete"`                 | a phase of the clock: `motion::blink` and kin  |
| `calcMode="spline"` + `keySplines`    | `Easing::EaseInOutCubic` and family           |
| `fill="freeze"`                       | the default: the state stays where it went    |

`keySplines` are cubic beziers; lilt's named easings are the
easings.net curves (`EaseInOut` is the sine, `EaseInOutCubic` the
cubic polynomial, and so on), which are not beziers, but easings.net
publishes a bezier beside each that tracks it closely enough at trace
scale: `0.37 0 0.63 1` for `EaseInOut`, `0.65 0 0.35 1` for
`EaseInOutCubic`, `0.45 0 0.55 1` for `EaseInOutQuad`, `0.33 1 0.68 1`
for `EaseOutCubic`, `0.61 1 0.88 1` for `EaseOut`. Write one of those,
so the transcription is a lookup and not a curve fit (`Easing::Custom`
exists for the day a trace needs a curve nothing named tracks).
Anything else — `keyTimes` with more than three stops, additive
animations, `<animateMotion>` along a path — is not in the vocabulary
until the iced side has a way to play it, and a trace that needs it
says so in its README instead of annotating.

**Two kinds of begin.** `begin="0s"` and friends are the document clock,
which is what a frame at time t is a frame *of*. An interaction —
hover, focus, a press — is `begin="<id>.click"` or `.mouseover`, and a
review tool cannot fire those; so a state that is reached by input
gives the transition both: `begin="click; 0.6s"`, the clock-based
start there so that `frame.sh --at 0.8` can show the state
mid-transition.

**Boot-ins.** A transition from the document's clock — `begin="0s"`,
or a hold and then a start — is how a screen comes up: neomil's GO
HOME panel wipes in under `#panel-open`, a `<clipPath>` whose `<rect>`
grows from no height to the panel's over 0.36 s, eased out. On the
iced side that is data on the era table, not code on the screen: the
group is wrapped in a `Prim::Motion` carrying the `<animate>`'s
`begin`, `dur`, easing and what it changes (`Change::Clip`), and
`scene.rs` paints it through `Frame::with_clip` at the width and height
`motion::progress` gives for the scene's moment. A screen with a
boot-in ticks every frame until `motion::REST` and then stops asking
(`screens::dashboard::subscription`); a screen with nothing moving
hands the scene its clock anyway. The two states themselves — rest and hovered, closed
and open — are drawn as sibling groups in `components.svg` next to the
element they belong to, cited back to the trace like everything else
on that sheet, so the design of the *destination* is reviewable without
seeking.

**Seeing a frame.** `scripts/frame.sh --at <seconds> <trace> out.png`
renders the trace at a moment — headless Firefox with the timeline
paused and seeked from a script appended to a scratch copy; the header
explains why Firefox and why the script has to be inside the SVG. Its
frame 0 matches rsvg's on all but 0.016% of the neomil login's pixels
(8-level fuzz), so a frame from it is comparable with one from the
static pipeline. Pass `--no-photo` for a frame to set beside an
implementation capture, as G2i does. The implementation's side is
`src/motion.rs`: every screen reads its time from `motion::now()`,
counted from one origin, and `--at-ms <n>` (or `CP_ERAS_UI_AT_MS=<n>`
in the environment, which is how the harnesses say it) pins that
clock, so `scripts/render.sh --at <seconds>` captures the same moment
`frame.sh` renders. `triptych.sh --at <seconds> --diff` stacks the
two with the photo and the heat row, which is the review view for an
animation: run it at a few moments across the cycle. With no `--at`
all three scripts take `motion::REST` (2.4 s), and the goldens are
that frame — `tests/visual.nix` exports `CP_ERAS_UI_AT_MS=2400` —
which is why a trace annotation and its transcription leave every
golden untouched: the static design *is* the rest frame, on both
sides.

**Who annotates.** Annotating is trace work and falls under the same
rule as the rest of `docs/`: a vision model, judging how the motion
reads, and never a coding model reaching into a trace to make its
implementation's timing match. What the coding model gets is the
annotation as text — attribute, values, timing, easing — and the table
above to transcribe it with; the transcription cites the `<animate>`'s
`id` in a comment beside the `Animation`.

Two animations are carried end to end this way. The login caret's
blink is the cycle, the worked example to read when the description
above is not enough: `#caret-blink` in three of the four `login-trace.svg`s
(neokitsch's field draws no caret in the photo, so it has none) —
`values="1;0" keyTimes="0;0.5" calcMode="discrete" dur="1.2s"
repeatCount="indefinite"` on the caret, which is a `<rect>` in kitsch,
an underline `<path>` in entropism and the `__` at the end of neomil's
masked run, split into a `<tspan>` so it could be animated alone.
`motion::CARET_BLINK` is the period, `motion::blink` the phase,
`style::Blink` says per era which of those three things the frame
turns off, and `screens::login` ticks once per half-period unless the
clock is frozen. `triptych.sh --at 0.3 neomil login` and `--at 0.9`
are the two halves. The neomil dashboard's `#panel-open` is the
transition: the `<clipPath>` in the trace's `<defs>`, the group's
`clip-path`, the `Prim::Motion` around `GO_HOME` in `src/eras/neomil.rs`,
and `triptych.sh --at 0.15 neomil dashboard` for the panel four fifths
of the way in.

## Iteration loop

```
rsvg-convert docs/<era>/<screen>.svg (or nix shell nixpkgs#librsvg ...)
compare_ref.py <source-or-golden> <svg render> --out /tmp/fid
inspect side-by-side.png / checker.png / edges.png / heatmap.png
edit the SVG  →  repeat until the geometry reads right
```

The SVG is the target; the app follows. Never fix the SVG by editing the
app's render, and never touch the goldens except when the iced
implementation genuinely changes (the procedure in `tests/bar.nix`).

## Direction of change

Any mismatch found by these gates is owned by the earlier stage: if G2
fails, the SVG or the implementation is wrong (fix the earlier one); if
G1/G1i fails, the trace is wrong (fix the SVG). The material always wins.

## Division of labour: who may touch which stage

**Only a model with strong vision may write or modify the SVGs in
`docs/`.** Every reference trace in this repo that was written without
opening the source image turned out to be a fabrication, and a wrong
SVG poisons everything downstream — the SVG is the *target* the iced
implementation is built and judged against. Editing one means reading
the source photo, cropping into regions, and judging renders by eye;
the gates check a reading, they cannot substitute for one.

**SVG → iced (stage 2 → 3) is coding work and does not need vision.**
The SVG is text carrying measured coordinates and sampled colours, and
G2/G0 give scripted, numeric feedback (`fidelity_check.sh`, the golden
matrix). A coding model converting `dashboard-trace.svg` into a
`screens::` arm should treat the SVG as the spec: take geometry and
palette from it verbatim, iterate against the gates, and **never edit
the SVG to make its own render match** — any mismatch it wants to fix
by touching `docs/` is a signal to stop and report instead
(see Direction of change above).

## Read the image

Stage 1 is a picture, and the only way to know what it holds is to look
at it. Two of the two traces ever checked against their material turned
out to have been written without that — both scored respectably on G1,
both drew things that are not in the photo, and both had their invented
descriptions copied into `docs/sources.md` as if observed, where
everything downstream then trusted them. `Layout::OpsCharts` exists
because of one of them.

So: before writing or trusting a trace, open the source. If the tool in
hand cannot display an image, that is a reason to stop and say so, not a
reason to infer the contents from a downsampled colour grid. `compare_ref.py`
and `extract_spec.py` are for *checking* a reading of the material; neither
is a substitute for having read it. `extract_spec.py --crops DIR` writes a
zoom per detected shape, which is the cheapest way to inspect a source
region by region.