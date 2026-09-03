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
  staggered 3+3 menu** labelled above row 1 and below row 2, and a
  chamfered GO HOME info panel at the right with a bar-and-step motif
  on its edge. Gate: PASS, 94% area. It corrects a previous revision
  that drew three red chart cards, a right rail and a corner block;
  none of those are in the photo.
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
  across the run" and sampled nothing from it.
- `dashboard.svg` — the ops-charts composite `screens::dashboard` still
  assembles under `Layout::OpsCharts` (three red chart diamonds, a
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
