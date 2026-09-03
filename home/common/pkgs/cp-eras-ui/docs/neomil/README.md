# Design targets

Hand-drawn SVG references, in the strict reference palette (sampled
from the Neo-Militarism Behance images — the consts live in
`src/eras/neomil.rs` now, not the old `src/colors.rs`; see also
TODO.md's palette-correction entry).

**Which file is authoritative: read `docs/PIPELINE.md` and
`docs/sources.md` first.** The four `*-trace.svg` are measured
schematics of the four sourced screens and are what an implementation
is built and judged against. The `target-*.svg` and `dashboard.svg`
files predate them and are app-shaped compositions; where a trace
covers the same screen, the trace supersedes it.

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

## The older composites

- `target-components.svg` — every widget the toolkit should grow, with
  states: buttons (primary/ghost/override/disabled/icon), text input
  (+focus), select (+open list), slider, toggles/checkbox/radio, tab
  bar, meters (segmented/bar/indeterminate), badges/tags/status dots,
  toast+banner (warn/error), modal, table with selection + scrollbar,
  key-value rows, log view, status bar, context menu, tooltip, 16px
  icon set. Sampled across the run.
- `target-app.svg` — "NEOMIL OPS": a services table, meters, a live
  log, a detail panel, action buttons, a nav rail and a status bar.
  **An original composition, not a trace** — the old "traced from
  `img-08-main.png`" claim was false, and it is superseded by
  `mailbox-trace.svg` and `store-trace.svg` (2026-09-02). It shares
  only the palette and the header grammar with the material; do not
  use it as the reference for a screen a trace covers.
- `bar.svg` — the status bar: host tape, workspaces, tray, the
  wired/audio/CPU/MEM modules and the clock, at the 1600x220 geometry
  the bar golden tests render. The bar has no photo source, so this is
  an original composition, redrawn 2026-09-02 from the four traces'
  chrome with every decision cited by file and coordinate in the SVG
  header. **It no longer matches `bar()`**: it is the design target and
  `bar.rs` has not followed yet (crate TODO.md § "Bar restyle"), so
  read the SVG's IMPLEMENTATION DELTA block, not the current render.
- `dashboard.svg` — an **app-shaped composite**, not a trace: the
  ops-charts screen `screens::dashboard` assembles under neomil's
  `Layout::OpsCharts` — a cold-blue top band with red crests, three
  large red diamonds carrying chart polylines, a right rail and a
  corner block — at the 1600x900 geometry the dashboard golden tests
  render, which is what G2 compares against the golden. It is not the
  material: the photo holds the six-diamond hub `dashboard-trace.svg`
  draws, and the inventory gate scores this screen's neomil golden —
  the same composition — at 0% of the source's shape area
  (`src/screens/dashboard.rs`).

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/neomil-bar.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 900 dashboard-trace.svg -o /tmp/neomil-dash.png
```

Render with Rajdhani + Orbitron available to fontconfig:

    FONTCONFIG_FILE=<conf with the fonts> rsvg-convert -w 1600 -h 900 target-components.svg -o /tmp/sheet.png
