# Font and text-effect audit

2026-10-02. This audit checks the rendering foundations before further
individual glyph fitting. It covers the current implementation and earlier
font, contour and echo corrections. It does not establish which typeface
was used in a photographed reference.

## Font selection

All 24 reference SVGs resolve their effective text family and weight,
including inherited styles and expanded local `use` references. Numeric
CSS weights select the expected bundled Rajdhani and Orbitron files in
the pinned reference Fontconfig environment. Kitsch `SC` explicitly uses
Rajdhani 600. FreeSans Bold declares OS/2 weight **600** despite its name;
the native selector correctly requests 600. Librsvg's SVG 600/700
FreeSans specimens are pixel-identical, so the SVG request of 700 is not
evidence of synthetic extra bolding in this stack.

The application loads twelve faces: five Rajdhani weights, four Orbitron
weights, FreeSans Bold, the named Kitsch derivative, and the Noto subset.
Orbitron 800/900 files exist but are not loaded or requested by current
artwork. The packaged and development font inputs agree. Installed Latin
duplicates have identical bytes; a different fontdb ID alone is therefore
not a fallback failure.

An actual Iced paragraph probe checked seven scene face selectors against
five representative Latin strings under Basic and Advanced shaping in
both host and isolated font environments: **140 Latin cases**, all using
the expected bundled bytes. A permanent regression additionally covers
all eleven loaded Latin faces, including Light and intermediate Orbitron
weights, under Basic, Advanced and the canvas default. It checks actual
glyph bytes, declared and requested weights, and missing-glyph IDs.
The three font tests pass in both environments.

The Noto subset is not a general fallback guarantee. The current MASURAO
logotype is vector art. In the probe, Basic shaping of its three kanji
through a Latin selector produces three missing glyphs; Advanced selects
host Unifont JP for Rajdhani Regular/Medium but the bundled Noto subset in
the isolated environment. Other tested weights select the subset in both.
A future CJK text caller needs an explicit supported family/weight and
an isolated coverage check. The obsolete claim that current artwork
relies on automatic Han fallback has been corrected in `fonts.rs`.

The older Orbitron compatibility branch in `default.nix` renames Light to
Regular and Bold to SemiBold when passed the old nixpkgs package. The
current overlay and shell use `orbitron-vf`, so this branch is not active.
Such an override cannot inherit this audit's font or fidelity guarantee.

## Shaping and earlier corrections

The pinned canvas default is **Auto**, which chooses Basic for these ASCII
strings. Scene/login width measurement explicitly chooses Advanced.
Tracked text measures successive prefix widths and paints individual
characters; `TrackedWords` also measures each word again. Prefix widths
are not a general way to recover positioned glyph origins or kerning.

The actual Iced probe compared fifteen cases at 1600-design and 4K scale,
including mailbox punctuation/actions/badges, Neo-kitsch module labels
and spaced metadata, Kitsch compliance/title text, and login legends.
Host and isolated results agree. The sampled production Rajdhani and
FreeSans cases keep the same glyph and font IDs; Basic/Advanced origins
differ by less than 0.0001 output pixel, and cumulative split placement
differs by at most 0.000153 pixel after matching tracking and anchors.
This does not explain their visible source-contour residuals.

The limitation is observable with the Orbitron `AV VA To` control:
Basic/Auto width is 275.568 pixels at 4K scale, versus 272.064 Advanced;
prefix-based split placement differs by up to 2.592 pixels. Current era
scene tables do not instantiate Orbitron text, so this is a limitation to
resolve before extending those primitives to kerning-sensitive callers,
not evidence to move the calibrated current labels. Shaping policy has
not been changed globally.

Scene coordinates include route-specific baseline calibrations, including
Entropism Mail's measured 0.89 and FreeSans raster baseline rounding.
Nonuniformly stretched canvas text goes through glyph outlines; ordinary
text uses the cached raster route. Matching outer bounds does not make
those routes interchangeable.

