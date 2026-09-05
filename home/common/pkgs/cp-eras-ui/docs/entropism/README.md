# Entropism design targets

Sampled from the Behance Part 1 entropism run: the nine screens after
the "ENTROPISM — NECESSITY OVER STYLE" title card — logins, the module
hub, the mail screen and the 4ST store. An earlier revision numbered
the run "doc #24–32"; those positions come from a smaller scrape and
are shifted by ten. `docs/sources.md` holds the canonical positions
(title card 33, screens 34–42) and the Behance ids, and is where each
SVG's source is recorded.
See `../kitsch/README.md` for how the four era runs were recovered from
the gallery.

**Which file is authoritative: read `docs/PIPELINE.md` and
`docs/sources.md` first.** The four `*-trace.svg` —
`login-trace.svg`, `dashboard-trace.svg`, `mailbox-trace.svg`,
`store-trace.svg` — are measured schematics of the four sourced
screens, each gated by `scripts/fidelity_check.sh --inventory entropism
<screen>`, and each carries a header comment narrating its source
region by region with measurements. Read those headers, not this file,
for geometry. The app-shaped `target-app.svg` and `dashboard.svg`
compositions that predated them were deleted 2026-09-03; the notes at
the end say what they were. `components.svg` is the widget sheet, rebuilt
from the traces the same day.

Note the file-name swap recorded in `docs/sources.md`:
`images/entropism-store.png` is the **module hub** (traced by
`dashboard-trace.svg`) and `images/entropism-dashboard.png` is the
**4ST store** (screen #42, traced by `store-trace.svg`).

## Sampled palette

Sampled off the reference and carried as the era consts in
`src/eras/entropism.rs`:

```
bg          #110c07   warm dark olive-brown (#1a140c upper, #0d0603 lower)
sage solid  #a6d3a7   selection fills, footer band (was #9cb795 until 2026-09-05)
sage text   #94bb94   labels, titles
mid         #728f76   top-bar text, secondary
outline     #8fba97   frame strokes (was #5d7752 until 2026-09-05)
dim         #3d4d38   faint rules, captions
on-solid    #1f2a1c   dark text on sage fills
```

The traces' own k-means samples run brighter than these (see each
trace header's palette block). Two were settled 2026-09-05: the
outline ink by eye against the photographs -- the frames are the bright
sage, the 1.25px core every trace's stroke profile was measured to, and
#5d7752 was the one value of the four screens whose outlines receded
into the ground -- and the solid, which followed it: at #9cb795 a
selection fill no longer stood off the brightened frames, and the three
traces agree on ~#a6d3a7 for it (a fill, unlike a line, does not dilute
in the rescale).

**One hue, with one exception.** The era is a single sage green on a
warm dark ground — a monochrome terminal that somebody keeps repairing
— except on the store, where each product card carries a yellow
PETROCHEM / BETTERLIFE TEC band (`store-trace.svg` samples it at
`#eebf09`, 1.1% of the canvas: the only non-sage ink in the run).

## Observed era rules

- Square everything; no rounding, no chamfers, no gradients.
- **The designed stroke is 2px**, not 1px: `mailbox-trace.svg` measures
  every outlined frame and every row divider at 2px. What the photo
  adds around each bright edge — a 1px near-black undershoot and a
  faint sage overshoot a few px out — is a photographic halo, so the
  trace draws it and the iced implementation draws the 2px stroke only.
  The "no glow" rule stands for the implementation.
- Selection is a solid sage fill — tiles, list rows, nav rows, buttons,
  T-levels.
- One full-width outlined header strip on every screen, cut by two
  dividers into a left, a centre and a right string (RIPPERDOC SURGICAL
  SOFTWAREV2 / STORE ACCESS SCREEN / FLAIR TRS 5MMP; the store's left
  string reads DIGITAL DISTRIBUTION SOFTWAREV2).
- A build-string footer on every screen (`INTERFACE LOADED · PROVIDED
  BY NEXUS NETWORK V10.8 · BUILD 6.47.48441.R15`), drawn as a thin
  outlined strip on the hub, mail and store screens; the login swaps it
  for one tall solid sage band with no outline.
- Boxed-letter section headers ([A] MAIL BOX, [B] MESSAGE …); on the
  store the letters sit *below* the things they label.
- Menu tiles carry two-cell caption strips at their foot.
- Dense small maintenance captions throughout.

## Toolkit divergence (handoff) — historical

This section was written when entropism lived in its own crate. The
fold into `cp-eras-ui` has happened and the one-hue reduction it asked
for was done: neither `src/colors.rs` nor `src/glow.rs` exists any
more, and `src/eras/entropism.rs` carries the seven consts above and no
glow. What remains open is the desktop theme (`home/themes/entropism`):
its default `burn-in` variant is amber, an invention; of its three
variants only `salvage-phosphor` is near the reference, and it is still
cooler and darker than the sampled sage. Suggested then and still
open: add a reference-sampled variant (working name `nexus`, after the
build strings) and make it the default.

