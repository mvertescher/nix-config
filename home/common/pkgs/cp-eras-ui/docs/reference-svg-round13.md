# Thirteenth reference checkpoint — AB, 2026-09-29

AB corrects four bounded source/implementation defects: Neomil's small
margin word and rule, Kitsch's SC badge apertures and native shelf-brand
runs, and Neo-kitsch's native mailbox action labels. All 286 Rust tests
and the release build pass. All 24 SVGs parse with unique IDs, and all
three affected source gates pass. All three implementation gates pass
at their existing thresholds. Twenty-six native captures are reviewed;
the full repository check passes all 22 checks. All 27 visual cases match
100.000% on their first attempt, and all 199 frozen file hashes agree with
the Nix source. Changes remain staged and uncommitted.

## Neomil margin word

The old font word overlaps the accepted CJK; a long rule crosses the
source's intended word location. Six compact stencil contours and a short
slash replace them. After discarding zero-area subpaths, the glyphs use
20 rings and 181 vertices. The CJK definition is unchanged. Word, slash
and hatch use semantic foreground ink; the dark O ring uses background
ink. The hatch has constant .67 opacity, with no new renderer primitive.

In the native capture, per-glyph source RGB errors fall from
31.19/29.13/28.32/25.83/27.11/36.41 to
15.22/15.81/16.54/15.43/16.40/24.43. CJK error falls 25.15→10.21 because
old overprinting is removed, and the slash falls 30.49→8.11. All 3,054
changed 4K pixels are confined to the word/rule region.

The O/plaque remains a material limitation: native RGB error falls
32.06→25.55, but bright intrusions into source-dark gaps rise 3→55.
The second-A/plaque junction is still partly merged; several small glyph
gaps also remain over-inked. This establishes a placement/primary-contour
improvement, not an exact word or photographic-material match. See
[the printing record](neomil/store-printing.md).

The added opacity group exposed an old assumption that every top-level
store motion was the shelf clip. Material replacement now selects clipping
motions and preserves independent opacity artwork. A regression exercises
that preservation, and the existing shared-opening test now explicitly
selects the clipping motion it intends to compare.

## Kitsch SC badge

Four compact corner apertures restore repeated source topology without
moving the square, disc or letters. SVG uses even-odd holes; Iced paints
in the existing disc/band ink, including selected variants. Card 1 guides
the fit and the other three copies check it independently.

The SVG changes only 178 corner pixels and improves all 16 fixed corner
patches. Native improves 15 of 16 and each card's combined score. Native
card-4 lower-right error worsens 20.06→22.63; its photographed aperture is
softer than the shared sharp contour. This limit remains explicit rather
than being hidden by the overall gain. See
[the certification record](kitsch/store-art.md).

## Kitsch native shelf brands

The SVG already has broad bold PETROCHEM / BETTERLIFE TEC runs, but Iced
retained narrow size-8 text and a Regular lower line. The source and SVG
PETROCHEM core ends 39 pixels after the previous native run on all three
visible copies. Independent native widths/baselines now match the source
endpoints within one or two pixels, preserving ordinary/selected ink.

Top source/native F1 improves .382→.729, .392→.818 and .375→.684; lower
F1 improves .076→.311, .148→.427 and .106→.394. All first/middle/last
segments improve at the central thresholds. The strictest ordinary lower
ink threshold still fails, reflecting the sharp native ink versus the
photograph. Fresh production pixels match the approved native trial exactly
in all three brand regions. The SVG is unchanged. See
[the typography record](kitsch/typography-fit.md).

## Neo-kitsch mailbox actions

Four native RIFLES labels still used 15px Regular versus the SVG's wider
16px Medium runs. Native widths were 92–93 pixels versus source 109.
The fitted native Medium 16.75 runs preserve button frames and use local
origins to account for the source's small nonuniform spacing. No pointer
behavior exists on these action drawings; this adds none.

At the central gold threshold, source/native F1 improves
.217→.826, .215→.756, .170→.796 and .263→.763. All 36
first/middle/last controls across three thresholds improve. All 6,366
trial changes are inside label bounds, and the full production4K frame
matches that trial byte-for-byte. Source bloom and exact glyph
contours remain separate. See
[the mailbox typography record](neokitsch/mailbox-typography.md).

## Native state and integration checks

Twenty-six captures cover production4K/1600, fractional rest, ordinary and
selected held cards, last-card selection/clipping, custom palettes and early
opening. Mailbox row-held/custom/opening states preserve action drawing.
All matched changes are confined to the intended regions: Neomil store
3054/699 pixels at4K/1600, Kitsch store9198/1953, Neo-kitsch mailbox6366/1561.
The three reviewed1600 goldens are refreshed; no other golden changes in AB.
Source gates pass (Neomil72%shape area, Kitsch.84ink placement, Neo-kitsch.72),
and implementation gates pass at89%/91%/97%shape area respectively.
Thresholds are unchanged. Full repository verification passes all 22 checks,
including 27 exact visual cases on their first attempt. The next local
margin-plaque boundary correction is separate from this frozen AB result.

## Remaining work

The Entropism primary-content recheck finds no new omission; E2/E4 remain
scoped to exact lettering and fine printing. Other fine material work,
missing original assets, design-dependent interaction rules and live
input/presentation checks remain open. Neither shape gates nor matching
goldens close those source and desktop limitations.
