# Reference SVG round sixty-eight — maker scan and title calibration

The accepted font selection, shaping calibrations and bounded text caches
remain unchanged. EZ rechecks whether Entropism's SVG title correction
should also be applied to Iced. FA refreshes the Neomil Dashboard native
baseline; FB tests an independently measured scan cycle before FC/FD try
to render that fixed material on the existing maker lettering.

## Entropism Store title calibration

AH's width correction addressed the SVG specifically. Its old ordinary
`MAGNUM 650` span was 321 physical pixels, against 304 in the source;
the native span was already 299. The corrected SVG spans 303. Native keeps
one Rajdhani Medium text run and its separately reviewed subtitle baseline,
while the SVG uses distinct width/tracking settings. CQ's selected M is
also already integrated. These different settings reflect measured renderer
calibrations, not an unported font fix. Earlier native spacing and original
font-outline substitutions failed glyph/gap controls. E4 remains open for
contours and photographic printing; this audit supplies no native edit.

## Neomil baseline and independently measured cycle

A fresh packaged 4K Dashboard capture is byte- and RGBA-exact to EF's
accepted frame (`73fb4c13…`). The current 1600 matrix frame is also exact
(`c1bc933c…`). All 349 tracked crate files present at FA matched its guarded
checkpoint before these prose updates. The maker M and detached dot already have independently fitted
scan material; the old flat-material description is historical.

FB transfers no fitted microtext parameters. It compares the existing M
and dot red cycles with the four frozen EH word regions. Pitch is 1.984934
design units; origins 678.847428 and 712.591306 differ by exactly seventeen
cycles. Exact piecewise-linear means are 238.5 and 185.5625. Integrating each
cycle over the physical row footprint avoids point-sampling phase error.
The source/native chroma ratio uses the unchanged no-text SVG proxy ground
and fixed text-support threshold. Each word is centered once over its full
box with support-count weights; primary/tail subsets are not recentered
for the sign comparison.

| 4K word | Mean source/native gain | Primary correlation with M / dot |
| --- | ---: | ---: |
| PRECISION | .9472 | .723 / .706 |
| LIQUID | .9501 | .812 / .785 |
| POLYMER | 1.0390 | .436 / .410 |
| MUSCLE | 1.0607 | .372 / .353 |

All four full-word correlations are positive, strongest in the first line.
This supplies a phase constraint, not an amplitude or physical recipe:
the M and darker dot have substantially different contrast, first-line
mean gains are below one, and lower-line gains are above one. The proxy
ground is a model; off-support samples contain echoes. Smaller-size row
integrals are idealized box averages near/beyond Nyquist, not a simulation
of Lanczos sampling of glyph coverage times ink. Their mixed correlations
cannot substitute for actual SVG/native rendering. Astra independently
verifies 31 input hashes, 128 retained rows and 72 template/subset groups,
using a separate exact segment integration.

## Rendering-method controls

FC first replaces the entire microtext group with a white alpha mask and
constant original `#f93333` tint. All sixty uses, their geometry, ordering,
transforms and opacity remain. Despite this algebraic equivalence, the
extra compositing changes 3,935 / 789 / 717 pixels at 4K / 1600 / fractional
size, by at most 4 / 3 / 4 channel levels. The changes stay inside the text,
but the exact no-op prerequisite fails. No cycle candidate is rendered
through that route; the method is rejected without pixel patches.

FD moves the same solid stroke from the two definitions to their sixty
uses, preserving direct drawing. Its structural no-op is pixel-exact at all
three sizes. A second no-op exercises the actual repeating-gradient paint
path with all thirty-three stops held at the original color; all three
images are also exact. Thirteen gradient definitions compensate the existing
copy y translations, keeping one global phase across all sixty uses. Astra
independently checks the complete geometry/order/opacity subtree, every
phase compensation and all six exact images. No per-glyph mask or filter
is introduced.

The one variable candidate changes only those thirty-three stop colors to
`#f93333 × (Mred/238.5)`. Geometry, mean nominal ink, copy opacity and
background are fixed. Actual SVG renders change 1,970 / 416 / 382 pixels,
by at most four channel levels; alpha and all exterior pixels are unchanged.
Both complete lines and their existing copies receive the same globally
phased material. No parameter is fitted to the text.

| Word | 4K whole-box RMS | 1600 whole-box RMS | Fractional whole-box RMS |
| --- | ---: | ---: | ---: |
| PRECISION | 14.6934→14.6703 | 9.5400→9.5353 | 10.0755→10.1207 |
| LIQUID | 13.3912→13.3434 | 9.4238→9.3187 | 9.3043→9.2742 |
| POLYMER | 15.7241→15.7560 | 10.6403→10.5641 | 10.9032→10.9207 |
| MUSCLE | 16.1967→16.2547 | 11.4578→11.3601 | 12.0169→12.0717 |

**Reject the transfer.** Whole-word RGB L1 worsens in ten of twelve
word/size comparisons. RMS is mixed: first-line 4K and all four 1600 boxes
improve, but both lower-line 4K boxes and three fractional boxes worsen.
The lower-line 4K primary and tail regions both lose, as does fractional
PRECISION primary and tail. Fixed text-support comparisons confirm these
losses; the decision does not require every pixel to improve. The favorable
phase diagnostic alone is insufficient to prescribe material. Do not tune
a gain, phase or row lookup to rescue this candidate. No component or Rust
port follows; the existing cached text layer remains intact.

Astra verifies all sixty phase references, the thirty-three frozen color
values, unchanged candidate XML outside those colors, baseline provenance
and the frozen source-region scores. The worker independently reproduces
all 72 fixed-region L1 deltas. No production SVG, Rust, font, cache,
original image or golden changes. The prior round-sixty-seven runtime
verification remains current: 305 packaged Rust tests, 22 repository checks,
28 matrix cases and exact fresh packaged 4K parity. These new changes are
prose only, so the build and visual matrix are not rerun.

Evidence: `/tmp/cp-eras-next/ez-entropism-title-audit.md`,
`fa-neomil-primary-scan/`, `fb-neomil-scan-transfer/`,
`fc-neomil-micro-cycle/`, `fd-neomil-direct-cycle/` and `fd-root-review/`
under `/tmp/cp-eras-next/`. These studies preserve every source image and
font file. The twenty-nine broader scopes remain open.