The general Rajdhani 0.84 conversion was retained, not established as an
exact SVG baseline by the font-selection/advance checks above. JN now
measures **0.892 × size** in the pinned Iced paragraph and actual canvas
outlines, with the resolved face bytes equal to bundled Rajdhani Regular.
Auto, Basic and Advanced agree at sizes 7.5 and 18; the baseline control
glyph H has font yMin=0. This also matches the centered-line formula:
`(1.2 − (0.930 + 0.346))/2 + 0.930 = 0.892`.
Subtracting 0.84 leaves +0.052 × size in local y, which a clockwise
quarter-turn maps to negative world x: **−0.936 pixel at 4K** for ordinary
PETROCHEM. This explains a coordinate contribution to JI/JK/JM's failed
native transfer, independently of antialiasing or printing weight. JO's
local +0.39-design-x compensation passes three-size source review, thirteen
state checks and actual-package parity in [round seventy-seven](reference-svg-round77.md).
JR adds a permanent CPU regression for the measured outline baseline at
both sizes and all three shaping modes. The full suite passes 311 packaged
tests, 22 repository checks and 28 matrix cases. Shared coordinates and
cached child outlines are preserved; the test adds no runtime font work.
Existing source-calibrated artwork prevents a blanket change. See the
[PETROCHEM evidence](neomil/store-printing.md#ordinary-petrochem-aperture).

The related routes use different line-box conventions:

| Route | Line height / subtracted size fraction | Evidence |
| --- | --- | --- |
| Scene Rajdhani | 1.2 / 0.84 | Retained source calibration; Regular outline baseline directly measured at 0.892. |
| Login Rajdhani | 1.276 / 0.93 | Explicit natural-height line and ascender subtraction; consistent with the font metrics, not this exact route's raster probe. |
| Mail Rajdhani | 1.2 / era factor | Entropism 0.89, Neomil 0.84, Kitsch/Neo-kitsch 0.95 are source/layout calibrations. |
| Scene Orbitron / FreeSans | 1.2 / 0.984 or 0.95 | Metrics-derived constants; JN did not directly probe these outline routes. |

All Rajdhani weights share the ascent/descent headers, which predicts the
same physical offset but does not establish every caller's final raster
placement. Existing local changes, including the accepted Neomil Mail body
baseline adjustment, remain supported by their own source/state evidence.
Normal text widgets have no authored SVG-baseline conversion. The read-only
route review is retained in `/tmp/cp-eras-next/js-baseline-coverage/`.

KA/KB separately audit the ordinary Store compliance captions after their
local vertical correction. Each is a complete Regular text run using the
backend's cached-text route; position changes do not add layout-cache keys.
The exact full/truncated strings pass 24 CPU cases and 1,008 actual glyph-byte
checks. Auto/Basic agree exactly; Advanced origins differ below 0.0001 pixel.
Matched source words and the rounder source O still differ substantially.
This establishes correct implementation selection, not the original font's
identity or permission to apply a global stretch. A nonuniform `Wide` port
would add outline work and bypass the current Bold-only reusable-title
optimization. See the [caption evidence](neomil/store-typography.md#ordinary-compliance-lines--jv).

KD–KF extend that distinction to a generic FreeSans Regular comparison.
The exact file resolves correctly in SVG and all 1,008 Iced glyph records;
its measured baseline is 0.95 × size. The unfitted face/cap substitution
still fails source dark-region and counter checks. A two-parameter source
word-position fit is closer but retains up to 4.066 pixels of held-out
edge error at 4K. Neither proves source identity or authorizes a font swap.
Adding tracking would change the current complete-run drawing route and
requires its own shaping and cost review before a native transfer.
KG's complete SVG layout test then improves 44/54 line controls but fails
ten dark subsets and the first O-hole comparison. It is rejected; no
extra text shaping, per-character drawing or package font is introduced.

The named Kitsch derivative was independently checked against all 2,911
FreeSans glyphs: only `M` differs, including its hint program. All advance
metrics, cmap, GPOS and GSUB are identical. Existing exact original-outline
replay evidence remains applicable. Entropism badge paths retain literal
guards, complete-run layout, semantic ink and fallback. The new punctuation
trial is still rejected: splitting an SVG run changed 55 pixels outside
the intended region at 4K and lost bright detail at 1600. Future local
substitutions must first pass an exact no-op replay.

## Effect measurements and verification

The serialized sweep uses the real Iced renderer and current canvas programs
at fixed bounds. It covers Neomil Dashboard, Store, Mailbox and Login at
1600 and 4K in rest, opening, hover and held states, plus Kitsch/Neo-kitsch
rest at 1600. Each of the four release variants completed 280 draw groups:
40 cases with a first call and six repeated calls. Timer-only and counted
builds are separate. Release dependencies, including patched `iced_wgpu`,
are selected by Cargo fingerprints; this is a scratch release build, not
a packaged application or hardware presentation benchmark.

Baseline release **foreground CPU construction**, warm median in ms:

| Neomil screen | 1600×900 | 3840×2160 |
| --- | ---: | ---: |
| Dashboard Scene | 4.195 | 4.594 |
| Store Scene | 24.681 | 23.155 |
| Mailbox Sheet | 0.252 | 0.306 |
| Login Art | 2.957 | 2.942 |

These are separate screens, not quantities to add into a frame time.
They exclude widget layout, GPU work and presentation. Debug build timings
are much higher and are retained only as diagnostics. First access to a
software-material key is measured separately; the shared 384 MiB LRU is
warm on subsequent identical draws. A case's first draw is not necessarily
a process-cold cache access.
The sweep forces repeated draws; these event-driven examples do not imply
a continuous loop paying that cost while idle.

Neomil's login echo builds one local tile from two text runs (104 path
callbacks) on a cache miss. The next six draws build no tile or paths.
The release warm cache lookup is below the timer's one-microsecond
resolution; its first tile construction is about 3.2/5.3 ms at 1600/4K.
The cache tracks palette, static slot identity and bounds. Tile dimensions
are bounded at 4096×512. Pointer feedback does not enter its key.

Store's repeated Neomil title uses one outline extraction and 60 cache
hits per draw. The cache is deliberately local to the draw and bounded to
four entries, 64 content bytes and 256-pixel font sizes; transforms,
clipping and color are applied on replay. Its recorded ten path callbacks
are **not** all font work: internal `fill_text` outline fallback is outside
those counters. There are still 66 entered clip groups and repeated path
fills. Counts identify work volume, not its CPU cost.

The integrated cleanup skips unused width measurement for Start-anchored
`Wide` and `ReusableWide`. It removes 73 of 385 width calls from Neomil
Store, all 12 from its Dashboard, and 19 of 1,276 from Kitsch Store.
Other measured counters are unchanged. This is **not a measured speedup**:
4K Store foreground medians are 23.1545 ms before and 23.1600 ms after,
with overlapping ranges; other cases vary in either direction. The large
number of prefix measurements in tracked text therefore needs profiling,
not an assumption that eliminating calls will dominate the frame cost.

All eleven before/after native pairs are RGBA-exact: Store at 4K in all
four eras, Neomil at 1537×947, and fractional rest, opening, hover, held,
custom-color and selected-state controls. Both synthetic rest wrappers
also match the actual Store. These controls validate the small cleanup;
they do not establish the photographed typefaces or close live GPU checks.

The counted instrumentation also produces RGBA-exact ordinary Neomil
Dashboard, Store, Mailbox and Login frames at 1600, compared with the
uninstrumented examples. The blank timing driver is not itself pixel proof.

A further release profile separates text spans without changing their
order. Neomil Store at 4K spends median 6.26 ms in tracked-character paint,
3.48 ms in uncached Wide `fill_text`, 1.94 ms replaying cached title paths,
0.47 ms measuring tracked prefixes, and 0.056 ms looking up/extracting
cached outlines; 11.02 ms remains outside these spans. Kitsch Store at
1600 spends 7.08 ms in tracked prefix measurements, 5.64 ms in Wide
`fill_text`, and 12.95 ms outside the spans. These are instrumented ownership
measurements with timer/atomic overhead; residual work includes geometry
finalization and uninstrumented primitives. They support targeted follow-up,
not attributing all Store cost to font lookup or the title echo cache.

### Repeated tracked advances

The measured Kitsch repetition is avoidable: 47 tracked runs use only
eight distinct text/font/size keys. DY reuses the existing advance vectors
within one Scene draw. Keys include the complete `iced::Font` and exact
size bits; position, color, tracking and anchor are applied afterward.
The FIFO holds at most sixteen entries, each limited to 256 content bytes
and a finite positive size no larger than 256 pixels. Its retained float
payload is bounded by 16 KiB, excluding entry/allocation overhead; the
cache is discarded after the draw. Ineligible runs use the original path.
The existing shaper, float order, character painter and outline cache stay
unchanged. Regression cases cover same-weight distinct families, including
FreeSans versus the named Kitsch derivative, as well as exact sizes and
eviction.

Four fresh release sweeps each complete the same 280 groups. Kitsch Store
at 1600 drops from 1,257 prefix measurements to **299**, with 39 cache hits,
eight misses and 299 retained floats. Warm foreground CPU median falls
from **25.3385 to 20.2215 ms (20.2%)**; observed ranges are 25.276–25.978 and
20.189–20.802 ms. Painting, clipping and outline counts are unchanged.

Neomil Store drops from 312 width calls to 126, with thirty hits and
eighteen misses; FIFO eviction leaves sixteen entries and 102 floats.
At 4K rest, the observed foreground median is **22.970 to 22.559 ms**
(1.8%; ranges 22.877–23.328 and 22.428–22.681 ms). Its 1600 rest ranges
overlap. These smaller changes need caution: the unchanged Neomil
Dashboard also shifts by about 0.08–0.10 ms despite zero cache hits.
All Scene medians fall in this sweep; untouched Mail/Login/backdrop paths
show small changes in both directions. The strong Kitsch result and
deterministic call reduction are the firm evidence, not a blanket FPS claim.

First-draw results are mixed. Neomil's 1600 rest Store Scene takes
26.883 to 30.534 ms in this pair, while Kitsch's first Store Scene improves
26.305 to 21.076 ms. The unchanged Neomil Dashboard and initial backdrops
also vary substantially. A case's first draw is not generally process-cold,
and the advance cache resets every draw; only renderer/process/shared-cache
state warms between repetitions. One fixed-order process per variant does
not establish startup benefit or explain the slower first Store sample.

The remaining Neomil tracked-character paint cost needs a separate
rotated-versus-ordinary breakdown before changing the renderer route.
The measured 6.26 ms bucket contains all 48 tracked runs, not just the
nine known rotated runs. General kerning-sensitive split text also needs
an explicit layout policy before adding callers; this cache does not
correct that latent API limitation.

The cache passes eleven additional RGBA-exact native pairs: Kitsch Store
at 1600, 4K and 1537×947; fractional rest, custom palette, held, opening
and fourth-card selection; Neomil and Neo-kitsch Store at 4K; and Kitsch
Dashboard at 1600. Both Kitsch rest wrappers match the actual Store exactly.
The captured candidate's complete Rust source matches the integrated code.

## Future text inspections

- Verify the actual resolved font bytes, weight and glyph coverage, including
  an isolated font environment, before attributing a mismatch to the typeface.
- Compare full-run glyph IDs and origins with split placement at the same
  size, baseline, anchor and tracking; matching total width is insufficient.
- Check actual paragraph/outline baselines separately from advances. The
  Rajdhani regression guards the measured engine offset; source-fitted
  scene coordinates can contain additional calibration.
- Preserve the measured raster or outline route and its baseline calibration.
  Rotation and nonuniform stretch can change that route.
- Require an exact no-op replay through the proposed substitution pipeline,
  with frozen source/crop/registration controls, before editing a glyph.
- For rendering changes, measure first and warm release draws separately,
  use counted runs to explain the cost, and verify relevant scales and states.

## DX/DY integrated verification

All **298 local and packaged Rust tests** pass, as do all 253 library tests
with isolated fonts. The full repository check passes all **22 checks** and
the **27 visual cases** on their first attempts. Independent output comparison
finds 26 RGBA-exact goldens and the same one-pixel, one-channel-level
Neo-kitsch bar residual recorded before this work. No golden was updated.
The 332 tracked files supplied to the test build match the frozen source;
all sixteen original source images retain their earlier hashes.

Verification evidence is in `/tmp/cp-eras-next/dx-validation/`, with
`dy-root-review/tests-summary.json` and the cache-specific comparisons above.
Only the three audit/TODO Markdown files receive completion notes after
that source freeze; executable code, assets and test inputs stay unchanged.
The five changed files remain staged and uncommitted. Source-font identity,
future CJK coverage, general kerning policy and live GPU/presentation behavior
retain the explicit limits described above.

Scratch evidence: `/tmp/cp-eras-next/dx-font-selection/`,
`dx-text-semantics/`, `dx-text-effects/` and `dx-root-review/`. The latter
contains the executed Iced probe logs, numeric summaries and font-test
logs. The follow-up cache candidate, frozen release measurements and
native comparisons are in `dy-advance-cache/`, `dy-advance-perf/`,
`dy-advance-verify/` and `dy-root-review/` under the same scratch root.
The source photos, reference SVGs and accepted font outlines have not
been edited by this audit.


## DZ rotated-text follow-up

The [tracked-text study](neomil/tracked-text-performance.md) separates the
remaining Neomil Store cost: repeated quarter-turn outline extraction
dominates the rotated labels. A bounded per-draw positioned-path cache cuts
109 character extractions to 40 without changing paint order or font/layout
semantics. Release ABBA measurements show 4K rest foreground CPU time
22.63→19.42 ms (14.2%) and 1600 rest 24.35→20.65 ms (15.2%). All 23 cache
native comparisons (including five Kitsch Dashboard pairs) are exact on
Wgpu/tiny-skia and 258 library tests pass in
both font environments. The packaged build passes all 303 Rust tests and
the full check passes all 22 checks and 27 visual cases on their first
attempts. Twenty-six goldens are RGBA-exact and the prior one-pixel,
one-level Neo-kitsch bar residual is unchanged. Fresh packaged 4K Store
and fractional Kitsch Dashboard frames match the candidate exactly.
All 333 tested tracked files match the Nix source and all sixteen originals
are unchanged. Only five Markdown files receive completion/SC-study notes
after this freeze; runtime code, assets and test inputs remain unchanged.
The seven changed files remain staged and uncommitted. Evidence is in
`/tmp/cp-eras-next/dz-validation/`; the earlier DX/DY verification above
remains the record for that checkpoint.


## EA SC photographic-control follow-up

After the font audit, a bounded Kitsch SC study tests whether photographic
blur/registration can explain the lighter source lettering before another
ink or font trial. Six frozen source/runtime/fontconfig/raster hashes and
all four crop arrays are rechecked. Sixty blur/translation models per card
use only blank-disc, square and text-free edge controls; SC pixels do not
select parameters. Separate held-out edge MAE changes are 15.635→15.630,
17.863→16.944, 16.484→16.986 and 19.936→19.604. Every card has both zero
and 0.35-pixel blur among near-best fits. This limited model does not
separate geometry from blur reliably enough to justify changing ink.

The result is a rejected diagnostic, with no artwork change. Its 1600
scores use a whole-image downsample of rounded grayscale modeled SVG,
whereas the source is resized in RGB before luma conversion; they are not
native-renderer or independently rendered 1600 SVG parity evidence. Root
reviewed the source, scores and montage. K8 ink/phase fidelity stays open.
Frozen evidence: `/tmp/cp-eras-next/ea-text-followup/hash-manifest.json`.

## EB complete-run SVG replay follow-up

The Entropism mailbox urgency runs can use uniquely renamed copies of
bundled Regular/Medium Rajdhani without changing any pixels at 4K or
direct 1600. Only `name` and the `head` checksum differ in the fonts; the
complete text nodes retain shaping, positions and the original raster
route. Root independently checks tables, selected file hashes and both
full RGBA frames. Keep baseline/candidate fonts in separate sibling
directories: Fontconfig can recursively discover same-name candidates
when their common parent is listed. Verify the resolved bytes in each
environment before rendering.

This is an SVG replay prerequisite, not a native route or source-font
identification result. A subsequent parentheses-only contour edit still
changes 36 other 4K letter pixels, up to three channel levels, and loses
source shape/bright-core controls. Exact no-op success does not guarantee
locality after an outline edit. Require both checks. The rejected trial
and independent verification are in
[round fifty-five](reference-svg-round55.md#entropism-complete-run-font-replay).

EC narrows that locality failure: an independent Pango probe preserves all
44 placed glyphs, while minimal complete-run SVGs reproduce the exterior
alpha changes. Unchanged-prefix SVGs and a separate unused-glyph edit are
exact at both sizes. This is consistent with a complete-run painting
interaction; it does not identify librsvg's internal stage or establish
a Cairo bug. Keep the current shaping/draw route and both no-op/exterior
checks. No contour replacement or extra per-letter draw is justified.
See [round fifty-six](reference-svg-round56.md#entropism-whole-run-paint-locality).

## EK vector-edge blending check

A controlled actual-Store/Scene vector probe confirms a separate reason
for SVG/native edge differences: the current Iced path blends in linear
light, while the pinned librsvg path blends encoded RGB. Matching
half-covered `#b3ddb1` edges over `#110c07` produce native `(131,162,129)`
and SVG `(98,117,92)`. Four fixed phases on three background colors,
with independent black/white controls and exact solid-ink checks, agree
with those respective blend models within one channel level. Astra
independently reproduces all 24 colored sample predictions.

The compiled graphics feature fingerprint excludes `web-colors`; Cargo.toml
already documents this deliberate choice to preserve glow and text across
the Iced migration. Native filled paths use 4× MSAA. Account for this
pipeline difference before changing a font's weight, outline or opacity
to match a photographed/SVG edge. The probe uses vector rectangles, so it
does not establish identical raster-font coverage or identify a source
font. It also does not explain every fine-detail residual: geometry,
coverage quantization and photographic filtering can differ independently.

No font, renderer setting, cache or per-frame operation changes. The
bounded text optimizations and their measured CPU gains remain intact.
Evidence and the rejected rifle trial that prompted this check are in
[round sixty-three](reference-svg-round63.md) and
`/tmp/cp-eras-next/ek-native-coverage/`.


### EM follow-up: local encoded material without font compensation

The [rifle follow-up](reference-svg-round64.md) applies EK's measured
blend distinction through the existing software compositor. It adds no
font-weight correction, shared color-mode change or per-frame geometry
generation. Five cached images cover the ground and four ordinary rifles;
all twenty selections reuse them. The final production same-renderer paired
4K foreground CPU measurement improves 16.54→13.15 ms (20.5%), with roughly
12 MiB additional retained rifle pixels. This is a local material result, not evidence that
native and SVG glyph rasterizers have identical coverage. The font audit's
byte identity, layout checks and source-font limitations remain intact.


### LA follow-up: Dashboard footer-code reuse

The three Bold `68SD1D1100D1S` runs now opt into the existing
`ReusableWide` cache. Their local font, content and scaled size are
identical; each run retains its own transform and live ink/opacity.
At 1600 and 4K, separate counters verify one successful thirteen-path
extraction, two hits and one retained entry. The four-entry per-draw
bound, other callers, fonts and shared renderer are unchanged.

A fixed confirmation with 200 warmup draws, 256 samples per case and
balanced BAAB processes reduces warm foreground construction from
4.410 to 3.828 ms at 4K rest (13.2%), with the same percentage for hover.
At 1600, rest and hover improve 12.8% and 12.5%. The initial shorter
protocol had mixed 1600 results and substantial process drift; it is
preserved, not averaged away or described as a demonstrated speedup.
These are CPU construction measurements, not GPU, startup or live input
results. No frame-to-frame outline cache or additional paint pass is added.

Ten Wgpu state/size pairs and one tiny-skia 1600 pair remain pixel-exact;
custom/hover/held controls produce distinct states. Three fresh packaged
sizes reproduce the reviewed candidate. All 311 packaged tests, 22 checks
and 28 first-attempt matrix cases pass; the known one-pixel Neo-kitsch bar
residual is unchanged. No golden or reference SVG changes. See
[round eighty](reference-svg-round80.md) for full measurements and limits.


### LC follow-up: Dashboard SemiBold caption reuse

Six caption runs opt into `ReusableWide`, whose eligibility now includes
SemiBold alongside Bold. All other font, size, anchor and stretch guards
remain. The two caption triplets and Bold code use three of four available
per-draw entries, keyed by content, full font and exact scaled size.
Independent counters at 1600 and 4K verify three successful extractions
and six hits: thirteen Bold paths, nineteen Combat SemiBold paths and
fifteen Defence SemiBold paths. Transforms and ink remain outside the key.
No fonts, geometry, paint order, effect passes or persistent cache change.

One predeclared 200-warmup/256-sample BAAB experiment compares against
the already-optimized LA baseline. Warm foreground CPU construction falls
3.835→2.312 ms at 4K rest (39.7%) and 3.934→2.414 ms on hover (38.6%).
At 1600, rest/hover improve 39.2%/38.0%. Both candidate process medians
are below both baselines in all four cases; Astra reproduces all 4,096
raw-sample statistics. These are incremental CPU results, not live GPU,
startup, background preparation or whole-frame timings.

Ten Wgpu pairs, one tiny-skia pair, both counter sizes and three fresh
packaged captures are pixel-exact. A permanent regression verifies
distinct Bold/SemiBold outlines for identical caption text and sizes,
including subsequent cache hits. All 312 packaged tests, 22 checks and
28 first-attempt matrix cases pass, with the known one-pixel Neo-kitsch
bar residual unchanged. See [round eighty-one](reference-svg-round81.md).


### LG follow-up: Kitsch Store band reuse

Four ordinary/grown band-label tags use the existing per-draw outline
route. Eighteen resting/selected traversals now require two extractions
and sixteen hits, retaining two entries; opening uses two misses and four
hits. Fonts, shared renderer, transforms and ink are unchanged. One fixed
four-process comparison reduces warm foreground CPU construction from
20.377 to 16.554 ms at 4K rest (18.8%), with 18.9% hover and 20.5% at 1600.
Astra reproduces all 4,096 samples. Eighteen image pairs, five counter pairs
and three fresh packaged sizes are exact. All 312 packaged tests, 22 checks
and 28 first-attempt matrix cases pass. The preserved failed selection
hook, vacuous selected-held control and CPU-only scope are explicit in
[round eighty-two](reference-svg-round82.md). No font, artwork, effect pass,
cache-bound or golden change follows this optimization.


### LN follow-up: Kitsch Dashboard header reuse

Six SemiBold header-note tags share three repeated outlines within a draw,
reducing six direct extractions to three misses and three hits. The existing
four-entry bound, full-font/size key, renderer and font bytes stay unchanged.
Seventeen paired images and both counter sizes are exact. One fixed warm
four-process comparison improves foreground CPU construction 16.072→14.275 ms
at 4K rest (11.2%), 10.9% hover and 12.2–12.3% at 1600. Astra reproduces all
4,096 raw samples. The first timing fixture selected the wrong module; it
was corrected before any timing, and the preserved v1 was never measured.
Three fresh packaged sizes, all 312 packaged tests, 22 checks and 28
first-attempt matrix cases pass. Palette/selected-hover controls that do not
change visible pixels and CPU-only timing limits are explicit in
[round eighty-three](reference-svg-round83.md). No new font, paint pass,
global cache or source-art correction follows this optimization.


### LT/LV follow-up: Entropism captions accepted, Neomil title trial rejected

Entropism Dashboard's three repeated SemiBold caption keys now share
outlines across six modules: eighteen direct runs become three misses and
fifteen hits. Three declarations opt into the existing four-entry per-draw
cache. Fonts, shaping, coordinates, ink, effects and renderer policy stay
unchanged. One fixed four-process comparison improves warm 4K rest
foreground CPU construction 11.912→6.930 ms (41.8%). Twenty-one pixel pairs, state,
palette, partial-glyph, backend and counter checks pass, as do three fresh
packaged sizes, 312 packaged tests, 22 checks and 28 first-attempt matrix
cases.

The Neomil primary-title trial also preserves pixels and removes four
extractions, but its frozen comparison shows only 0.6% improvement at
4K rest and a 0.5% hover regression, with overlapping process ranges.
The 3.0–3.6% smaller-size gains do not establish a useful 4K benefit.
That helper change and its test-only correction stay in scratch. Astra
verifies all 8,192 raw samples across both trials. These measurements
exclude GPU presentation, startup and backdrop preparation. See
[round eighty-four](reference-svg-round84.md) for controls and limits.

### MJ: remaining cache callers

A source review after round eighty-five finds no new measured cache
opportunity among the shipped callers. The repeated eligible Bold and
SemiBold runs already use the bounded per-draw outline cache. Apparent
duplicates with different font weights or sizes have different keys;
rotated tracked paths and static software layers have separate bounded
caches. The remaining Neomil primary-title reuse is the rejected LV trial
above. No new timing or broader optimality claim follows from this review.
Another optimization needs a specific uncached repeated-path or recurring
static-layer miss in a real state. The pinned source review and Astra's
checks are in `/tmp/cp-eras-next/mj-next-text-efficiency/`.
