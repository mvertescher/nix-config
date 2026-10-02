# Reference correction round thirty-five — AX

AX restores the missing `store-ground` definition in the Neomil component
sheet. Its ordinary product card referenced the parent's ground through
`c1fill`, but the group was absent, leaving a flat red-brown background.
The fix copies the parent's twelve rectangles; the sheet already has
their gradients and masks. No parent trace or runtime artwork changes.

The production sheet matches the reviewed candidate at 1920×2000 and
960×1000. All changed pixels lie in the ordinary card; selected-card,
login-card and upper-row controls are unchanged. All 24 SVGs parse, have
unique IDs and resolve their local references.

After undoing the sheet's translation, the complete card subtree and all
66 reachable definitions match the parent. Three clean interior samples
match exactly. Of the 2,519 unequal pixels in the full rectangular crop,
2,493 are on or outside the card's stepped/chamfered boundary, where the
surrounding scenes differ. The remaining 26 are QR echo pixels with at
most a one-level RGB difference. This comparison supports the dependency
fix; it does not establish exact fidelity to the original photograph.

Two Sol lanes also prepare Neo-kitsch mailbox paragraph and dashboard
footer annotation fits. Astra accepts their bounded alignment gains
after comparing source, SVG and native captures at three sizes. All 150
mailbox whole-line overlap controls improve; dashboard whole-line R>100
overlap improves for all four lines at every size. Local gap, strict-ink
and fractional-size losses remain documented in the [text review](neokitsch/text-fit.md).
The mailbox component excerpt and its captions are synchronized.

Eight paired state captures preserve all pixels outside the affected
text: first/last selections, custom palettes and opening frames. The
dashboard first-selection case repeats its resting default; seven pairs
exercise nondefault states. Six baseline frames exactly match the
previously verified implementation. The production frames match the
reviewed candidates exactly at all three sizes, as do both fresh packaged
4K frames. Only the dashboard and mailbox goldens change, by 2,716 and
32,327 pixels respectively. Component-sheet changes stay inside the
intended card, paragraph and caption regions at two sizes.

All four affected fidelity gates, 292 local and Nix Rust tests, 24 SVG
reference/ID checks and all 22 repository checks pass. All 27 visual cases
pass on their first attempt; direct comparison finds 26 exact frames and
only the pre-existing one-level Neo-kitsch bar pixel. All 309 frozen
files match the Nix source, and eight original images remain unchanged.
Final status prose preserves the tested implementation, SVGs and goldens.
The component dependency follow-up closes; 29 broader requirements remain
open. Prior staged work is preserved; no commit, push or deployment.
