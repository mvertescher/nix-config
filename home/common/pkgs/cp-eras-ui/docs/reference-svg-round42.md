# Reference typography correction — round forty-two (BH–BN)

This round advances two bounded font fits into native review and integrates
the accepted Neo-kitsch Store correction in BM. Login remains a scratch
study. All 315 frozen BF files matched at entry; the 29 broader TODO boxes
remain open. Integrated verification is recorded below.

## Neo-kitsch store metadata

BG's analytic word-space model did not predict actual librsvg placement.
With negative tracking, splitting a sentence into zero-dx tspans itself adds
advance at each boundary. BH's literal-space trial therefore double-counts
part of the intended spacing and worsens all pair/line RGB comparisons
against its zero-dx control. BI measures the boundary effect using only
plain renderer fixtures, independent of the photograph: 1.076935 native
pixels per boundary. Subtracting it from the intended +.760234 design-pixel
space addition gives an SVG-only dx of +.311511. Native spacing must keep
the intended advance, not copy that librsvg compensation.

The single compensated SVG improves every pair/line RGB and threshold-F1
comparison against the original at all three sizes. All 594 independently
recomputed word/variant/threshold comparisons agree with the frozen audit;
there are no word-F1 losses against the original. Some lines and words still
lose against rejected intermediate controls. R>190 false ink inside fixed
source gap apertures is 127/5/5 pixels versus the original 10/0/0, but all
27 apertures retain a clear column at every tested threshold and size. Upper
first-line cap phase, second-line endpoints and O-counter ink remain imperfect.

BJ tests the intended spacing through the actual `Store` widget. Its three
baseline captures reproduce frozen production RGBA exactly. The candidate
changes exactly six metadata runs, uses the embedded FreeSans Bold selector,
and derives word positions from one native shaper model. It changes
15018/3776/3516 pixels at 4K/1600/1537, with no alpha or exterior changes.
All pair/line/word threshold-F1 comparisons improve or tie. Upper A and lower
B RGB errors improve, but upper C pair error worsens
26.487→26.678 / 20.133→20.179 / 23.130→23.485. C2 loses at all sizes and C1
at fractional size. Every local word loss is retained in the audit.

The upper native first glyphs are one pixel right of the source at 4K;
upper second-line ink occupies rows 362–371 versus source 361–370. A single
BK calibration is assigned: move both upper blocks left by 1/2.4 design
pixel and only their second lines up by 1/2.4. Lower B and all font/spacing
parameters stay fixed. This uses A as training and C/internal words as
holdouts. Native fixed source-gap apertures also have small-size ink losses;
those apertures alone do not establish whether the actual moved word spaces
are closed. The trial is not yet accepted for production.

BK confirms the 4K/1600 gains but moves the fractional second-line baseline
across a pixel-rounding boundary, worsening C2 RGB error. The renderer's
rounding equation gives one shared offset interval that preserves the 4K
correction and the smaller-size baseline. BL uses −.25 design pixel for
the two upper second lines, with BK's x adjustment unchanged. Its actual
4K/1600 frames are identical to BK; only 1001 pixels in the two fractional
second lines change. All pair/line RGB and pair/line/word threshold-F1
comparisons now improve or tie against production at all three sizes.

BL still has 2/2/5 individual-word RGB losses at 4K/1600/1537. R>190 false
ink in source gap apertures is 235/26/16 versus production 203/0/0, while
all 27 apertures retain a clear column at all three thresholds and sizes.
The 4K O2 source-counter rectangle contains 23 bright pixels versus source
14; this counts ink in a fixed rectangle, not total hole area. Fractional
second-line spans are 145–148 versus source 144–148: the bottom matches,
but the top row is still missing. The 1600 cap residual also remains.
Astra reproduces 2160 word/gap/variant comparisons and the full-RGBA
locality. BL is accepted as a bounded rest fit and proceeds to state review.

Seven actual Store baseline/candidate pairs at 1537×947 then cover rest,
first and last selections, ordinary and selected held-material fixtures,
custom palette and partial opening. Both rest frames reproduce BL exactly.
Every pair changes the same 3524 metadata pixels, with no exterior or alpha
changes and the same signed RGBA delta. The selected-held fixture equals
rest because both use the intended `GROWN` material. Held fixtures are
synthetic material substitutions, not pointer-event replay.

