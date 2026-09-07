# Design targets

Hand-drawn SVG references, in the strict reference palette (sampled
from the Neo-Militarism Behance images — the consts live in
`src/eras/neomil.rs` now, not the old `src/colors.rs`; see also
TODO.md's palette-correction entry).

**Which file is authoritative: read `docs/PIPELINE.md` and
`docs/sources.md` first.** The four `*-trace.svg` are measured
schematics of the four sourced screens and are what an implementation
is built and judged against. The app-shaped `target-*.svg` and
`dashboard.svg` compositions that predated them were deleted
2026-09-03; the section at the end says what they were.

## The traces — the material

Each carries a header comment narrating its source region by region
with measurements; read that header, not this file, for geometry.
Each is held to its photo by
`scripts/fidelity_check.sh --inventory neomil <screen>`.

- `login-trace.svg` — `images/img-06-private.png` (screen #59): the
  header (CUSTOMER / LEVEL T1 badge, #NC488402 block, SECURITY LEVEL
  T1–T4 with T2 filled) over a hairline rule, and three USER 01 cards
  with a top-right chamfer and a notched left tab — card 1 solid red
  with an avatar box, a password field and a filled Login button;
  cards 2 and 3 dim translucent. Gate: PASS, 79% area.
- `dashboard-trace.svg` — `images/img-07-dashboard.png` (#60), the
  module hub: a broad blue glow over near-black (not a band with an
  edge), the header and hairline rule, a tab row, **a six-diamond
  staggered 3+3 menu** labelled above row 1 and below row 2 — each
  diamond's *outward* tip (row 1's top, row 2's bottom) is cut flat 15
  short of the point, a 30px plateau at 89 from the centre, and the
  inset outline is cut the same way; the trace carries this as two
  cell variants, `#cell-up` and `#cell-down` — and a chamfered GO HOME
  info panel at the right with a bar-and-step motif on its edge, the
  bar chamfered 8 at both ends. A two-cell outlined footer tape sits
  under the panel. Gate: PASS, 94% area. It corrects a previous
  revision that drew three red chart cards, a right rail and a corner
  block; none of those are in the photo.
- `mailbox-trace.svg` — `images/img-08-main.png` (#61): the same
  header, a tab row, a column of eight disc icons beside an eight-row
  message list (row 1 filled, three rows flagged NEW), a scroll rail,
  the "Urgent Information (!)" outlined panel, and four buttons with
  Switch Weapon filled. Gate: PASS, 86% area.
- `store-trace.svg` — `images/img-09-store.png` (#62): **no header
  row** — a KIROSHI chip strip, the MASURAO logotype, a filled
  CUSTOMER bar, LOYALTY DISCOUNT / LAST UPDATE lines, a five-row nav
  with bottom-left chamfers, and four MAGNUM 650 HAND GUN cards with
  the second selected and grown. Gate: PASS, 71% area.

## Motion

The traces carry their boot-ins as SMIL on the element that moves
(`docs/PIPELINE.md` § Motion); rsvg draws none of it, so the gate
numbers above are unchanged by the annotations, and
`scripts/frame.sh --at <s>` shows a moment. Neomil comes up as
top-down wipes — a `<clipPath>` whose rect grows from no height,
EaseOutCubic (`0.33 1 0.68 1`), at ~1280 px/s — and the cycle on the
login. The ids, all frozen well before `motion::REST` (2.4 s):

- `login-trace.svg` `#caret-blink` — the `__` at the end of the masked
  password run, 1.2 s discrete cycle.
- `dashboard-trace.svg` `#panel-open` — the GO HOME panel and its
  glitch echoes, clip x 1120..1380, y 306..766, 0.36 s from 0 s.
  Transcribed (`src/eras/neomil.rs`).
- `store-trace.svg` `#shelf-open` — all four product cards as one
  curtain, clip x 430..1570, y 144..808, 0.5 s from 0 s; cards 1, 3, 4
  are whole at y 613 and the selected card keeps coming to y 800, so
  the wipe ends on the selection. The nav column and logotype stay.
- `mailbox-trace.svg` `#list-open` — the eight disc icons and the
  eight message rows, clip x 125..525, y 305..883, 0.44 s from 0 s;
  and `#message-open` — the Urgent Information panel and its four
  buttons, clip x 720..1465, y 304..768, 0.36 s from 0.15 s (a `<set>`
  holds the rect at 0 until then). Heading, scroll rail and R widget
  stay. Not `#panel-open` because the dashboard's has that id and
  both tables share `src/eras/neomil.rs`.

Rest-frame check 2026-09-07 for the store and mailbox annotations:
rsvg before/after 0 px on both; `frame.sh --at 2.4` before/after 0 px
on both; `frame.sh --at 2.4` against the unannotated rsvg render at
8-level fuzz 838 px (store) / 703 px (mailbox), identical to the
unannotated files' own Firefox-vs-rsvg residue (glyph rasterising,
the kanji among it), so the clips nick nothing at rest.

## Hover and press

Read 2026-09-07 from the whole run, still by still: `images/run-neomil/`
53 (title card), 54 (mood board), 55 (the login as a tilted 3D mock),
56 (the hub beside a text column), 57 and 58 (login + store, hub +
mailbox on tablet mocks), 59..62 (the four screens), plus the four
full-res photos `img-06..09` with every button, nav row, list row,
field, badge and diamond cropped at native 3840 scale.

**The run shows no hover and no press.** No still carries a pointer.
No control is drawn differently from its siblings except the
selections the traces already transcribe: the filled VIDEO nav row,
list row 1 and its disc, SECURITY LEVEL T2, the grown and washed
product card 2, the filled Switch Weapon button. The hub's six
diamonds are identical in fill, outline and size (VEHICLES carries a
different barcode and reads 161-9A against the others' 61-9A, which
is content, not a state); their offset echoes are glitch residue on
all six. So the reading on `components.svg` § 9 is **inferred, not
sourced**, and every hover and press group there is captioned so. It
is built only from the era's own rungs of emphasis, all sourced:

- *rest* — dark translucent fill, thin mid-red stroke, bright ink, dim
  spine (nav AUDIO, list rows 2..8, the Confirm / Jump buttons).
- *lit* — the same outline with a translucent wash of the bright red
  inside it (product card 2 against card 1; on the photo the upper
  card reads +45 of 193 red levels over card 1, i.e. the bright ink
  at about 0.22 over the ground).
- *inverted* — solid bright fill, dark ink, no stroke, bright spine
  (VIDEO, row 1, Switch Weapon, T2). This is selection.
- *held* — the darker of the era's two filled-control reds: the Login
  bar is `#a52223` with `#420f10` ink where every other filled
  control is `#e63132` / `#df3131`.

The reading: **hover is one rung up** from where the control is. An
outlined control takes the lit wash (bright ink at 0.22 over its rest
fill: button `#451010`, nav row `#551719`, field `#6a1617`) and its
stroke, spine and ink go to the bright ink; a filled control lifts to
`#f63333`, the lightest ink the traces sample. **Press is the held
red `#a52223` with the dark ink on every class**, so a press still
reads on a control that is already selected. Neither changes
geometry: the selected nav row's extra 5px and the product card's
growth are selection, not press. The groups are `#button-rest/hover/
press` (outlined), `#button-filled-rest/hover/press`,
`#nav-rest/hover/press` (the same reading applies to a list row and a
product card) and `#field-rest/hover/press`.

The field is the exception in which direction is sourced: the run
holds one field, the login's password field, and it is drawn
*focused* (masked run, lit caret), so `#field-press` (= focus) is the
sourced state and `#field-rest` is the inference, the same box with
the caret gone.

One mark the run does show and the band does not transcribe as a
state: the Login bar carries a **hollow slot hanging from its top
edge at its centre**, 3x19 at trace scale (x 497.5..500.6, y 635..654;
native x 1194..1202, y 1523..1570), outlined about 0.6 in the bar's
dark ink, open at the top. It is in stills 55 and 57 too.
`login-trace.svg` draws it as a solid 2x15 rect at (496,640) and
calls it a text caret; on the photo it is hollow and 5px higher (a
trace correction for another pass, not made here). None of the four
mailbox buttons has one, so a single instance cannot be read as a
state; focus on the default action is the obvious candidate and a
candidate is all it is. It is drawn once at 1:1 in § 9
(`#login-slot-observed`) so the shape is on record.
## The widget sheet — derived from the traces

- `components.svg` — every reusable component of the four screens,
  drawn once in isolation, built 2026-09-03 from the four traces and
  `bar.svg`. Every component is a translate-only copy of a trace
  element with an XML comment citing file and coordinates and an
  on-sheet caption of the numbers: the header chrome (CUSTOMER/LEVEL
  badge, SECURITY LEVEL row, #NC488402 block, `next` and MASURAO
  logotypes, tab boxes, footers, margin chips), the store nav cell
  both states, message-list rows (selected / NEW / plain) with the
  disc column and scroll widget, the menu diamond and its separator,
  buttons, the NEW pill, the user card active and idle, the product
  card plain and selected, the GO HOME panel, the message-panel
  corner, then 36 measured palette values, typography, ground stops,
  observed era rules and an implementation-delta box listing where
  `src/eras/neomil.rs` still disagrees with the traces (a global
  Chamfer 15, the RED consts, white tape, `Ground::Flat`, OpsCharts,
  `Menu::Table`), and since 2026-09-07 a § 9 hover/press band at the
  foot (canvas 1920x1310; see "Hover and press" above). Not gated —
  the traces are. It replaces the deleted
  `target-components.svg` below, which sampled nothing.

## The bar — the one original

- `bar.svg` — the status bar: host tape, workspaces, tray, the
  wired/audio/CPU/MEM modules and the clock, at the 1600x220 geometry
  the bar golden tests render. The bar has no photo source, so this is
  an original composition, redrawn 2026-09-02 from the four traces'
  chrome with every decision cited by file and coordinate in the SVG
  header. **It no longer matches `bar()`**: it is the design target and
  `bar.rs` has not followed yet (crate TODO.md § "Bar restyle"), so
  read the SVG's IMPLEMENTATION DELTA block, not the current render.

## Deleted composites (2026-09-03)

Three app-shaped drawings used to sit beside the traces; `docs/sources.md`
keeps a row per file saying what each got wrong. In short:

- `target-app.svg` — "NEOMIL OPS", a services table + sessiond panel.
  An original composition despite its "traced from `img-08-main.png`"
  claim; superseded by `mailbox-trace.svg` and `store-trace.svg`.
  `src/widgets/table.rs` and `style::Menu::Table` were built to it.
- `target-components.svg` — a widget sheet that claimed to be "sampled
  across the run" and sampled nothing from it. Replaced the same day by
  `components.svg` above, built from the traces.
- `dashboard.svg` — the ops-charts composite `screens::dashboard`
  assembled under `Layout::OpsCharts` until the fold of 2026-09-03 late (the screen is now a `Prim` table transcribed from the trace, G2i 96%): three red chart diamonds, a
  right rail, a corner block). The photo holds the six-diamond hub
  `dashboard-trace.svg` draws; the inventory gate scored the composite
  at 0% of the source's shape area. Until the `Layout` decision in the
  crate TODO.md, the dashboard screen has no SVG that agrees with it —
  G2i now compares it against the trace and reports that honestly.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/neomil-bar.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 900 dashboard-trace.svg -o /tmp/neomil-dash.png
```

Render with Rajdhani + Orbitron available to fontconfig:

    FONTCONFIG_FILE=<conf with the fonts> rsvg-convert -w 1600 -h 900 login-trace.svg -o /tmp/sheet.png