## Files

- `login-trace.svg` — `images/entropism-login.png` (#39): the header
  strip, an empty upper two thirds, a `USERNAME:` label over an
  outlined masked field with a caret and a filled NEXT button, and the
  tall solid sage footer band. Gate: PASS, 83% area.
- `dashboard-trace.svg` — `images/entropism-store.png`, the **module
  hub**: the header strip, boxed A MAIL BOX / B MESSAGE / C SECURITY
  LEVEL letters, a **3x2 grid of six tiles** (BRAINDANCE filled solid
  sage, the selection) each with a two-cell caption strip at its foot,
  a MESSAGE detail panel beside the grid, a four-badge SECURITY LEVEL
  column with T2 filled, and the outlined footer strip. Gate: PASS, 92%
  area. It replaces a fabricated revision that named the wrong source
  and claimed the grid, sidebar and detail panel were not in the frame.
- `mailbox-trace.svg` — `images/entropism-mail.png` (#41): the header
  strip; A MAIL BOX / B MESSAGE / C ENCRIPTION LEVEL (the source really
  spells it that way); an outlined seven-row list with row 1 filled and
  an envelope glyph per row; the MESSAGE panel with a filled title bar
  over three lorem paragraphs; a four-segment button row with REPORT
  SPAM filled; a 2x2 badge grid reading T1 T3 / T2 T4 with T2 filled;
  the footer strip. Gate: PASS, 88% area. Its header also carries the
  measured edge profile every entropism frame photographs with.
- `store-trace.svg` — `images/entropism-dashboard.png` (#42, the store
  despite the name): the header strip; the 4ST logotype over S T O R E;
  an outlined CUSTOMER box with LOYALTY DISCOUNT / LAST UPDATE lines; a
  five-row category nav (SMG filled) closed by one tall empty cell;
  four MAGNUM 650 HAND GUN cards, the **first** grown and filled solid
  sage through its values row with the recoil/spread/range and bonus
  block beneath; boxed A and B letters in the foot; the footer strip.
  Gate: PASS, 80% area. Supersedes `target-app.svg`.
- `components.svg` — the widget sheet, rebuilt 2026-09-03 from the
  four traces and `bar.svg` (it was `target-components.svg`, drawn by
  eye before the traces existed; the traces contradicted every
  dimension on it — 120x120 hub tiles against the traced 194x212, the
  wrong card grown, no PETROCHEM band, no socket row). Every component
  is a translate-only copy of a trace element with an XML comment
  citing file and coordinates and an on-sheet caption of the numbers:
  header and footer strips, the login footer band, letter boxes,
  mail-list rows and envelope glyphs, the category nav cell both
  states, the hub tile both states, the product card plain and grown
  with its socket row and QR glyph, the 4ST logotype, the message
  panel and button row, the hub detail panel, security badges, the
  login field and NEXT button, then the sampled palette, typography,
  ground stops, observed era rules and an implementation-delta box
  listing where `src/eras/entropism.rs` still disagrees with the
  traces (stroke 1.0, palette, OUTLINE/BG, TileRow). Not gated — the
  traces are; this is derived from them.
- `bar.svg` — the status bar: host tape, workspaces, tray, the
  wired/audio/CPU/MEM modules and the clock, at the 1600x220 geometry
  the bar golden tests render. The bar has no photo source, so this is
  an original composition, redrawn 2026-09-02 from the four traces'
  chrome — the bar *is* the era's header strip: one outlined frame with
  dividers, no cell gaps, one filled segment per run. **It no longer
  matches `bar()`**: it is the design target and `bar.rs` has not
  followed yet (crate TODO.md § "Bar restyle"), so read the SVG's
  IMPLEMENTATION DELTA block, not the current render.

## Deleted composites (2026-09-03)

Two app-shaped drawings used to sit beside the traces; `docs/sources.md`
keeps a row per file saying what each got wrong. In short:

- `target-app.svg` — the 4ST store as a loose composite, superseded by
  `store-trace.svg`: it grew the second card where the photo grows the
  first, and had none of the pale header blocks or the yellow PETROCHEM
  / BETTERLIFE TEC band.
- `dashboard.svg` — the tile-row composite `screens::dashboard`
  assembled under `Layout::TileRow` until the fold of 2026-09-03 late (the screen is now a `Prim` table transcribed from the trace, G2i 98%): a boxed [A] TILE MENU header over
  a single row of four tiles and a caption strip. `dashboard-trace.svg`
  measures the hub as a 3x2 grid of six tiles **with** a MESSAGE detail
  panel and a SECURITY LEVEL badge column; the "row of four tiles, no
  sidebar" reading came from a fabricated revision of the trace. Until
  the `Layout` decision in the crate TODO.md the dashboard screen has
  no SVG that agrees with it — G2i now compares it against the trace.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/en-bar.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 900 dashboard-trace.svg -o /tmp/en-dash.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 store-trace.svg -o /tmp/en-store.png
```
