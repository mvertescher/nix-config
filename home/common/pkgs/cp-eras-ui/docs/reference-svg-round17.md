# Seventeenth reference checkpoint — AF, 2026-09-30

Three Sol medium lanes investigated Entropism store headings, Neo-kitsch
frame printing and Neomil ordinary cartridge ribs. Astra reviewed the
source, SVG and native evidence. The staged AD/AE changes are preserved;
this round remains uncommitted.

## Entropism product-heading baselines

The native MAGNUM 650 and HAND GUN runs rise 1.5 design pixels in both
ordinary and selected cards. At 4K, title caps move from y652–688 to
y648–684, against photographed y649–683. Subtitle caps move from
y702–733 to y699–730, against source y699/700–729. Size, spacing, width,
weight, ink and content are unchanged. The SVG and component examples
move only the subtitle baseline 45→44.5; the title's half-pixel lift is
rejected because it worsens first/last glyph segments on every card.

All 24 whole-run native comparisons improve across four cards, two runs
and three thresholds. Independent first/middle/last segments improve in
67 of 72 comparisons. At the middle threshold their mean IoU improves
.174→.260, with 23 of 24 segments improving. The selected subtitle's
first segment loses overlap at every threshold (.091→.069 at the middle
threshold); its middle segment also loses at the low threshold, and the
fourth title's first segment loses at the high threshold. Those contour
tradeoffs remain explicit. Exact glyph forms, SVG title width and
photographic softness remain open. See [the measurements](entropism/store-typography.md).

Only heading pixels change: 20,591 at 4K and 4,482 at 1600×900. Seven
paired fractional states cover rest, ordinary/selected/last-card press,
last-card selection, custom colors and opening. All changes stay within
the headings; frames, weapon/value/socket art and clipping remain
pixel-identical. The early opening capture is entirely unchanged. Custom
selection ink and reverse-video behavior remain intact.

## Rejected printing trials

The Neo-kitsch fifth lower frame echo was narrowed and given independent
ordinary/selected opacity in a scratch native preview. Its straight lower
ridge approaches the source, but the selected side becomes too bright
(red contrast 45.4 versus source 32.4), and the split introduces an
unsupported late strength step. Upper and gap controls remain unchanged.
Moving the split to the source crossing alone would retain the selected
side error. No frame code or SVG is changed; [NK-14 remains open](neokitsch/store-frame.md).

Neomil ordinary cartridge ribs retain their accepted geometry and ink.
Narrower SVG strokes worsen three of four full-strip holdouts; wider
strokes trade improvements on earlier rows for losses on later rows.
Independent native core/edge/gap samples show that width alone cannot
resolve the photographed variation. No cartridge code or SVG is changed.
See [the four-row controls](neomil/mailbox-fidelity.md).

## Integration

All 286 Rust tests and the release build pass. The approved native result
passes local and fractional-state review. Only the Entropism store golden
is refreshed. The final source and implementation gates pass at 89% and
91% matched shape area, with unchanged thresholds. All 24 SVGs parse with
unique IDs. The full repository check passes all 22 checks; all 27 visual
cases match 100.000% on their first attempt. All 216 frozen source,
documentation, script, test and TODO hashes match the tested Nix snapshot.

Evidence is under `/tmp/cp-eras-next/af/`, including `check-verification.json`,
`state-review.json`, the heading segment measurements and both rejected
printing trials. Final prose records these results; runtime, SVGs, scripts,
tests and goldens remain unchanged from the verified snapshot. Changes
remain staged and uncommitted. Live desktop checks remain separate: AF
preflight found no graphical user session, and the prepared demo launcher
has not been run.
