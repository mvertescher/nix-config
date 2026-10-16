## Current remaining work (2026-10-04)

The requested [font and text-effect audit](todo/font-rendering.md) and its
Neomil painting follow-up are complete. Actual bundled Latin font bytes and
weights pass; layout semantics and earlier corrections have been audited.
Earlier font/advance and rotated-path caches preserve accepted pixels
while reducing measured CPU work; their full evidence remains in the
[audit](docs/font-rendering-audit.md). Later bounded follow-ups are listed
below. Source-font identity and live GPU performance remain separate.

The current [group-opacity audit](docs/group-opacity-audit.md) verifies an
actionable rendering defect: Neo-kitsch panel text becomes too bright
during its existing fade because overlapping shapes fade individually.
The fresh package replays the rest golden, but 2,529 mid-fade pixels
exceed both endpoint colors. The scoped GPU fixture now passes exact
endpoint and intermediate-color checks with native Rajdhani text and
reused textures. Compatible texture views also remove shader color
conversions. The Motion scope owns actual screen integration, cache
invalidation, backend coverage and cost checks; no runtime change from
this study is accepted yet. The real-panel layer split now passes six
exact packaged comparisons, including 4K and fractional size, with
draw-site activation verified. The native parent-renderer bridge now
passes RGBA/BGRA endpoint, intermediate-color, layer-order and reuse
checks in scratch. Preserving native MSAA layer batches now resolves
the scoped candidate's 91-pixel fractional-scale mismatch with unchanged
direct controls. All 104 normalized fixture frames pass, including
cached input and isolated transform/content invalidation; optional-feature
compilation is repaired too. NT/NV now connects the actual Dashboard:
the 2,529 over-bright pixels become zero, while hidden/rest endpoints
and 4K/fractional rest stay exact. The scratch package passes 312 tests.
Thirteen same-process cache controls now pass, and the larger-size fade
checks remove 21,898 over-bright pixels at 4K and 4,785 at fractional size.
Other backends and cost still precede production integration. OB/OE now
preserves all nine endpoint/midpoint captures while
releasing group textures at exact hidden/rest endpoints. OD/OF's 23-step
test verifies release and re-entry, including 99,532,800→0→99,532,800
nominal group pixel bytes at 4K. Intermediate full-viewport storage remains
experimental. OG/OI's 112 state frames preserve endpoints and warm replay,
with midpoint error below half a channel level. Its small palette change
does not meaningfully alter the fixed opaque backing; broader palette
exposure is not claimed. OL/OM completes 1,664 fixed cost samples: the
exact 4K portion shows the prototype is slower, so it remains unaccepted.
The full pixel gate catches two original-route grain pixels after a 4K→1600
resize; OQ/OS isolates the effect to retained Engine state without relaxing
the gate. OU removes the separate mix submission and preserves all state
and renderer-fixture pixels. Its paired 4K warm submission time improves
67–72% against the previous prototype; cold and total completion costs
remain unresolved. PD/PE attributes the large software cold spans to
endpoint creation/submission rather than pipeline setup. PH's offscreen
RTX 4070 comparison instead finds endpoint preparation dominates roughly
72 ms cold frames; both variants produce identical hardware pixels.
PI now reuses one child across the two endpoint draws. It preserves all
112 state and 104 renderer-fixture images. In 832 paired cost samples,
RTX 4070 cold frames including completion improve 71→43–44 ms (38–39%);
software timings remain mixed. PR/PS verifies all eleven panel text runs
against bundled font bytes and native ink bounds at three sizes on both
drivers. PT/PU's GPU crop preserves all 106 native fixture images per
driver, with expected retained-memory changes for partial clips. PW/PX then
preserves all 112 Dashboard state frames and paired 4K frames per driver,
reducing nominal retained fade storage from 99.5 to 22.0 MB (77.85%). The
font/size/transform guard falls back outside measured cases and reuses font
checks on warm frames. PZ's 832 fixed samples preserve pixels and counters;
hardware passes the 15% cost guard, but cold completion stays 44–45 ms and
the separate warm-speed gate fails. Software remains mixed. QD/QF now
verifies early fade pixels and reduces cropped GPU misses from nine to one
across the sampled 401–500 ms times, preserving all CPU endpoint revisions
and full-render fallback. Native endpoint draws fall from 18 to two;
elapsed early-frame cost is not yet measured. QH's corrected clip control
checks selection, palette, font-version and animated-backing invalidation
in one process. Integration and broader backend coverage remain open;
no production or desktop-latency claim.

There are 29 open source, toolkit, asset and live scopes. The latest
[runtime checkpoint](docs/reference-svg-round85.md) corrects weak resting
DELETE text in Entropism's working mail client. It uses existing darker
ink only for the reference palettes, preserving the font, shaping,
effects, other states and custom palettes. Seventeen paired captures,
312 packaged tests, 22 checks and 28 first-attempt matrix cases pass.
Only 136 glyph pixels change in its 1600 golden; the packaged result
exactly matches the reviewed candidate. The broader Data display scope
still needs callers for its remaining toolkit work.

The preceding [runtime checkpoint](docs/reference-svg-round84.md) shares Entropism
Dashboard caption outlines, reducing eighteen extractions to three.
Warm 4K rest foreground CPU construction improves 11.912→6.930 ms (41.8%), preserving
fonts, layout, effects, state behavior and pixels. Three fresh packaged
captures, 312 packaged
tests, 22 checks and 28 first-attempt matrix cases pass. Goldens and SVGs
stay unchanged; GPU and startup performance remain outside these timings.
The Neomil primary-title trial preserves pixels and removes extractions,
but its mixed 4K timing does not justify integration; it stays in scratch.

