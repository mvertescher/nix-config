# Fifteenth reference checkpoint — AD, 2026-09-29

AD accepts two bounded Neomil corrections. Sol implementation lanes
investigated the store margin, Kitsch SC printing and Neo-kitsch veneer;
Astra reviewed their source/SVG/native evidence and calibrated the native
dashboard captions. The Kitsch and Neo-kitsch proposals are rejected.

## Accepted source corrections

The Neomil store plaque uses wider hatch ink only between local x89 and
x99. The accepted leading slant, second-A junction, six primary glyphs,
O ring, slash and CJK retain their geometry. At 4K, 184 pixels change at
x112–136/y1163–1187. Plaque RGB error improves 23.09→22.00 and source-core
matches 275→312; source-dark bright intrusions increase 6→7. The separate
A2 crop and leading boundary are unchanged. The 1600 capture is identical
to AC; fractional captures change 31 pixels. This is a partial material
improvement, with exact ink and the A2 junction still open. See
[store printing](neomil/store-printing.md).

The dashboard's COMBAT COLONIZATION and DEFENCE PROGRAM runs painted one
and two native pixels taller than their source/SVG counterparts. Native
sizes 9.0 and 8.75, each shifted −0.416667 design px, match the source's
vertical boxes while preserving nominal widths and existing echo offsets.
Whole-line RGB errors improve 22.73→21.16 and 18.72→16.59; red-core F1
improves .645→.676 and .708→.783. Five of six independent sections improve
core overlap; the final COLONIZATION section retains a small regression.
All 2,531 changed 4K pixels stay in the caption/echo region. The code,
divider and frame are unchanged. See [footer calibration](neomil/dashboard-footer.md).

## Rejected trials and remaining work

Kitsch's SC letters look too dark, and SVG opacity 0.4 lowers RGB error on
all four cards. The native port loses every measured dark core. Native
0.7/0.85/0.95 trials improve average color but still worsen held-out stroke
agreement. Even the conservative 0.95 trial improves the fitted first card
while losing agreement on the other three. No opacity proposal is applied
to either runtime or SVG. Investigate native glyph coverage/phase before
another ink adjustment; [the SC record](kitsch/store-art.md) retains the
measurements. Broad gates passed the rejected 0.4 trial, demonstrating why
they cannot replace these local controls.

Neo-kitsch's EMAIL source has crossing and longitudinal grain families.
An added quadratic fan misses their density/direction; replacing three
primary strands leaves an unsupported blank strip. Both worsen independent
RGB or ridge controls, so neither is applied. The
[veneer record](neokitsch/veneer-fit.md) retains their evidence and the
unresolved crossing topology. These failed trials do not establish that
further local fitting is blocked.

## Integration

Seventeen accepted native captures cover two production sizes, fractional
states, selection/held, custom palettes, opening and the dashboard fallback.
The footer production capture exactly reproduces the approved native trial.
The margin remains invariant under shelf-only states and recolors with the
custom foreground. The dashboard frame/code and all unrelated pixels stay
unchanged in matched states. Kitsch's rejected captures are retained
separately under `/tmp/cp-eras-next/ad-kitsch/`.

All 286 Rust tests and the release build pass after restoring the rejected
Kitsch artwork. The two Neomil source gates pass at 72% store / 86%
dashboard shape area; implementation gates pass at 89% / 94%. Thresholds
are unchanged. Only the two reviewed dashboard goldens require refresh;
the store golden is unchanged. The full repository check passes all 22
checks and all 27 visual cases. Each visual case matched 100.000% on its
first attempt. All 201 frozen source/documentation/script/test hashes
match the Nix source used by those exact derivations. Evidence is under
`/tmp/cp-eras-next/`, including the final full-check log, matrix plan,
source verification and native state review. New changes remain
staged/uncommitted; fine material and live-desktop tasks remain open.
