# Reference SVG round forty-five — CD–CH mechanism and component checks

Scoped Sol lanes investigate Entropism's selected Store M, Kitsch's
ordinary compliance M and Neomil's Dashboard footer edge. Astra checks
the source measurements and actual renders. The 318-file CC checkpoint
matches at entry. CF ports the accepted native footer correction after
source and state review, with production/package parity and repository
verification complete. CH synchronizes the Entropism selected component
title to the accepted parent and passes its full repository check. CH is
the latest completed checkpoint; its package is identical to CF.
All 29 broader TODO boxes remain open.

## Kitsch original-font replay

CD tests a separately named copy of the existing FreeSans Bold through
the existing text route. Independent raw-table comparison confirms that
only font names and the required head checksum change: the other 17
tables, all glyphs, advances, hint programs, weight 600 and timestamps
remain exact. M retains its 138-byte instruction stream. This is a
scratch font, not a new production asset or an identification of the
source photograph's font.

The actual Store harness changes only the face of the three ordinary
second compliance lines, including their existing feedback tables.
Startup checks both original and alias under Basic and Advanced shaping:
every glyph resolves to the expected font bytes at weight 600. The three
baseline captures exactly match BE; the three alias captures exactly
match those baselines in full RGBA at 3840×2160, 1600×900 and 1537×947.
Astra independently compares all six frames and checks all identity logs.
This establishes a font-route replay without the previous canvas-path or
clip-boundary changes. It does not accept a modified contour.

The frozen BS15 outline changes M's topology. Original phantom point
indices 13/14 become 15/16, while several instruction operands with the
same numbers are CVT indices and must not change. Central diagonal and
interpolation instructions also depend on the old points. Blind index
replacement or copying the original hint program would be incorrect.
The next controlled diagnostic removes only M's instructions in an
otherwise unchanged scratch font. It must pass original-outline replay
before any new outline is considered. The successful name-only replay
does not establish that changed hinting is suitable for production.

Evidence: `/tmp/cp-eras-next/cd-kitsch-font-replay/`, including
`root-font-audit.json`, `root-replay-review.json`, `captures.json` and
`hint-topology-note.md`. The first compile's harness mutability error is
fixed before the successful retry; its inputs and library are archived
separately. No shared renderer change is integrated.

CE performs that hint diagnostic. Independent table/glyph comparison
confirms unchanged M coordinates and metrics, 138→0 instruction bytes,
and exact raw glyph slices for all 2910 other glyphs. Three fresh
baselines still match BE. The first unhinted 4K replay changes 81 pixels:
80 inside the fixed M windows and one outside, at card 4 x3487/y1351
(41,95,82)→(33,77,67). Alpha stays exact. Astra reproduces those counts,
checks the shaped-font identity logs and views the three ordinary copies.
This fails unchanged-shape replay, so smaller unhinted frames and the
proposed BS15 contour are not rendered. Keep the original hinting.
The next read-only plan maps point operands for an unchanged-outline,
15-point control before attempting any topology change. Evidence:
`/tmp/cp-eras-next/ce-kitsch-unhinted-replay/`.

CG preserves the original polygon with two duplicate points, remapping
eight point-index operands while retaining CVT/function operands and all
138 instruction bytes. Astra independently checks the raw tables: only
`head` and `glyf` differ from CD, and all 2910 other glyphs remain exact.
Three fresh baselines and three original-outline controls match BE in
full RGBA at all three sizes, including pixels outside the fixed M windows.
Basic and Advanced shaping still resolve to the expected bytes at weight
600. This accepts the topology control; the proposed BS15 contour still
needs review of how the hints act on its changed geometry before rendering.
No production font changes. Evidence:
`/tmp/cp-eras-next/cg-kitsch-topology-replay/` and
`/tmp/cp-eras-next/cg-kitsch-topology-review/results.json`.

## Entropism selected M gap

The fixed 4K gap x1173:1179, y645:691 has no source dark pixels at
R,G<80/100, but the current SVG paints a 36-pixel stem at x1173. At
threshold 120 the source has two pixels elsewhere in the gap. An isolated
M reproduces the excess threshold pixels; an MA diagnostic reproduces
the gap's full RGB exactly. The selected A's horizontal bounds already
match. Astra reproduces all three gap counts and inspects the source,
full title and isolated glyph. Current title rows exactly match AH;
historical native captures are not asserted to be current.

One full-run, M-only clip replay preserves A, `650`, ordinary cards,
alpha and every pixel outside M. At 4K it changes ten internal antialias
pixels by at most two channel levels, with no threshold-mask changes;
the smaller replay frames are exact. One candidate trims M at source
extent x1173/y683. It changes 76/38/22 pixels and improves whole-M RGB
and overlap, removing the unsupported 4K gap stem.

