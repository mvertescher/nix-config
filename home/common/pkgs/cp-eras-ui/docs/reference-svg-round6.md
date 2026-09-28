# Reference corrections, sixth batch — 2026-09-27

Two GPT-6 Sol lanes implemented the mailbox and footer corrections; a
Luna lane handled citation cleanup. Astra reviewed the code, measured
source/SVG/Iced crops and integrated the changes. Prior staged work is
preserved.

## Neo-kitsch selected mailbox printing

Selected subjects, senders and envelopes now use three distinct reference
inks. Native captures reproduce the configured colors; all 7,067 changed
pixels lie in the selected row's printing. Both the standalone and
published reference palettes are recognized, including their different
panel colors. Other variants and customized palettes keep semantic roles.
Explicit feedback overrides retain priority.

Selected rest, hover and held previews agree pixel for pixel. Other-row
held states retain open/closed glyph choices and the same distinct inks;
selection movement and custom-color fallback are covered by a regression
and headless previews. The preview's rest frame matches the production
mailbox exactly. Source font shapes, weight and veneer remain open under
NK-05/NK-07. See [ink measurements](neokitsch/mailbox-inks.md).

## Neomil dashboard footer

Each primary run has an independent height, horizontal fit and baseline.
SVG cap heights match the measured source while retaining narrow widths.
The native Iced cap heights improve but remain 0.42–0.84 design pixels
taller; its font contours are still heavier than the photo.

Two local translucent frame/text copies improve eight separate crop
comparisons. They use ordinary canvas primitives and the foreground role,
so they add no software-rendered cache image and follow custom colors.
The initial unsupported software-group implementation was rejected by
renderer tests and replaced before acceptance. Fine striations, softer
edges and repeated lettering remain open under NM1. See
[footer measurements](neomil/dashboard-footer.md).

## Kitsch source citations — K9

Corrected bracket, barcode and USER-box citations distinguish source mask
bounds, path control points and actual endpoints. Removed stale blanket
ink/stroke and implementation claims. Source stills, inferred interactions
and the original bar composition are explicitly distinguished.

Independent XML comparisons confirm the login/bar drawings are unchanged;
the component sheet retains its geometry, text placement and interaction
examples. Revised captions fit the rendered sheet. See
[source citations](kitsch/source-citations.md).

## Validation

The final Rust suite passes all 256 tests (211 library, 45 bar-window).
All 24 SVGs parse with unique IDs and resolved local references.
Native 3840×2160 and fractional 1537×947 captures were reviewed against
the original photographs and SVGs, with separate interaction/opening
previews. Local measurements establish improvement within the named
regions, not exact photographic fidelity.

G1i passes for the Neomil dashboard (83% of counted source shape area);
G2i passes for Neomil dashboard and Neo-kitsch mailbox (94% and 91% of
counted SVG shape area). These broad shape gates do not establish glyph,
ink or fine-printing accuracy. Three goldens are refreshed: Neo-kitsch
mailbox, Neomil dashboard and fallback dashboard. Pixel changes are
confined to selected-row printing and the footer; the opening dashboard
retains the same static footer. All 103 Rust/SVG/golden files match the
tested package's source snapshot byte for byte.

The full repository check passes all 22 checks. All 27 visual cases pass
on their first attempt at 100.000% similarity, including live mailbox/store
startup and fallback rendering. Evidence and logs are in
`/tmp/cp-eras-round6/`. Changes remain staged and uncommitted; live desktop
and IME validation remain separate.
