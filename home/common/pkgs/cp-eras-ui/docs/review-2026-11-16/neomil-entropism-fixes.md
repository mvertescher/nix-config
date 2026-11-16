# Neomil and Entropism: second-review fixes

Measured 2026-11-16 at 3840x2160 against the reference art, ours with
`CP_ERAS_UI_PRINT=0` unless stated. Answers the gaps in
[neomil.json](neomil.json) and [entropism.json](entropism.json).
Coordinates are design units (1600x900); "px" are native 4K pixels.

## Neomil

### Ink on red: `ON_CARD #1a0405`

The Login label was pale for the reason Neo-kitsch's veneer ink was
([veneer-print.md](../neokitsch/veneer-print.md)): `ON_CARD #420f10` was
sampled where strokes never reach their plateau, and drawn exactly.

| Mark | Reference | Ours until 2026-11-16 | Now |
| --- | --- | --- | --- |
| USER 01 (4 px stems, plateau) | `#1a0608`, darkest `#120204` | `#420f10` | `#1a0405` |
| Login (3 px stems) | eroded median `#4b0f10`, darkest `#1f090a` | `#420f10` flat, darkest `#420f10` | darkest `#1a0405` |
| Switch Weapon (control, printed in `#1a0405`) | eroded median `#470b0c`, darkest `#120101` | `#1a0405` | unchanged |

Login was also Regular, 2-3 px stems against the reference's 3-4; now
Medium (stems 3/4; ink mass at the midpoint 750 against 718, was 526).
`ON_CARD` is the colour of the palette's `on_select` and also prints the
login's margin numerals, the slot marks in the Login button and the
pressed hub glyphs.

### Store CUSTOMER bar: `CUSTOMER_BAR #842d31`

Bar median between the letters: reference `#842b2e` / `#832e35`, ours
was the mid red `#a32226`. Labels in the bright ink in both; ours were
Regular at 70 % of the reference's ink mass (557 against 802), now
Medium (780).

### Margin tabs: `MARGIN_TAB_GAP` 5.4, `MARGIN_TAB_MARK_SIZE` 5

The reference has no crisp mark left of the small square, only striped
print copies of the two squares. The trace's two thin bars (8 and 6 x 1.5,
`#chip` in both traces since the canvas conversion, not a recent change)
were a reading of those copies; removed. The small square measured 5 x 5
(mailbox 5 x 4.6), top-aligned with the numbered square, 5.4 to its left:
store 52.5..57.5 x 187.9, mailbox 51.25..56.25 and 1532.1..1537.1 x 222.1
(ours were 6 x 6, 1 lower). `margin_tab_mark()` places it for both.

### Store top code and arrow

| | Reference | Before | Now |
| --- | --- | --- | --- |
| run x | 860.0..972.9 | 862.5..956.3, mid red | 859.6..972.9, bright ink (stretch 1.204) |
| coverage mass | 855 px | 754 | 937 |
| arrow | tip 979.2, head to 990.6, 5.8 tall, shaft 1.25 | head 977..985, 10 tall, shaft 2 | matches |

### Header badge: reported, not fixed

The plate's fill is right: reference `#43323a` / `#422f35` beside T1 on
the login, ours `#422e34` (mailbox `#473139`, dashboard `#4d3038`).
What differs is where the copies go. Mean / max red at 4K, login:

| Zone | Reference | Ours, print off | Ours, print on |
| --- | --- | --- | --- |
| up-left of T1, inside the plate (128..137 x 119..125) | 69 / 79 | 66 / 66 | 85 / 155 |
| plain plate (160..172 x 140..156) | 66 / 69 | 66 / 66 | 66 / 69 |
| outside the plate, up-left (108..116 x 98..140) | 47 / 77 | 38 / 39 | 38 / 46 |

The reference prints the plate over the copies: nothing of T1 or LEVEL
shows inside it, and a ghost of the plate itself shows outside, up-left.
Ours puts T1's copy inside and almost no ghost of the plate. No Neomil
gate parameter separates the two: the plate's encoded luminance (about
0.20) equals the glow ground's at the top left (about 0.18), so a lower
`echo_gate` would also strip copies from the ground and the inactive
login cards. It needs the print pass to treat such a plate as covering
the copies of what is printed on it (an enclosure test for light ink on
a translucent plate, or a per-plate flag), which is `src/print.rs`.

### Header rule: real but systemic, not fixed

Login at x 1500 and 2500, red channel of rows 450 / 451 / 452: reference
`5e fb 86` (encoded coverage 0.31, 1, 0.49 = 1.80 px = 0.75 dp), ours
`89 fb b8` (0.50, 1, 0.71 = 2.21 px). The line is drawn at `hairline`
0.7 dp, centred where the reference's is; it reads 23 % heavier because
wgpu blends light ink on dark ground in linear light, which brightens
every partly covered row. Fitting the reference's encoded profile under
linear blending wants 0.56 dp. That effect is on every thin light line
in every era, so it is not corrected on this one line.

## Entropism

### Hub caption strips

Both right-hand lines are one cap height in the photo (17 rows); ours
set line 1 at 12 (20 rows) over line 2 at 10.5, and the gap between
them was 5 rows against 6, which the print copies filled.

| | Reference | Before | Now |
| --- | --- | --- | --- |
| line 1 x / y | 522.1..602.5 / 415.4..422.5 | 522.1..597.9 / 415.4..423.8 | 522.1..602.5 / 415.4..422.5 |
| line 2 x / rows | 522.1..590.8 / 1020..1037 | 522.1..587.5 / 1022..1038 | 522.1..590.8 / ~1020..1037 |
| code x | 424.2..482.9, stems 4 | 424.6..480.4, stems 3 | Bold, 424.2..483 |
| selected ink (darkest) | `#1c3412` code, `#1e3b17` lines | `#2e4a2c` | `#223c1b` |
| selected lines ink mass | 1194 / 1056 | 1575 / 1246 (Bold) | 1114 / 993 (SemiBold) |

The selected box also has a frame in the photo: dark 1.4 dp rules just
inside the solid on the left (centre 1.25 in), right (193.15) and foot
(0.2 up), luma about 45 against the sage's 193. Ours had none, so the
strip dissolved into the tile. Drawn now (`CAPTION_FRAME`).

### Store header

The store's strings measure the same as the other three screens' (caps
y 49.2..60.4, first string from x 61.7); ours were Regular 15 on
baseline 61. `strip_label()` now sets all four screens' header strings
(Medium 17, baseline 60, `HUB_STRIP`); the store trace already said so.

### PETROCHEM tag: `STORE_TAG #352a0a`

Box `#382d09` (selected card) / `#31270b` (idle), x 4.5..64 and
y 65..73.3 on the card; ours was `#35462e`, 3..61 x 62..73. Lettering
stems 3-4 px against ours 2 (ink mass 1655 against 1188) and 54 units
wide against 44: now SemiBold stretched 1.18 (mass 1909, stems 3).
BETTERLIFE TEC is unchanged.

### Not attempted

Ink on sage (store stat numbers, login footer, NEXT), item 10 of the
brief.
