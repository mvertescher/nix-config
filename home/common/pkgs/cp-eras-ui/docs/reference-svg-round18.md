# Eighteenth reference checkpoint — AG, 2026-09-30

Two Sol medium lanes handle Kitsch store details and Neo-kitsch login
badges. Astra owns the Neomil login-printing correction, reviews all
source/SVG/native evidence and integrates the changes. The accepted
AD/AE/AF work remains staged; this round is also uncommitted.

## Kitsch warning height

The warning triangle is shortened locally on all four product cards.
All 12 native whole-triangle comparisons improve across three dark
thresholds. Of 36 first/middle/last segment comparisons, 34 improve,
one is unchanged and one has a small loss (.1437→.1419). Source-rounded
feet, exact ink and microprint remain unresolved. Only 952 native 4K
pixels and 168 golden-size pixels change, all in the warning contours.
Seven fractional states preserve all other artwork, selection feedback,
custom colors and fourth-card clipping. See [the local evidence](kitsch/store-art.md).

## Neomil login caption copies

The three top captions gain their two source-visible secondary copies,
independently fitted to the login photograph. Regular font outlines,
finite stroke profiles and explicit scan ramps share the existing
reference ground preparation. The primary captions and custom palette
path are unchanged.

The first third of each caption was fitted; all six middle/last native
holdouts improve. Bright native masks at red 180/210 and all 1,212 strong
primary pixels are unchanged. Loose SECURITY fringe overlap slips
.80843→.80762 through two added pixels; other loose primary checks improve.
Echo F1 improves from zero to .675–.809 across independent thresholds.
Blank controls and all pixels outside the three caption crops are intact.

Only 12,782 native 4K and 2,532 golden-size pixels change. Six matched
fractional reference states each change the same 2,817 caption pixels;
the custom-palette pair is byte-identical. Long synthetic input, caret
blink and pointer/disabled feedback remain confined to their existing
regions. Other login printing and exact texture remain open. See
[the fit and controls](neomil/login-primary.md).

## Neo-kitsch login badges

The source has larger A/B glyphs, a rounded/chamfered outline and a solid
lower tab. The former generic badge instead drew small Regular letters
and an unsupported internal fold. The measured proposal is restricted
to the login badges; other screens retain their independent artwork.
`Slot.badge_art` supplies the era-owned paths while empty artwork keeps
the existing renderer behavior, following the existing emblem-art model.

At the middle threshold, native glyph IoU improves .049→.717 and
.060→.699; frame and tab overlap improve on both badges as well. All
18 native and 18 SVG first/middle/last glyph comparisons improve across
three thresholds. Each 12-pixel letter-to-tab gap agrees with the source,
and a clear gap patch loses the old fold's six unsupported pixels per
badge. The A remains heavy (605 versus 473 source pixels); exact contours,
frame edges and photographed softness stay open.

Only 4,562 native 4K and 844 golden-size pixels change, confined to the
two badges. Seven matched fractional states each change the same 931
badge pixels, with other artwork and interaction regions byte-identical.
Source/SVG/native measurements are in [the badge record](neokitsch/login-branding.md).

## Integration

All six affected fidelity gates pass at unchanged thresholds. Source/native
matched shape areas are Kitsch store 92%/91%, Neomil login 94%/99%, and
Neo-kitsch login 76%/87%; source Kitsch/Neo-kitsch use the ink-placement
verdict. These broad gates supplement the local source checks above.
All 286 Rust tests and the release build pass. Before/after login previews
match their respective production images byte-for-byte at 1600×900; the
final build also preserves the earlier Neomil accepted capture exactly.
Only the Kitsch store and two affected login goldens are refreshed.

All 24 SVGs parse with unique IDs. The full repository check passes all
22 checks and all 27 visual cases; every case matches 100.000% on its
first attempt. All 218 frozen source, script, documentation, test and TODO
hashes match the tested Nix snapshot. The 44 recorded local/state pixel
checks preserve all expected region boundaries.

Evidence is under `/tmp/cp-eras-next/ag/`, including `check-verification.json`,
`state-review.json`, source/native measurements and review montages. Final
prose records the results; runtime, SVGs, scripts, tests and goldens stay
unchanged from the verified snapshot. All changes remain staged and
uncommitted. Live desktop/IME, hardware presentation and actual greetd
validation remain separate and have not been run.