BM integrates the six runs with a `TrackedWords` primitive that retains
each complete literal sentence. One shaper model supplies letter and word
advances; leading, repeated and trailing spaces retain their advances, and
anchors apply to the whole sentence. Native extra space remains +.760234;
the SVG uses its separate +.311511 boundary compensation. No other text,
source images or component artwork changes. The exact source font and the
listed word, gap, counter and cap residuals remain open.

## Neomil inactive login notices

BH's FreeSans trial improves the trained first line but worsens three whole
4K holdout lines. BI independently registers the repeated source notices:
interior-word masks select +694 native pixels at all four thresholds on
both lines. Astra reproduces all 84 source-mask comparisons. The source's
second line spans 380 pixels in both copies, versus BH's 383.

One BJ trial transfers the +694 copy spacing and fits a separate line-two
origin/tracking to the repeated 380-pixel span using actual plain renders.
It improves all twelve whole-line RGB comparisons against the original SVG.
The three previously failing 4K lines improve to 13.111, 15.252 and 12.610
MAE, versus original 16.676, 17.021 and 18.923. Both second-line endpoints
match the measured source spans. This is a geometry fit, not font identification.

Fixed local losses remain substantial: 40/29/37 regional RGB-MAE losses and
85/44/36 threshold-F1 losses at 4K/1600/1537. Card 3's first-word MAE worsens
22.651→34.127, and its first O paints 17 of 51 source-hole pixels versus
the original five. Independent severity review confirms all 30 word gaps
retain a clear column at all four thresholds and sizes; both 4K initial O
glyphs retain one connected contour and one enclosed counter. Card 3's
R>155 counter has 42 pixels versus source 51. Its two-pixel horizontal
phase explains part of the fixed-aperture loss but does not waive it.
Some smaller-size gaps have only one clear pixel.

These whole-line gains and retained separators justify preparing a bounded
native trial, with all local losses explicit. Login needs a per-legend face
override shared by measurement, drawing and echo geometry. The active notice,
input and AJ echo must preserve pixels. Font-table inspection corrects a
preparatory assumption: FreeSans has hhea ascent .9, descent −.2 **and .1
line gap**; a 1.2-em line with centered leading uses a .95-em baseline
offset. A .9-offset/1.2-line combination would misplace it. Native behavior
and responsive containment remain unverified. Uniform fractional SVG/source
coordinates must not substitute for Login's responsive native registration.

BK then compiles an isolated copy of the actual Login renderer with the
optional face and shared metrics. All three baseline replays match freshly
captured production RGBA exactly; no production files change. The candidate
changes 16815/3865/3987 pixels only in inactive notices, with unchanged alpha
and active AJ artwork. Canonical source controls expose a native phase error:
first-line R>155 ink spans [1388,1400) versus source [1389,1401) at 4K, and
[578,583) versus [579,584) at 1600. Card-3 first-line RGB MAE worsens
17.304→19.757 at 4K and 12.417→21.157 at 1600; three 1600 lines regress.

The analytical top offset and the glyph renderer round separate quantities.
A BL scratch follow-up therefore applies independently rounded baseline
placement only to the new unrotated, unstretched FreeSans glyph path,
preserving legacy legends and vector/echo placement. This is a native
rounding correction, with the SVG fit and all table values frozen. At 1537
only locality is established; no unregistered source-F1 claim is made.
Changed R>130 candidate
pixels have only about .51/.13 output pixel of right clearance. That mask
excludes pixels equal to the old rendering, so it establishes changed-ink
locality, not full candidate-notice containment. Full bounds remain to verify.

BL's three baseline replays exactly match BK's candidates. The rounded
first-line rows now match the source at 4K/1600, and all four 4K whole-line
RGB errors improve against original production. Three 1600 lines still
worsen: card 2 lines one/two 13.030→13.186 and 13.835→14.431, and card 3
line one 12.417→15.609 MAE. All eight whole-line R>155 F1 scores improve,
but fixed local losses remain: 41/46 regional MAE, 41/46 RMS and 98/75
threshold-F1 losses at 4K/1600. The initial O paints 18/48 and 19/51
source-hole pixels at 4K, versus production 8/48 and 5/51.

