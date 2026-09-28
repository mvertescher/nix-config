# Eighth reference checkpoint — 2026-09-28

The K1 checkpoint passes 267 Rust tests and all 22 repository checks. All
27 screenshot cases pass on their first attempt. Twenty-five cases match
100.000%; the frozen/live Neomil store cases match 99.998%. Their only
Cargo/Nix difference is 170 pixels in the small margin `益荒男` run: two
available fallback fonts draw different glyphs. Source-vector replacement
is assigned for the next batch; this is not accepted as deterministic
printing. No thresholds changed. Seven reviewed goldens are refreshed.

Fourteen affected fidelity gates pass: twelve K/K1 gates plus the unchanged
Entropism store's two J gates. The other references retain the H checks.
All 24 reference/component/bar SVGs parse with unique IDs. Shape gates are
broad inventory checks, not detail-completeness scores.

The accepted changes include Neomil inactive-login card/badge fields and
frame inks, source-store logo and weapon contours, restored store printing,
selection-aware broad card/navigation fields, Entropism inverse weapon
seams, Kitsch rifle/socket geometry and fitted printing, and Neo-kitsch
weapon contours plus visible store grain/frame-ink repairs. Entropism's
SVG captions no longer inherit an unwanted text stroke. Remaining local
printing, glyph, baseline, certification and veneer-flow defects remain
in the owning TODOs.

The 135 held-out store material patches have source/native RGB RMS .52–.81;
SVG/native maximum channel difference is one. Native Neomil logo primary
mask IoU is .972, and ordinary/selected rifle masks are .879/.950. Neo-kitsch
rifle overlap is .821, replacing the old block silhouette. These scores use
explicit local primary masks; faint photographic echoes are separate.
See [store material](neomil/store-material.md),
[source artwork](neomil/store-primary-art.md),
[Kitsch artwork](kitsch/store-art.md) and
[printing fits](kitsch/typography-fit.md).

All 18 fractional store captures are reviewed. The production app and
rest preview are pixel-identical. Selected, hover/held, navigation, closed
and early-opening differences remain within their intended regions;
card4 keeps its permanent cut. Other eras' held fourth cards retain their
clipping, and a custom palette uses the semantic scene. Tests cover all 20
category/card background mappings, hit/navigation geometry, palette
provenance and material pixel samples.

Review rejected and corrected genuine errors along the way: stale
feedback recoloring after new text/art primitives, a grain layer painted
under its fill, dark preblended frame echoes over the blue ground, and
store ramps mistakenly specified in pixels instead of bounding-box
fractions. The rejected versions are not golden references.

K1 is the immutable reviewed checkpoint. Subsequent mailbox, typography,
socket and maker-printing edits need their own native review and checks.
All work remains uncommitted; live desktop/session verification is separate.