The preceding [runtime checkpoint](docs/reference-svg-round83.md) reuses three repeated
Kitsch Dashboard header-note outlines. Six extractions become three,
with exact pixels and unchanged fonts, renderer and cache bounds.
Warm foreground CPU construction improves 16.072→14.275 ms at 4K rest
(11.2%), 10.9% hover and 12.2–12.3% at 1600. Seventeen image pairs, both
counter sizes, three fresh packaged sizes, 312 packaged tests, 22 checks
and 28 first-attempt matrix cases pass. Goldens and SVGs stay unchanged.

The [Neo-kitsch frame study](docs/neokitsch/store-frame.md#lolp-background-intervals-for-lower-frame-profiles-2026-10-04)
rejects exact donor-background identity and validates a bounded interval
on 3,200 new clear pixels. Its conditional excess profiles keep background
uncertainty separate from stroke/halo inference; NK-14 remains open.
The subsequent [whole-scene tests](docs/neokitsch/store-frame.md#lzma-complete-photographic-svg-check-2026-10-04)
pin all five font faces and verify pixel-exact ground injection. The
existing line-glow pass adds too much surrounding ink, but removing it
leaves missing printing and narrow peaks. Text glow has no effect in the
measured strips. All three pass-omission variants fail the fixed source
checks; no new material or native effect is adopted.
The proposed isolated top controls also fail their fixed ground-donor
check in two lower bands; target-profile measurements stop before fitting.
See the [top-control result](docs/neokitsch/store-frame.md#mo-outer-top-donor-check).
The [straight-side study](docs/neokitsch/store-frame.md#msmt-straight-side-registration)
also rejects a shared fifth-strand shift: three cards have asymmetric
flanks, while the third card's isolated pass disagrees with its neighboring
lines. Astra reproduces every pixel and edge measurement against fresh
packaged captures and the pinned SVG. No geometry or material is changed.
The independent [opposite-side control](docs/neokitsch/store-frame.md#mvmx-opposite-side-control)
then rejects the third card's proposed one-pixel whole-card shift in all
48 fixed comparisons. That hypothesis stops before an artwork trial.

The preceding [Store checkpoint](docs/reference-svg-round82.md) reuses repeated Kitsch
Store band-label outlines. Four tags reduce eighteen extractions to two
per resting draw, with unchanged pixels. Warm foreground CPU construction
improves 20.377→16.554 ms at 4K rest (18.8%), 18.9% hover and 20.5% at
1600. Eighteen image pairs, five counter pairs, three fresh packaged sizes,
312 packaged tests, 22 checks and 28 first-attempt matrix cases pass.
Fonts, shared renderer, cache bounds, goldens and SVGs are unchanged.

The [footer source study](docs/neomil/dashboard-footer.md#lhli-source-text-displacement-2026-10-04)
finds repeated text displacements, but the position-only transfer loses
bright glyph edges and local controls despite aggregate gains at all
three sizes. That trial stays out of production and NM1 remains open.

The preceding [Dashboard checkpoint](docs/reference-svg-round81.md) shares the repeated
SemiBold Dashboard footer-caption outlines. Against the preceding Bold-code
optimization, warm foreground CPU construction improves 4K rest
3.835→2.312 ms (39.7%), with 38.6% hover and 38.0–39.2% smaller-size gains.
Ten Wgpu pairs, tiny-skia, both counter sizes and three fresh packaged
captures preserve pixels. All 312 packaged tests, 22 checks and 28
first-attempt matrix cases pass, including a font-weight separation
regression. Goldens and SVGs are unchanged. The cache still lasts one draw
and retains at most four entries; no effect pass or font change is added.
The preceding [Bold-code checkpoint](docs/reference-svg-round80.md)'s mixed
initial timing and confirmed gain remain recorded separately.

The preceding [frame checkpoint](docs/reference-svg-round79.md) closes
ordinary Neomil Store primary frame coverage. One bottom correction derived from measured
source coverage improves all changed native pixels at three sizes. Sixteen
state cases preserve feedback, selection, custom colors and opening clips.
All 311 packaged Rust tests, 22 repository checks and 28 first-attempt matrix
cases pass, along with both Store gates and three-size package parity.
Twenty-six of 27 visual images are exact; the existing Neo-kitsch bar
residual remains one pixel at one channel level. The change adds no
primitive, text work or effect pass. Its conditional backing model and
isolated SVG corner/cap losses remain explicit in the
[primary-frame study](docs/neomil/store-ordinary-frame.md).

The preceding [preparation optimization](docs/reference-svg-round75.md)
shares repeated contour-distance calculations. Fresh paired measurements
reduce initial 4K Store preparation from 627 to 462 ms (26.3%); smaller
sizes improve 52–53%. Retained payload, shelf hashes and the twenty-selection
cache state sequence match. These are CPU preparation measurements, not
warm drawing, GPU or startup timings. The optimization preserves artwork,
fonts and text caches. The [frame-echo study](docs/neomil/store-frame-echo.md)
retains the accepted round-74 artwork and its source limitations. Future
glyph fitting must start with the [font audit](docs/font-rendering-audit.md)'s
font, layout and no-op replay checks.

Rounds through 54 are committed in `70b0ed6`; their dated staged-state notes
below are historical. The later audit and follow-ups remain staged and uncommitted.

The [cached rifle follow-up](docs/reference-svg-round64.md) ports the
accepted stock details through existing encoded-RGB composition and corrects
the cropped fourth rifle's placement. Source fit improves at all three sizes;
selection, opening and seven feedback pairs pass. Final production timing
improves warm 4K foreground CPU drawing 16.54→13.15 ms (20.5%), while
reusing its images across all twenty selections. All 305 local/packaged Rust
tests, 22 repository checks and 28 first-attempt matrix cases pass. Fresh
packaged 4K pixels exactly match the reviewed candidate. Fonts and shared
color rendering remain intact. All 29 broader scopes remain open.

The [maker scan and title audit](docs/reference-svg-round68.md) confirms
that Entropism's SVG title calibration does not require a native port.
A fresh Neomil packaged 4K capture and the current 1600 matrix frame
reproduce the accepted baseline. An
independently measured scan phase warrants one fixed material test, but
its actual SVG transfer fails word/size controls after six exact rendering
no-ops. No font, runtime or artwork change follows. All 29 broader scopes
remain open; the audit record stays staged and uncommitted.

The [warning placement follow-up](docs/reference-svg-round67.md) combines
that frozen hollow with one independently derived ordinary x offset.
Whole-mark source comparisons improve on all four cards at all three sizes
in SVG and native; the fourth-card side loss is resolved. Small fractional
foot/exclamation losses remain explicit. Structural no-ops and twelve paired
states pass, and component/native copies are synchronized. Both Store gates,
305 packaged Rust tests, 22 repository checks and 28 first-attempt matrix
cases pass. Fresh packaged 4K pixels match exactly. All 29 broader scopes
remain open; changes stay staged and uncommitted.

The [complete-profile follow-up](docs/reference-svg-round66.md) confirms
narrower warning contrast in the source than in both SVG and packaged Iced.
One hollow-only correction improves all whole-mark comparisons, but native
fourth-card left-side error rises 14–22% and its foot also loses detail. The
trial stays out of production. Independent frame/stem controls then confirm
rightward ordinary-card contrast offsets and near-zero selected placement.
That checkpoint preserved production geometry; round sixty-seven evaluates
the bounded placement follow-up. All 29 broader scopes remain open.

The [warning-edge review](docs/reference-svg-round65.md) preserves the
current artwork. Fixed side controls do not isolate a new foot contour:
opaque-ink inference fails, and truncated SVG profiles invalidate the pooled
width comparison. Independent review corrects the measurements and confirms
the warning's actual SVG ancestry. Any next contour trial needs complete,
corresponding edge profiles; no font or shared renderer change follows.

The [native rifle review](docs/reference-svg-round63.md) rejects the joint
bright-ink/seam/band port: 4K improves, but smaller rail and receiver
crops lose detail. Fixed-mask diagnosis points to edge coverage; preserve
native art while testing that rendering difference. A controlled edge probe
confirms the intentional linear-native versus encoded-SVG blend difference;
future glyph fits must account for it before changing font weights. Fonts
and caches stay unchanged, and all 29 broader scopes remain open.

The [stock-band follow-up](docs/reference-svg-round62.md) adds the lower
segmented band to the Entropism source SVG and component. A corrected
junction improves every fixed segment comparison at three sizes while
preserving the slot and upper seam. Native ink remains under review;
all 29 broader scopes stay open. Both fidelity gates and the full check
pass (22 checks, 28 cached matrix cases; runtime package unchanged).

The [tiny-print/seam round](docs/reference-svg-round61.md) adds a missing
ordinary Entropism rifle seam to the SVG and matching component. Its
native trial loses source detail and stays out of Rust. Kitsch placement
and Neomil scan-modulation trials also fail transfer; the intended fonts
and accepted caches remain intact. Local native ink and exact printing
stay open, alongside all 29 broader scopes. Both Store fidelity gates,
24 SVG structure checks and the fresh full 22-check/28-case run pass;
the package and cached visual outputs are unchanged.

The [text/echo recheck](docs/reference-svg-round60.md) verifies current
Neo-kitsch packaged captures and corrects a historical fractional text
measurement. The intended bundled font is selected correctly; remaining
printing differences do not justify a shared baseline or weight change.
Neomil's frozen secondary profile still loses small-size/text detail after
the rim correction, and the veneer bridge remains unconfirmed in original
RGB. Only documentation changes; EF remains the verified runtime checkpoint.

The [footer coverage round](docs/reference-svg-round59.md) corrects excess
ink along the Neomil Dashboard's bottom rim using existing solid drawing
primitives. Every changed native pixel improves source RGB at three sizes;
seven state pairs and production/package parity pass. All 303 local and
packaged Rust tests, 22 repository checks and 28 matrix cases pass. The
trace, component and two Dashboard goldens are synchronized. Plaque/fan source diagnostics retain
their unresolved boundaries; all 29 broader scopes remain open.

The [sampling/junction round](docs/reference-svg-round58.md) adds a tested
uniform source sampler to prevent fractional geometry errors. All sixteen
canonical source samples are exact; nine tests and all 22 repository
checks pass, with 27 visual cases plus the new sampler case. The runtime
package is unchanged. Entropism's historical gap-loss count falls from
15 to 12 after accounting for the already accepted M; the Neomil grating
still lacks a supported lower junction. All 29 broader scopes remain open.

The [plaque/feet round](docs/reference-svg-round57.md) establishes Neomil's
unequal source hatch pitch and a coherent local phase. Its baked-centerline
trial improves the plaque at three sizes but loses leading/A2 detail.
Kitsch foot rounding still loses selected-card detail after correcting
inadvertent long-side movement. Fractional-comparison and scratch native
provenance errors are corrected before review. No artwork is integrated;
all 29 broader scopes remain open and the staged runtime stays at DZ.

The [ground/paint round](docs/reference-svg-round56.md) validates a local
Login-photo background estimate for Store subtitles. Both complete-echo
trials still fail local or small-size controls. Entropism's exterior font
pixels are narrowed to a whole-run painting interaction, while mailbox
phase and automatic veneer tracing provide no accepted geometry change.
These diagnostics preserve the completed font/performance audit and all
29 broader open scopes; no production artwork changes in this round.

The [font-replay/printing round](docs/reference-svg-round55.md) establishes
an exact complete-run Entropism SVG font replay, but rejects its contour
trial for shape and exterior-pixel losses. A Neomil subtitle row knockout
also worsens both sizes: low red contrast does not establish missing ink
over blue photographic bands. Neo-kitsch's lower grain needs a bounded
fan family with supported exits, rather than one truncated ridge. These
verified diagnostics add no artwork changes or task closures. All 29
broader scopes remain open; DZ is the latest verified runtime change.

The [donor/silhouette/subtitle round](docs/reference-svg-round54.md)
checks Kitsch Vehicles against guarded Mail/Store donor pixels, reproduces
the Neo-kitsch isolated-letter diagnostic, and confirms missing Neomil
ordinary subtitle impressions. The full subtitle copy fails local dark-ink
controls in SVG and actual native captures; a five-row opacity fit also
fails transfer. These trials remain in scratch. All 29 broader scopes stay
open at that checkpoint, when CZ was the latest tested implementation.

The [coupled-ground/font round](docs/reference-svg-round53.md) verifies
the Kitsch candidate through nine complete scene renders. Exposed-ground
averages improve but Vehicles card interiors regress, preventing a native
port. An official version-matching Exo2-Light font is recovered; its fixed
substitution loses bright lettering at 1600 and is rejected. The font's
document record does not establish label identity. The queue audit confirms
17 source-fidelity scopes, six caller/design prerequisites, two asset
scopes and four live checks. All 29 remain open; changes stay staged and
uncommitted at that checkpoint, when CZ was the latest tested implementation.

The [completion campaign](todo/completion.md) tracks the active lanes and
integrated verification for the request to work through all open items.
The earlier [edge round](docs/reference-svg-round43.md) integrates the
source-reviewed T2 numeral and calibrated Neomil inner-left footer edge.
Both state gates, production/package parity, 294 Rust tests, four fidelity
gates and the full 22-check / 27-case verification pass. All 29 broader boxes
remain open, with source-based work distinguished from external prerequisites.
The [geometry round](docs/reference-svg-round44.md) integrates the
bend-preserving native frame correction after three-size source and ten
state checks. Production/package parity, 294 local/Nix tests, both Store
gates and all 22-check/27-case verification pass. The separate SVG tail
and fan widening are rejected; the M transfer remains experimental.
The [mechanism round](docs/reference-svg-round45.md) verifies exact
Kitsch font-copy and original-outline topology replays, rejects the
hint-removal and Entropism M trim trials, and completes the native Neomil
outer-left footer correction.
CF passes production/package parity, 294 local/Nix tests and full
22-check/27-case verification; 26 cases are exact and the one-level
Neo-kitsch bar residual is unchanged. CH aligns the Entropism selected
component title with its accepted parent (1,296/410 title-only changed
pixels at full/half size; parent crop exact). CH passes all 22 repository
checks and 27 visual cases, with the same exact package as CF.
All 29 broader boxes remain open; changes stay staged/uncommitted.
The [Store lettering round](docs/reference-svg-round46.md) integrates the
selected Entropism SVG M and synchronized component, the named native
Kitsch M face, and the Neomil lower-A2 slit after source review. All 295
local and Nix Rust tests, six state pairs for each changed native era,
production/package parity, five fidelity gates, and full 22-check/27-case
verification pass. The 322 tested source files and nine originals match
their frozen hashes; no broader item closes.
The [replay/material round](docs/reference-svg-round47.md) stages the
source-reviewed native Entropism selected M and the shared production-font
Kitsch Store/component SVG correction. Seven selected-title state pairs,
custom-purple selected-away and a 500 ms fade pass; the Kitsch SVG retains
its documented 1600 gap/bright-pixel losses. SVG-only font and visible
Neo-kitsch veneer trials are rejected. The CQ checkpoint passed: 295 local/Nix Rust tests, three-size production parity,
four fidelity gates, 24 SVG checks and all 22 repository checks pass.
All 27 visual cases pass first attempt (26 exact; the unchanged
one-pixel Neo-kitsch bar residual). All 323 tested source files and nine
original photos match their frozen hashes. The fresh packaged 4K
Entropism Store frame and installed Kitsch font match their reviewed
references exactly. E4, K5 and all 29 broader boxes remain open.

The [broader material/notice round](docs/reference-svg-round48.md)
studies Kitsch fan overlap, full Neomil inactive notices and orientation-aware
Neo-kitsch ridge continuation. CW integrates the nearest-ghost SVG,
component and native correction after three-size source review and seventeen
paired states. All 295 local/Nix tests, both fidelity gates, 24 SVG checks
and the full 22-check/27-case matrix pass; the packaged 4K frame is exact.
Notice and grain studies retain their rejected trials and diagnostic findings. The queue
audit separates asset, caller/design and live-desktop prerequisites from
actionable fits.

The [stroke/weight round](docs/reference-svg-round49.md) separates deeper
fan stroke ownership from fill opacity and notice weight from advances.
CZ integrates deeper-stroke occlusion in SVG, native and the component
after source/state review, accepting a measured +6.48 ms 4K cold CPU cost
with unchanged cache hits and retained bytes. All 295 local/Nix Rust tests,
both fidelity gates, 24 SVG checks, all 22 repository checks and 27 visual
cases pass. The packaged 4K frame is exact; all 16 original source images
are unchanged. The lighter face, shared-spacing, opacity and contour
trials do not pass all canonical/local controls and remain unapplied.
All 29 broader boxes remain open; changes stay staged and uncommitted.

The linked workstream records preserve the implementation history. Earlier
"still open" lists are superseded by later completion entries, including
the five-task feedback/native-control/extractor batch committed in `de56b25`.
The earlier 82-file correction batch landed in `79c1a02`; its dated
"staged" validation notes describe the state before that commit.
The N–AC corrections and resize work are included in the current commit;
their dated "staged" notes likewise preserve the state during validation.

- **All-era reference SVG fidelity:** reviewed all 24 SVGs: 16 against
  source images, four against parent traces, and four as original bars.
  All 16 source gates pass, but
  the original audit found missing content, simplified artwork, geometry,
  typography and material differences. The later batches below resolve
  primary defects; named local typography/printing/material residuals remain. The [reference correction plan](todo/reference-svg.md)
  lists concrete work; the [audit](docs/reference-svg-audit.md) records
  coverage, source evidence and gate limits. The first correction batch
  completes nine tasks and parts of E2/K2 across ten screens. The
  [second batch](docs/reference-svg-round2.md) adds Kitsch login contours,
  Neo-kitsch badges/envelopes, Neomil socket spacing/current documentation,
  and further Entropism/Kitsch text fitting. Materials, artwork, exact
  font shapes and small typography residuals remain open. The
  [third batch](docs/reference-svg-round3.md) corrects Kitsch's fourth-card
  cutoff and Neo-kitsch envelope placement; the latter retains the
  source-supported 60.2px row pitch.
  The [fourth batch](docs/reference-svg-round4.md) calibrates Entropism's
  mailbox baseline and corrects Neo-kitsch store tabs and shoulders.
  The [fifth batch](docs/reference-svg-round5.md) corrects Entropism
  paragraph/badge geometry, Kitsch lower store-card corners and stale
  Neo-kitsch component/provenance examples.
  The [sixth batch](docs/reference-svg-round6.md) separates Neo-kitsch's
  selected mailbox inks, improves Neomil footer type/printing and closes
  Kitsch citation cleanup. Fine footer printing and font fitting remain.
  The [seventh batch](docs/reference-svg-round7.md) verifies primary login
  artwork/printing, broad grounds and lower image-upload cost; selected
  inverse weapon detail and remaining materials continue. Bars are original
  designs. The [eighth batch](docs/reference-svg-round8.md) verifies broad
  login/store materials, primary store artwork and state/clipping behavior;
  local typography, certification, veneer flow and fine printing continue.
  The [ninth batch](docs/reference-svg-round9.md) validates the recovered
  L/M work and N fixes: all 269 Rust tests, 22 repository checks and 27 exact
  visual matches pass. Source-derived margin CJK is deterministic; mailbox
  upper contours, socket scatter and maker tails are now verified. The
  [tenth batch](docs/reference-svg-round10.md) verifies store/mail/dashboard
  typography and printing, Kitsch face/ghost fields and a bounded Neomil
  resize preview: 272 Rust tests, 22 repository checks and all 27 exact
  visual matches pass. Selected-frame geometry and finer materials continue.
  The [eleventh batch](docs/reference-svg-round11.md) is verified:
  Z passes 275 Rust tests, all 22 repository checks and all 27 exact
  visual matches on their first attempt. V–Y native review accepts the selected frame,
  chip-2, dashboard labels, fan materials/ghosts, value baselines, mailbox
  ribs, lower frame echoes and veneer seam. Z calibrates the native footer
  code. The Neo-kitsch implementation gate's classification mismatch is
  corrected using strict component-mask evidence and negative controls;
  it now passes at 89%, with existing thresholds unchanged.
  The [twelfth batch](docs/reference-svg-round12.md) adds locally verified
  socket trails and asynchronous eligible resize preparation: 285 Rust
  tests, affected gates, all 22 repository checks and all 27 exact visual
  cases pass. The [thirteenth batch](docs/reference-svg-round13.md) verifies
  the Neomil margin word/slash, Kitsch badge apertures and native shelf
  brands, and Neo-kitsch action labels: 286 Rust tests, all 22 checks and
  27 exact visual cases pass. The [fourteenth batch](docs/reference-svg-round14.md)
  then corrects the small margin plaque boundary with ten native-state
  captures, both gates and another full check: 22 checks and 27 exact cases.
  The [fifteenth batch](docs/reference-svg-round15.md) improves central
  plaque ink and native footer-caption heights. Seventeen accepted native
  captures, 286 Rust tests, all 22 checks and 27 exact visual cases pass;
  201 frozen hashes match the Nix source. Kitsch opacity and Neo-kitsch
  grain trials are rejected after local controls fail. AD remains staged.
  The [sixteenth batch](docs/reference-svg-round16.md) follows with a
  bounded Kitsch SC type fit and Neomil second-A counter gap; Neo-kitsch
  crossing-grain trials remain rejected. Exact glyph/ink and junction
  fidelity remain open. AE passes 286 Rust tests, 20 native captures, four
  fidelity gates and all 22 repository checks; all 27 visual cases match
  exactly on their first attempt. Changes remain staged.
  The [seventeenth batch](docs/reference-svg-round17.md) corrects Entropism
  product-heading baselines after native source/state review. Exact glyph
  contours remain open. Neo-kitsch frame and Neomil rib-width trials fail
  independent controls and are rejected. AF passes 286 Rust tests, both
  affected fidelity gates and all 22 repository checks; all 27 visual cases
  match exactly on their first attempt. Changes remain staged.
  The [eighteenth batch](docs/reference-svg-round18.md) corrects Kitsch
  warning height, Neo-kitsch login badge lettering/contours and Neomil
  top-caption secondary copies. Local source and fractional-state controls
  pass, with exact glyph/ink/texture limits retained. AG passes 286 Rust
  tests, all six affected fidelity gates and all 22 repository checks; all
  27 visual cases match exactly on their first attempt. Changes remain staged.
  The [nineteenth batch](docs/reference-svg-round19.md) corrects Entropism
  SVG title width and inactive Neomil login notice placement. Local source,
  native and fractional-state checks pass with glyph/gap/ink limits
  retained. AH passes 286 Rust tests, four affected gates, all 22 repository
  checks and 27 exact visual cases. A pre-existing fractional notice
  overhang is recorded separately. All changes remain staged.
  The [twentieth batch](docs/reference-svg-round20.md) corrects locked-login
  notice ink and contains all three notice blocks in narrower windows.
  Both fidelity gates, 288 Rust tests, 22 repository checks and all 27
  visual cases pass at reported 100.000%; 220 frozen hashes match the Nix source. Custom
  palettes and control feedback are preserved. Changes remain staged.
  The [twenty-first batch](docs/reference-svg-round21.md) corrects the
  Neo-kitsch A contour and active Neomil notice's lower printing copy.
  Source/native/state/cache review, 288 Rust tests, four gates and all 22
  repository checks pass. All 27 visual cases pass on the first attempt;
  both changed login goldens are pixel-identical. Direct matrix comparison is documented,
  including one pre-existing one-level bar pixel; 222 frozen hashes match
  the Nix source.
  Bounded follow-ups close; fine glyph/printing/material residuals remain.
  The [twenty-second batch](docs/reference-svg-round22.md) integrates
  bounded Kitsch idle-outline, Neo-kitsch B-contour and Neomil inner-O
  corrections after local source/native/state review. All 288 Rust tests,
  six fidelity gates and 22 repository checks pass; all 27 visual cases
  pass first attempt. Broader glyph/material/printing residuals stay open.
  All changes stay staged and uncommitted.
  The [twenty-third audit](docs/reference-svg-round23.md) confirms missing
  ordinary store-title printing and shallow mailbox terminal geometry.
  Candidate fixes fail counter/gap or endpoint controls; the veneer join
  trial also remains unaccepted. Findings and concrete follow-ups are
  recorded, with production code, SVGs and goldens unchanged from AK.
  The [twenty-fourth batch](docs/reference-svg-round24.md) integrates
  the deeper mailbox terminal and bounded interior material after source,
  native and state review. Both gates, 288 Rust tests, all 22 repository
  checks and 27 first-attempt visual cases pass; changes remain staged.
  Exact fine printing remains open. The repeated store-title candidate
  preserves solid letters and is queued for native calibration.
  The [twenty-fifth batch](docs/reference-svg-round25.md) integrates that
  title printing and bounded Kitsch certification detail. Source/native
  and state controls, 288 local Rust tests, four fidelity gates and all
  22 repository checks pass. The matrix passes all 27 cases after one
  package-build retry; two intermittent font-measurement failures have a
  concrete [test-initialization follow-up](todo/verification.md#concurrent-font-initialization--an).
  Exact printing residuals and the additional 5.55 ms of warmed 4K title
  CPU work remain assigned to the next lanes. Changes remain staged.
  The [twenty-sixth batch](docs/reference-svg-round26.md) reuses title
  outlines for an 11.1% paired 4K CPU reduction with exact pixels and
  fixes concurrent test-font registration. All 290 local/Nix Rust tests,
  22 repository checks and 27 visual cases pass without retries. No
  golden changes; exact printing and live desktop work remain separate.
  The [twenty-seventh batch](docs/reference-svg-round27.md) corrects the
  selected Entropism manufacturer label. Thirteen paired states, both
  gates, 290 Rust tests, all 22 repository checks and 27 first-attempt
  visual cases pass. Its ordinary transfer is rejected for source-gap
  regressions; the follow-up remains open. Changes remain staged.
  The [twenty-eighth batch](docs/reference-svg-round28.md) verifies the
  ordinary Entropism labels: fourteen paired states, thirty fixed-ink
  crops, both gates, 290 Rust tests, all 22 repository checks and 27
  first-attempt visual cases pass. Kitsch height/cache and SC ink trials
  remain rejected; their follow-ups stay open. Changes remain staged.
  The [twenty-ninth batch](docs/reference-svg-round29.md) corrects Entropism
  mailbox T contours and the T1 gap. Thirteen paired states, both gates,
  290 Rust tests, all 22 repository checks and 27 first-attempt visual
  cases pass. Production and packaged frames match the reviewed candidate.
  Small edge/core residuals remain in the broader fidelity task. Kitsch's
  baseline trial is rejected; one veneer join awaits a clearer asset.
  All changes remain staged and uncommitted.
  The [thirtieth batch](docs/reference-svg-round30.md) records the Kitsch
  glyph/spacing and Neo-kitsch socket-grain investigations. Both candidates
  are rejected after source and size controls; implementation and goldens
  remain at the verified twenty-ninth batch. Eight recurring/cautionary
  checklist markers become standing bullets, leaving 29 real open boxes.
  The [thirty-first batch](docs/reference-svg-round31.md) integrates the
  T3 numeral and inferred Kitsch hub outline hover. All 292 Rust tests,
  22 repository checks and 27 first-attempt visual cases pass; production
  frames match the reviewed candidates. Paired compliance glyphs and
  socket grain remain unaccepted. Changes remain staged and uncommitted.
  The [thirty-second batch](docs/reference-svg-round32.md) corrects the T4
  numeral with eleven paired states, both gates, 292 Rust tests, all 22
  repository checks and 27 first-attempt visual cases. Production and
  packaged frames match. T2, whole-card alignment and frame-material
  trials remain unaccepted. Changes stay staged; 29 open boxes remain.
  The [thirty-third batch](docs/reference-svg-round33.md) verifies
  Neo-kitsch mailbox A/B/C/D glyphs, lower tabs and external chamfers.
  Ten paired states, both gates, 292 Rust tests, all 22 checks and all 27
  first-attempt visual cases pass. Production and packaged frames match;
  only the mailbox golden changes by 1725 pixels. Fine frame ink and
  photographic softness remain. Dashboard and store badge corrections follow in AW; changes remain staged.
  The [thirty-fourth batch](docs/reference-svg-round34.md) integrates seven
  dashboard/store section badges after three-size source/native review
  and eight paired states. Production and packaged parity, four gates,
  292 Rust tests, all 22 checks and 27 first-attempt visual cases pass.
  Two reviewed goldens change; broader fidelity remains open. A separate
  Neomil component dependency audit is assigned to AX. Changes stay staged.
  The [thirty-fifth batch](docs/reference-svg-round35.md) restores that
  missing component background and integrates bounded Neo-kitsch mailbox
  body and dashboard footer alignment after three-size native review and
  eight paired state captures. Production/packaged parity, four gates,
  292 Rust tests, all 22 repository checks and all 27 first-attempt visual
  cases pass. Local printing losses remain documented; 29 open boxes
  remain. Changes stay staged and uncommitted.
  The [thirty-sixth batch](docs/reference-svg-round36.md) verifies a bounded
  Kitsch warning bridge fit with nine paired states, three-size production
  parity, fresh packaged parity, both gates, 292 Rust tests, all 22 checks
  and 27 first-attempt visual cases. Only 24 golden pixels change. Neomil
  secondary-frame trials fail small-size controls; primary-edge geometry
  is a narrower follow-up. The 29 broader tasks remain open; work is staged.
  The [thirty-seventh batch](docs/reference-svg-round37.md) adds missing
  Neomil dashboard bottom-edge coverage. Three-size production parity,
  nine paired states/fallback, component and fresh packaged parity checks
  pass. Both gates, 292 Rust tests, all 22 repository checks and 27 first-attempt
  visual cases pass; each dashboard golden changes by 142 pixels. A mailbox
  body-baseline proposal awaits native review. Exact printing remains open.
  The [thirty-eighth batch](docs/reference-svg-round38.md) integrates a bounded
  Neomil mailbox body-baseline correction after source/native/state review. The
  1600 native image is identical; 4K phrase and line comparisons improve, with
  local counter losses retained. The dashboard top-band trial is rejected for
  false bright-core coverage at 1600. Three-size production and packaged 4K
  parity, both gates, 292 Rust tests, all 22 checks and 27 first-attempt
  visual cases pass. No goldens change. Exact printing and live-desktop tasks
  remain open; changes are staged.
  The [thirty-ninth audit](docs/reference-svg-round39.md) rejects Kitsch
  `ONLY` contours and a two-line placement trial after local counter/gap,
  word and small-size endpoint losses. Independent source correspondence
  supports a placement difference, but the tested font translation does
  not resolve it safely. Runtime, SVGs and goldens remain at BA; all 29
  broader tasks stay open. Documentation and prior work remain staged.
  The [fortieth batch](docs/reference-svg-round40.md) integrates bounded
  Kitsch compliance typography after three-size source/native review and
  fifteen paired state cases. All 293 Rust tests, both gates, 22 repository
  checks and 27 first-attempt visual cases pass. Production and packaged
  frames match the reviewed candidate; one store golden changes. Fine glyph,
  cap-row and gap losses remain recorded, so the 29 broader boxes stay open.
  Changes remain staged and uncommitted.
  The [forty-first batch](docs/reference-svg-round41.md) calibrates Neomil
  dashboard top coverage without BA's false 1600 core. Nine paired states,
  source/native review, production/package parity, both gates, 293 Rust
  tests, 22 repository checks and 27 first-attempt visual cases pass.
  Only the two dashboard goldens change, by 142 pixels each. A Neo-kitsch
  font/spacing trial is rejected for source-gap and cap-row losses. All
  29 broader boxes remain open; work stays staged.
  The [forty-second investigation](docs/reference-svg-round42.md) advances
  Neo-kitsch metadata spacing and Neomil inactive-notice geometry into
  native review. Source-copy, word/gap/counter and actual Store baseline
  checks retain local losses. The Store follow-up passes seven state pairs,
  three-size production and packaged 4K parity, both gates, 294 Rust tests,
  22 repository checks and 27 first-attempt visual cases. Only its metadata
  golden changes, by 3787 pixels. Login's baseline rounding helps, but
  three 1600 whole-line RGB regressions keep its candidate unaccepted.
  All 29 broader boxes stay open; changes remain staged and uncommitted.
- **Neomil dashboard performance:** compositor/cache fixes reduce
  the measured 4K preparation median from 36.61 s to 0.404 s while
  preserving the original image layers. Route returns stay cached.
  See the [performance record](todo/performance.md) for measurements,
  fidelity checks and remaining resize/live-desktop work.
- **Neomil mailbox fidelity:** the six audited correctness defects are
  fixed: startup clock, selection heading, source typography,
  materials, cartridges and omitted printing/row shading. Fine printing
  echoes and cartridge edge detail remain. The reference recheck’s lower
  reader corner/side step and upper contour are corrected and verified.
  Exact background texture needs
  original source material. See the [mailbox record](todo/neomil-mailbox.md)
  for validation and the separate live-desktop follow-up.
- **Neomil login correctness:** password containment, mouse submission
  and shared ground are fixed. The third batch corrects the extra Iced
  card color, notch/tab/chamfer and label-tracking drift while preserving
  custom palettes. Primary avatars/portraits, mask/card/header printing and
  reference bright inks are now corrected and verified. Inactive card/badge
  broad fields and frame inks also pass native review. AI corrects locked
  notice ink and narrow-window notice containment with source/native/state
  and full repository checks. AJ verifies the active-notice lower
  printing copy through source/native/state/cache and full repository
  checks; scan modulation and other fine printing remain. See the
  [login plan](todo/neomil-login.md) for the measured defects and preserved
  source details.
- **Neomil store correctness:** startup, selected/cropped bounds, wash
  contour, open edge, lower chamfer and shared ground are fixed.
  Broad card/navigation fields and primary source logo/rifles now pass
  native/state review. Primary certification/KIROSHI and deterministic
  margin CJK also pass. Independent ordinary/selected typography and footer
  fits are verified through T. V's selected top/right frame and side-printing
  correction passes native/state review and the full Z check.
  AB corrects the small MASURAO word and slash, preserving CJK. Its
  O/plaque leading edge is corrected in AC and central hatch width in AD;
  exact inner ink and the second-A junction remain local follow-ups,
  alongside repeated printing and fine scan echoes.
  Socket occupancy, measured spacing and per-card origins are corrected;
  directional printing echoes now pass AA native/state and integrated
  review; irregular fine grain remains open. See the
  [store plan](todo/neomil-store.md). Frozen goldens and broad shape gates
  passed despite the audited defects; retain live/state-specific checks.
- **Neomil dashboard source fidelity:** chip-2 ink passes local native/state
  review; Z corrects footer code height/baseline and AD calibrates the two
  native caption heights. Fine footer printing and
  exact glyph contours remain. Tiny tape-mark ink and maker microtext
  tails/edge softness are verified; exact horizontal scan detail remains. The fine
  background still needs its original asset or authoring recipe. Follow
  the [measured plan](todo/neomil-dashboard.md); broad shape-area gates do
  not establish detail completeness.
- **Kitsch dashboard hover:** inferred front-outline feedback is implemented
  and headless-verified; the fan stays stationary. See the [hub rule](docs/kitsch/dashboard-hover.md).
  Existing resting trails are not evidence of a sourced pointer state.
- **Kitsch store selected-card hit regions:** fixed with selection-aware
  bounds through the lower details and footer, retaining card4's cutoff.
  See the [toolkit record](todo/toolkit.md#current-runtime-follow-ups).
- **Animated interaction transitions:** need reviewed timing/easing and
  transition rules. Component sheets specify destination drawings only;
  existing boot-in/caret animation timings are not interaction timings.
- **Desktop verification:** native input/IME, hub navigation, feedback,
  and first-frame haze/route latency still need a live check. Headless
  tests and previews do not close these items.
- **Future widgets:** slider ticks, shared icons, spec/log rows, meters,
  tooltips/modals and tab widgets wait for actual callers or reference
  material, as detailed in the [toolkit roadmap](todo/toolkit.md).
  Do not build unused widgets to close its umbrella checkboxes.
- **Extraction diagnostic:** the Kitsch mailbox yellow extraction remains
  explicitly "do not fix" in the [pipeline record](todo/design-pipeline.md).

The three reader follow-ups are resolved: Entropism cursor wording,
Neomil's title-card index and the separate hollow Login-button slot.
See [the source audit](docs/source-followups.md); the password caret
was already correct, and the slot's state semantics remain unknown.

Follow-up audit: three native-event regression tests now exercise button
cancellation/disable, touch release/loss and selected-range Unicode commits
after input re-enabling. All five control tests pass using the headless
software renderer. No production behavior changed; OS IME/preedit and live
desktop verification remain open.

## Workstream records

- [All-era reference SVG correctness](todo/reference-svg.md): complete
  source audit, concrete corrections and derived-sheet consistency.
- [Neomil login correctness](todo/neomil-login.md): input containment,
  action behavior, source artwork, printing and materials.
- [Neomil store correctness](todo/neomil-store.md): startup, hit regions,
  clipped selection, shape corrections and source fidelity.
- [Neomil mailbox correctness](todo/neomil-mailbox.md): source-to-trace
  corrections, missing renderer details and runtime regression coverage.
- [Dashboard performance](todo/performance.md): measured cold-draw costs,
  cache invalidation, optimization work and fidelity requirements.
- [Neomil dashboard fidelity](todo/neomil-dashboard.md): the current source
  correction plan, accepted measurements, validation and remaining detail work.
- [Design pipeline and component sheets](todo/design-pipeline.md): source
  audits, trace corrections, gates and extraction diagnostics.
- [Bar styling](todo/bar.md): measured era chrome and implementation history.
- [SVG-to-iced conversion](todo/svg-to-iced.md): renderer coverage, scene
  conversion and the widget-layer decisions.
- [Toolkit, interactions and themes](todo/toolkit.md): reusable controls,
  theme plumbing, motion, caller-dependent widgets and desktop follow-ups.
- [Verification infrastructure](todo/verification.md): headless rendering,
  desktop renderer troubleshooting and golden-history notes.

## Historical validation before the reference campaign

These batch counts and staging notes describe their original checkpoints;
the current completion record above supersedes them.

The resumed batch adds six matrix/code ink fits, margin and vertical-brand
copies, and GO HOME heading/maker corrections. The earlier dashboard/reader
work was committed in `6da816e`, with the TODO split in `bc19ef8`; this batch
remains staged. The [printing audit](docs/neomil/dashboard-printing.md)
records source evidence and limitations. All 209 Rust tests and both
fidelity gates pass; native/fractional state and opening captures are
reviewed. The full repository check passes all 19 checks, including all
25 golden cases. Two dashboard goldens and Login's previously missing
hollow-slot golden are refreshed. Live verification remains separate.

The mailbox correction batch also remains staged. Its six audited fixes
and source-detail limits are recorded in [the mailbox TODO](todo/neomil-mailbox.md).
Final validation passes 238 Rust tests, both fidelity gates and all 19
repository checks, including all 26 goldens. The added unfrozen mailbox
case matches 100.000%; headless presentation scheduling is fixed without
changing the clock or comparison thresholds. Only the mailbox golden
changes in this follow-up; the prior staged work is preserved.

The first login/store correction batch is also staged. It fixes login
password containment and pointer submission, store startup and selected/
cropped geometry, and the shared ground on both screens. All 249 Rust
tests, all four screen fidelity gates and all 19 repository checks pass,
including all 27 visual cases. Only the Neomil login/store goldens were
refreshed in this batch. Remaining artwork, typography and card-material
corrections stay open in the linked plans; live verification remains
separate.
