# Kitsch dashboard hover

The hub uses an inferred hover treatment: brighten an unselected blade's
front outline from `#a9e6df` at 1.4 design pixels to its existing label
mint `#7cffe5` at 1.8. Keep the fan stationary. The source photograph and
[rest trace](dashboard-trace.svg) establish the resting and selected art;
they do not show a pointer-hover frame. This rule adapts the era's emphasis
to the hub's existing five/seven-step trails, rather than treating those
resting trails as evidence for another hover step.

| State | Drawing |
| --- | --- |
| Unselected rest | Existing mint front outline, label, face and trails |
| Unselected hover | Brighter, thicker front outline only |
| Selected hover | Existing gold selected drawing |
| Held | Existing gold face and removal of only that blade's trail |
| Release on original blade | Existing selection/route action |
| Cancel | Existing selection and trail restoration; hover follows pointer |

All six blades use the same rule, including EVENTS when another blade is
selected. The era data reuse the original paths, rounded corners,
translations, labels and selected drawings. Shared painting, hit testing,
release/cancel logic and animation are unchanged. This is an instantaneous
state; it does not close animated transitions or live desktop verification.
The component sheet's generic one-ghost control example remains applicable
to those controls; the already extruded hub has this distinct rule.

Nineteen paired native cases pass at 3840×2160, 1600×900 and 1537×947.
All twelve hover comparisons change only the target front outline; rest
and representative selected/held/release/cancel materials are pixel-exact.
The 1600px rest also equals the existing golden. These screenshots use
synthetic material/backdrop selection through the production scene painter.
Separate unit tests replay pointer events for all six blades, including
release and cancellation before and after selection, and recursively
check that only one outline changes per blade. All 292 Rust tests pass.
Integrated production rest frames at all three sizes and hover frames for
all three orientations exactly match the reviewed candidates. The resting
trace and dashboard golden are unchanged. All 22 repository checks and
27 first-attempt visual cases pass. Live desktop interaction remains
unverified.
