# Reference SVG correction, round twenty-eight — AQ

AQ continues the ordinary Entropism manufacturer labels and Kitsch store
compliance caps. These are local lettering corrections; the original font
outlines and photographic printing remain approximate.

## Entropism ordinary manufacturer labels

AP's selected label stays unchanged. Transferring its table to ordinary
cards failed source-gap controls. A new card-2-only table also fails the
third card: SVG gap ink rises from 40 to 73 pixels, including four tall
columns. Both candidates are rejected.

The accepted candidate uses one shared table constrained by both ordinary
source copies. It intersects their card-local dark-core supports and keeps
each glyph inside the intersection with a small raster-edge inset. This
is explicitly a two-card calibration, not a blind holdout. The wording,
Medium weight, fixed ink and common card geometry are preserved.

At 4K, whole-label source/native IoU improves .0202→.4426 and
.0131→.4564. Every fixed first/middle/last segment improves at thresholds
90, 100 and 120. Source-gap ink falls from 44/71 native pixels to zero.
All 3,063 changed native pixels lie in the two ordinary labels; selected
printing and the fourth-card cutoff remain exact. The native left edges
are one source pixel late, and the third card's right edge is two pixels
short. SVG overlap improves .0298→.4680 and .0480→.6682, with zero
measured gap intrusion on both ordinary copies.

At 1600×900 and 1537×947, 696 and 698 pixels change inside ordinary
labels only. Width and spacing improve; fine raster edges remain an
approximation at these small text sizes.

## Rejected Kitsch ordinary compliance experiment

Ordinary source lettering occupies 11–12 cap rows against 13 in the old
native rendering. A vertical factor of .885 preserves advances, tracking,
wording and ink. The SVG scales about its existing baselines; native line
one additionally moves up half a 4K pixel to balance the photographed
copies' different vertical phases.

All 36 first/middle/last comparisons across the two complete ordinary
cards improve at green thresholds 100, 140 and 180. The fourth card's
visible first/middle regions also improve; its clipped ends remain exact.
Word-gap ink improves or ties, and blank bands gain no ink. All 14,951
changed native pixels lie in ordinary compliance text. Selected printing,
footer, brands and certification marks remain unchanged. Some cap bounds
still differ by a source pixel.

Selected text needs separate treatment. The ordinary factor, a one-pixel
phase correction, and a selected source-derived height/phase trial all
lose independent glyph controls at one or more thresholds. They are
rejected; selected compliance fidelity remains open.

The scratch `Prim::TrackedHeight` supplies the vertical fit without changing horizontal
layout or selection geometry. Nonuniform canvas text otherwise extracts
outlines repeatedly: a naive trial raises warmed 4K `Scene.draw` from
24.45 to 66.67 ms. A separate 64-entry glyph cache, cleared after each draw,
reduces the fitted trial to 27.42 ms with pixel-identical output. This
retains an approximately 3 ms cost versus the former text. Cache keys
include glyph, font and physical size; colors and transforms remain live.
Raster fallback uses the existing text path. The earlier title cache is
unchanged, and no font state survives a draw.

Small-size review rejects this implementation. At 1600×900, the existing
card-1 native caps already match the source's five rows. The candidate adds
a first-line fringe and moves line 2 down one pixel. With local contrast
normalization, line-2 IoU falls .7000→.6126, .6285→.5190 and .5096→.4266
at 35/50/65%; line 1 also loses at all three levels. Card 3 and both
1537×947 controls improve, so the failure is scale/phase-specific rather
than a uniform baseline error. Thirteen state pairs correctly constrain
changes to the intended text, but cannot establish source fidelity.

The Kitsch data, SVGs and shared renderer are restored to AP. The cache
experiment and its tests remain in scratch, not in the staged implementation.
A follow-up must preserve small-text rasterization while fitting larger
caps. The owning task stays open.

## Verification

Fresh baseline previews match AP's packaged 4K images exactly. All 24 SVGs
parse with unique IDs. The provisional experiment passes 292 Rust tests after
correcting a new test's assumption that a space must return no path objects;
Iced returns an empty path. The corrected assertion checks that the path
has no drawing commands. Cache tests compare fresh and reused outlines
across glyphs, fonts and sizes, and exercise bounded size churn.

The two source gates pass. All fourteen Entropism state pairs confine
changes to ordinary manufacturer text, with fourteen within-variant
fixed-ink control groups passing. The early opening pair is exact. Native
small-size source controls also improve or tie in every first/middle/last
comparison at thresholds 90/100/120, with crops excluding dark band edges.
Production images at 1600×900 and 3840×2160 match the reviewed candidate
exactly. The implementation gate passes. The final implementation passes
290 Rust tests; only the Entropism store golden changes (696 pixels).
All 30 fixed-ink crop comparisons pass, and parent/component label groups
match exactly. All 22 repository checks and all 27 visual cases pass on
their first attempt. The Nix package passes 290 tests without retry.
All 294 frozen files match the tested Nix source; five original images
remain unchanged. A fresh packaged 4K frame exactly matches the reviewed
ordinary-label candidate. Direct visual comparisons retain only the
pre-existing one-level Neo-kitsch bar pixel at (749,20).

The ordinary manufacturer width/weight follow-up is verified. Broad
Entropism fine lettering and the rejected Kitsch height/SC work remain
open. All changes remain staged and uncommitted; nothing is deployed.
