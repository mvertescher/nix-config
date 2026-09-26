# Mailbox source correction

The design target is source #61, `images/img-08-main.png` at 3840×2160.
Its 1600×900 coordinates divide native coordinates by 2.4. This screen is
`cp-eras-ui-mailbox` / `screens::mail::MailBox`, including the dashboard
hub route; the separate `cp-eras-ui-mail` example is a different client.

The 2026-09-21 audit compared the original, `mailbox-trace.svg` and a
fresh Iced capture. Shape gates passed despite visibly undersized body
copy, simplified cartridges, missing rotated printing and incorrect
material. Frozen screenshots also concealed the standalone launcher's
missing animation subscription. A passing inventory or golden alone
therefore does not establish correctness.

## Typography and primary printing

| Region | Source-supported setting in design pixels |
| --- | --- |
| Body | Rajdhani Regular 17.5; x750, first baseline 347.5; 21 line pitch; one extra line between paragraphs |
| List subjects/senders | Rajdhani Regular 17.5; baseline 339.4167, then 409.4167 +70 per row |
| Message heading | Rajdhani Bold 20; original x742/baseline287 |
| Action labels | Rajdhani Regular 16.6667; left inset 12.5, baseline 746.6667 |
| Header captions/tabs | Rajdhani Medium 12.9167; native-source positions |
| Margin codes | Rajdhani Regular 11.25, no added tracking, rotated −90° |
| Maker labels | Clockwise 90°; BETTERLIFE TEC and framed PETROCHEM in the panel corner |

At native scale the first body line's core ink starts around x1804
(design 751.67), while its advance at font 42 is 1531.64 native pixels
(design 638.18). Its font origin is x1800/design 750; the visible sidebearing
accounts for the remaining offset. The previous 15px SVG advance was
about 545px. All ten original line breaks and words are retained. The
source's bright stroke cores in body, normal list, heading and outlined
button text are RGB(251,53,53), sampled independently in each region.

The LEVEL lettering and custom T1–T4 shapes match the dashboard's source
forms. Their paths are reused with mailbox placements; the header's
protocol block, notices and tape are measured separately. The tape's
leading miniature inscription is not legible. It is recorded as observed
small ink modules rather than assigned a guessed string.

Static text is outlined from the bundled Rajdhani fonts, so SVG and Rust
use the same geometry without depending on text rotation inside the
software compositor. `neomil_mailbox_art.rs` exports:

- `HEADER`: primary header/badge outlines, protocol and tabs; badge
  interiors come from the mailbox material table.
- `MARGINS` and `FOOTER`: margin chips/codes and footer arrow/printing.
- `PANEL_PRINTING`: both maker labels and the PETROCHEM frame, painted
  inside the message opening motion.
- `NEW_PILL`: source geometric NEW lettering and its complete rounded
  outline, recolored with the row printing state.
- `ICON_NORMAL`, `ICON_SELECTED` and `ICON_POSITIONS`: complete cartridge
  paths and measured placements.

## Cartridges

The old four-corner casing and solid ellipse did not match source #61.
Native crops show a stepped right edge, a narrow contact strip, an inset
lower tile, a C-shaped disc with a lower cutout and two eccentric inner
rings. The new SVG and Rust paths preserve all of these features.

The normal cartridge template is traced from the second source icon,
whose native reference origin is (452,948). Correlation of its bright disc
against the six following source crops gives these design origins:

| Row | Normal anchor x | Anchor y |
| --- | ---: | ---: |
| 1 | 188.3333 | 326.6667 |
| 2 | 188.3333 | 395.0000 |
| 3 | 188.3333 | 463.3333 |
| 4 | 188.3333 | 531.2500 |
| 5 | 188.3333 | 599.5833 |
| 6 | 188.3333 | 667.9167 |
| 7 | 188.3333 | 741.6667 |
| 8 | 188.3333 | 809.5833 |

The selected variant carries a −15.8333px horizontal displacement, giving
row 1 its source top position x172.5. Keeping that displacement in the
selected variant makes selection of another row use the same visual
state; the source only establishes the initial row 1 selection. The list
opening clip starts at x115 and is 410px wide so the corrected left tip
at about x122 is fully visible. It retains the original 440ms timing.

## Review and remaining limits

Inspect source→SVG→Iced with `scripts/triptych.sh --diff neomil mailbox`,
and run both inventory and implementation modes of
`scripts/fidelity_check.sh`. Use the freshly built binary directory via
`--bin-dir`. Also inspect a real-clock headless launch: fixed rest-frame
renders cannot prove an animation subscription is wired.

These corrections target primary geometry, typography, missing printing,
row fields and mailbox-specific material. Native source images contain
scan variation, glow and offset printing echoes. The primary vector
paths do not completely reconstruct those residual effects around every
letter and cartridge. Cartridge curves are compact geometric traces,
not an assertion of pixel-perfect correspondence. Their terminal strip
is schematic at subpixel scale. Such residuals should remain visible in
source comparison rather than being hidden by changing gate thresholds
or tagging primary artwork as photographic residue.

The body remains fixed source lorem; action/scroll artwork remains
primarily a visual design target. Row selection must show the selected
subject after leaving the explicit initial fixture. These are separate
requirements from a working mail backend.

## Row follow-up from the combined capture

Native review corrected two more inherited assumptions. The bright primary
selected plate begins at native y757 (315.4167); the higher y313 mark is
an offset printing echo. Primary row geometry is x241.25,y315.4167,
269.5833×67.5 on a70px pitch, with a15px lower-left cut. Normal outlines
are approximately RGB(102,14,15),0.8333px wide. No solid3px spine appears
at x237–240; rendering one turned faint source residue into bright artwork.
The selected plate's clear interior is approximately RGB(227,49,49).
Selected text's flat representative is RGB(83,23,25); native ink varies
under the source's scan/echo treatment.

Direct correlation of the bundled Regular42px glyphs against native list
crops confirms17.5design type and baselines near339.58,409.17 and479.17.
The shared layout uses row origin+24.0, retaining at most0.42px of source
placement variation. The earlier337.5/407.5 baseline transcription was
too high and is superseded by those native fits.

The source NEW letters occupy native(1028,1038)–(1096,1052):
28.3333×5.8333 design pixels. They are wide geometric forms, not Rajdhani
Bold. The pill centreline frame is75.4167×10.8333, with a1px stroke and
3px corner turns. Its normal origin is relative(162.5,44.375) and its
selected origin is relative(162.5,46.875), both measured against the
logical row. Keeping the selected offset explicit preserves the initial
source placement without shifting every normal pill down.