Reject that exact trim: the 1600 gap loses two source pixels at threshold
100; the fractional gap loses two at 120, with right-stem and bottom-row
overlap regressions. Astra independently reproduces the regional RGB and
threshold metrics and views all nine comparison crops. The original
source, full-photo Lanczos 1600 sampling and uniform top-left Bicubic
1537×947 sampling are retained. The next investigation measures the
stem's changing position and weak lower-right coverage; a hard rectangular
clip does not preserve that coverage. Three height bands show a source
left stem increasingly farther left than SVG and a right core curving
toward its lower edge. Astra reproduces 36 source/SVG contrast centroids.
Per-window normalization establishes position evidence, not absolute ink
recovery or a finished contour. No title-contour or native change is made.
A further source-only join/foot audit confirms separated upper branches
at y653–656 and weak source feet at y683, where SVG remains dark through
y685. Astra reproduces 48 source/SVG rows and their threshold/contrast
measurements. Branch-combined centroids and near-background terminal
centroids do not establish a vector join or hard cutoff; one bounded
soft-terminal treatment still needs replay and small-size source controls.
Evidence: `/tmp/cp-eras-next/cg-entropism-m-source-constraints/root-review.json`.

Evidence: `/tmp/cp-eras-next/cd-entropism-ma-gap/`,
`/tmp/cp-eras-next/cd-entropism-ma-trial/` and
`/tmp/cp-eras-next/cd-root-entropism-review/` and
`/tmp/cp-eras-next/cd-entropism-m-profile/`.

## Neomil outer-left footer edge

The current native primary rim paints 4K x2901 red 175 in three straight
windows where the source is about 106; SVG is about 98. The later BT
inner strip affects x2904 and leaves this outer shoulder unchanged.
The x1209 design boundary and observed intensity agree with half coverage
under four-sample antialiasing and linear-light resolution. The exact
device sample locations are not asserted from that analytic model.

A bounded scratch trial represents the primary rectangle as a filled
ring, then moves only its straight outer-left boundary to x1209.1 over
y868.5..884.5. The inner boundary, corners, other sides, lettering and
echoes stay fixed. Three actual Dashboard baselines match BT exactly;
the three original-boundary ring replays also match in full RGBA. The
candidate changes exactly 38 pixels at x2901, y2084..2121. Every changed
pixel improves source RGB distance, totaling 2614 channel levels; the
straight shoulder becomes (128,23,23), versus (175,35,35) before and
source (106,24,23). Both smaller frames stay pixel-identical.

Astra reproduces all 57 regional RGB and 171 threshold comparisons,
with no losses, and inspects the source/baseline/candidate crop. Alpha,
inner coverage, corners, right side, text and all exterior pixels stay
exact. The observed edge change agrees with the four-sample model, while
faint striations and the remaining material mismatch stay open. Three
unchanged pixels near the joins remain bright in the fixed straight-window
samples; this correction does not finish the entire outer shoulder.

Six actual-Dashboard pairs pass: fresh rest, custom foreground, partial
opening, selection plus synthetic held, no-config fallback, and Kitsch
negative control. The five Neomil pairs each change the same 38-pixel
corridor; Kitsch changes none. Custom core ink is (51,238,221), while the
outer shoulder changes (35,175,162)→(23,127,118). Partial opening changes
the panel but preserves the exact rest footer delta. Selection/held and
fallback frames equal their corresponding rest frames; the held fixture
does not claim distinct visible feedback or live pointer replay. Astra
independently checks all twelve frames, logs and frozen inputs.

CF ports only the native primary footer primitive in `src/eras/neomil.rs`.
All SVGs, production fonts, shared rendering and goldens remain unchanged.
Three production sizes and packaged 4K match the reviewed candidate exactly.
Both Dashboard gates, all 294 local/Nix Rust tests, 24 SVG structural checks
and 22 repository checks pass. All 27 visual cases pass first attempt:
26 are exact and the existing Neo-kitsch bar differs by one level. All
319 frozen files match the tested 16 MB source; nine originals stay unchanged.
Evidence: `/tmp/cp-eras-next/cd-neomil-outer-left/` and
`/tmp/cp-eras-next/cd-neomil-outer-left-native/`, with independent results
in `/tmp/cp-eras-next/cd-root-neomil-review/`. State evidence is in
`/tmp/cp-eras-next/cd-neomil-outer-left-states/root-review.json`; integrated
checks are recorded under `/tmp/cp-eras-next/cf-validation-plan/`.

## CH selected Entropism component title

The component sheet still used the older single `MAGNUM 650` run despite
AH's accepted width and spacing correction in the parent Store trace.
CH copies the two accepted title nodes into that selected specimen.
The ordinary title, all paths, parent trace and runtime stay unchanged.

Astra confirms the exact two-node replacement, unchanged 35 component
paths, and changes confined to the title: 1296 pixels at 1920×1400 and
410 at 960×700, with no alpha or exterior changes. The full-size title
crop matches the parent in RGBA after the known integral translation.
This closes specimen drift; it does not resolve the M contour or identify
the source font. Evidence is in
`/tmp/cp-eras-next/cg-entropism-component-title/root-review.json`.
CH passes all 22 repository checks and all 27 visual cases on their first
attempt: 26 exact, plus the unchanged one-level Neo-kitsch bar pixel.
27 matrix outputs are reused from the cache; their current derivations
and output pixels are independently verified. All 319 frozen files match
the tested 16 MB source. All 24 SVGs have unique IDs and resolving local
references; all nine original images and 25 goldens stay unchanged.
The package store path is identical to CF, retaining its 294 local/Nix
test results, fresh packaged 4K parity and installed FreeFont notices.
CH does not claim another native capture. Evidence:
`/tmp/cp-eras-next/ch-validation-plan/`. Final prose preserves these
tested runtime/artwork bytes; changes remain staged and uncommitted.
