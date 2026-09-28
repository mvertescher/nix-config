# Reference corrections, fifth batch — 2026-09-27

Three GPT-6 Sol implementation lanes handled scoped corrections, with
Astra integration, source review and independent Rust review. Previous
staged work remains intact.

## Entropism mailbox paragraph and badge geometry

The third paragraph starts at y535 rather than y533.2. Optional paragraph
origins preserve the existing pitch for missing entries and do not shift
later paragraphs. The first eight lines and all words/breaks are unchanged.
A regression checks explicit origins, partial fallback and other-era
paragraph positions.

Per-badge text geometry restores the source widths and centers in SVG,
the component excerpt and Iced. At native 3840×2160, three runtime labels
match the measured source horizontal bounds exactly; T4's right edge
differs by one native pixel (0.417 design px). Third-paragraph mean
SVG-to-Iced vertical edge error falls from 1.46 to 0.42 design px.
Font contours, vertical residuals and action-label widths remain open in
E2. See [measurements](entropism/mailbox-layout.md).

## Kitsch store lower corners

Plain cards and the selected detail body use a source-supported 10.5px
quadratic corner span. It is not an exact circular radius. The plain
bottom rule moves down 0.5px with its socket dividers. Existing SVG/Iced
half-pixel offsets are documented, not silently treated as exact agreement.

Native right-edge profile error falls from 0.93 to 0.67 design px on the
plain card and from 1.13 to 0.42px on the selected card. Gun placement,
card contents, selection-aware hit regions and the fourth-card fade remain
intact. K8 stays open for artwork, socket scatter and certification marks.
See [corner measurements](kitsch/store-corners.md).

## Neo-kitsch component and provenance sync — NK-12

The sheet now carries the current dashboard's EMAIL/panel veneer, five-line
caption text at the parent coordinates, and two-line tape. Its six-plus-two
body copy was already current. The corrected excerpts retain the current
trace's material approximations; NK-05/NK-07/NK-11 remain open for source
typography, veneer and ground fitting.

Source descriptions distinguish traced examples, inferred interaction
states and the original bar design. Old claims that the bar implementation
had not followed its design are corrected. The bar's SVG drawing and the
sheet's interaction band are unchanged. See
[excerpt checks](neokitsch/component-sync.md).

## Validation

All 255 Rust tests pass (210 library and 45 bar-window). All 24 SVGs parse
with unique IDs and resolved local references. Independent Astra review
found no substantive issue in layout fallback or store interactions.

Native original/SVG/Iced crops and fractional 1537×947 captures were
reviewed. Store previews cover all four selected cards, plain hover/held,
selected fourth-card hover/held and the opening frame. Mailbox opening
and paragraph/badge crops were reviewed. At native size, runtime pixel
changes occur only in the intended badge/third-paragraph and card-foot
regions. Fourth-card feedback changes no pixels beyond the fade endpoint.

Only the Entropism mailbox and Kitsch store goldens are refreshed in this
batch. Both G1i and G2i gates pass on both screens: Entropism matches
88% of photographed shape area in G1i and 100% of SVG shape area in G2i.
Kitsch passes G1i through 0.75 ink placement and matches 97% of SVG shape
area in G2i. These broad checks do not replace the local source comparisons.
The full repository check passes all 22 checks. All 27 visual cases pass
on the first attempt at 100.000% similarity to their goldens. Changed
Rust, SVG and golden files match the tested package's source snapshot
byte for byte. Evidence and logs are in `/tmp/cp-eras-round5/`.

Changes remain staged and uncommitted. Live desktop and IME verification
remain separate.
