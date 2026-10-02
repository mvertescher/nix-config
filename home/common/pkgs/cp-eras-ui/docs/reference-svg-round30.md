# Reference SVG review, round thirty — AS

AS investigates Kitsch compliance caps and Neo-kitsch selected-store grain.
Neither candidate passes source-fidelity review, so production code, SVGs,
component excerpts and goldens remain at the verified AR checkpoint.

Kitsch's shared height/edge-response fit predicts row profiles but leaves
individual glyph contours wrong. An isolated source-shaped O improves
masks while colliding with the unchanged N. Narrowing it restores the gap
but regresses strict small-size detail. Both variants remain rejected;
a bounded O/N prefix with fixed suffix anchor and total advance is the next
local task. See [the typography study](kitsch/typography-fit.md#as-cap-geometry-registration-and-edge-response).

Neo-kitsch's candidate preserves the QR area and replaces only the three
text-socket stripes. Corrected native previews stay within that band at
4K, 1600×900 and 1537×947. Aggregate directions and RGB error improve,
but the first socket gains an unsupported sparse wedge and the fan loses
the source's tight bends and upper-right contrast. It is rejected. The
scratch clipping error and corrected comparison are recorded in
[the veneer study](neokitsch/veneer-fit.md#as-native-decision-reject-the-three-socket-trial).

The checklist audit converts seven recurring acceptance rules and one
explicit do-not-fix caution into standing bullets. All requirements remain;
the count changes from 37 to 29 without claiming eight completed tasks.
The open items include overlapping fidelity umbrellas, toolkit work needing
real callers/designs, source-material prerequisites and live checks. A
read-only desktop preflight still finds no active graphical user session.
See [the completion ledger](../todo/completion.md#as-checklist-accounting).

AS verifies baseline native parity at three sizes and frozen runtime,
reference, golden and original-image hashes. No new runtime tests, fidelity
gates or full repository check are claimed for this documentation-only
round. AR remains the latest full validation: 290 Rust tests, 22 repository
checks and 27 first-attempt visual cases. Changes remain staged; nothing
is committed, pushed or deployed.