Astra independently reproduces all 1944 regional metric comparisons and
full-RGBA locality. BL changes 16329/3529/3987 pixels at 4K/1600/1537, only
within inactive notices, with unchanged alpha and active AJ artwork. BL and
BK are identical at fractional size. Login is not accepted for production;
native spacing, glyph-shape losses and complete responsive notice bounds
need further investigation. Its production SVG, renderer and golden stay
unchanged.

## Integrated BM verification

All 294 local and Nix Rust tests pass, including the literal-space and
whole-line-anchor behavior test. Production frames match BL in full RGBA
at all three sizes, and a fresh packaged 4K Store capture matches exactly.
Both Store fidelity gates and all 24 SVG ID/reference checks pass. Only
the Neo-kitsch Store golden changes, by 3787 metadata pixels with unchanged
alpha.

All 22 repository checks and all 27 visual cases pass on their first
attempt. Direct comparison finds 26 exact images and the same pre-existing
one-pixel, one-channel-level Neo-kitsch bar difference. All 316 frozen
tracked files match the tested 16 MB Nix source, nine original images are
unchanged, and packaged FreeFont notices match their pinned source. Final
prose updates preserve that tested runtime, artwork and goldens. Changes
remain staged and uncommitted; no push or switch.

## Read-only continuation studies

BM independently verifies all 35 fixed Login gap control boxes at four
thresholds and both canonical sizes: each retains a clear column, despite
additional fringe ink. Full R>155 notice bounds at 1537×947, without a
changed-pixel mask, are [672,609,908,624) and [950,609,1186,624). These show
threshold-defined rest containment only; subthreshold ink and other layouts
and states still need review. Lower thresholds also include a decorative
frame column that must not be mistaken for text overflow.

BN's source-owned CLASS/TO slices expose repeated glyph and raster-profile
losses. The source CLASS has four internal clear-column runs; BL has two
at 4K and two/one at 1600. These slices can include neighboring candidate
glyphs, so their areas do not identify a font weight. The terminal O remains
closed when the existing source-line fringe includes its shifted right
edge: BL has a 40-pixel hole at 4K versus source 51/53. A crop clipped at
the old word endpoint would falsely report no counter. Astra reproduces
128 word profiles and those topology counts; canonical fixed-box losses
remain unchanged. No scalar weight/ink retuning or new native candidate
is justified from these findings alone.

A separate Entropism T2 audit localizes AU's rejected fractional contour
loss to the foot. AU's original local-Lanczos control stays authoritative
(.745→.685 whole-digit IoU, .900→.400 foot). Uniform inverse-affine Bicubic
around the same responsive anchor gives .786→.649 and .940→.383; this is
a resampling sensitivity check, not a moved control or new acceptance
score. Astra independently reproduces both methods. Both show the rejected
foot missing most of the leading row and the bottom row. A foot-local
edge-profile hypothesis is recorded with strict-core, sweep, gap and
three-size controls; no new contour is integrated. See
[the badge record](entropism/mailbox-badges.md).

## Evidence and scope

All source comparisons use the original 4K photograph, full-photo Lanczos at
1600×900 and uniform top-left inverse-affine Bicubic at 1537×947. Full RGBA,
word/gap/counter losses and backend distinctions are retained. Native Login
uses only the registered 4K/1600 source comparisons; its fractional capture
is reviewed separately in responsive coordinates.

Evidence under `/tmp/cp-eras-next/`: `bh-neokitsch-spacing/`,
`bi-neokitsch-boundaries/`, `bi-neokitsch-compensated/`,
`bj-neokitsch-native/`, `bj-neokitsch-native-review/`, `bh-neomil-login/`,
`bi-login-registration/` and `bj-neomil-login/` (including `severity/`).
Native follow-ups are in `bk-neokitsch-native/`,
`bk-neokitsch-native-review/`, `bl-neokitsch-native/`,
`bl-neokitsch-native-review/`, `bk-neomil-login-native/` and
`bk-neomil-login-review/`, `bl-neokitsch-states/`, `bl-neokitsch-port/`,
`bl-neomil-login-native/`, `bl-neomil-login-review/` and
`bm-validation-plan/`, `bm-login-diagnosis/`, `bn-login-glyph-plan/` and
`bn-entropism-foot/`.
Astra independently reproduces the fixed regional measurements, SVG word
scores, source registration, Login separators/counters and native locality.
Detailed continuation records: [metadata](neokitsch/text-fit.md) and
[login notices](neomil/login-primary.md).
