[All workstreams and current status](../TODO.md). File paths in these
records are relative to the crate root. Dated notes retain their original
reasoning; later completion entries supersede earlier open-item lists.

### Bar restyle (2026-09-02) — SVG done; `bar.rs` followed 2026-09-03

The four `docs/<era>/bar.svg` were one skeleton with a palette swap.
Each was redrawn by a vision agent from its era's four verified
traces — the bar has no photo source, so the traces' chrome is the
only legitimate vocabulary — with every decision cited by file and
coordinate in the header, and an **IMPLEMENTATION DELTA** block at the
foot written for a coding model (no images needed). Rendered and
checked at 1x and 2x: legible, no overlaps, palette-only. G2 drift is
the point, not a regression: neomil 0.989/0.890/0.966 →
0.741/0.729/0.694, entropism 0.986/0.904/0.942 → 0.843/0.667/0.942,
kitsch 0.973/0.893/0.956 → 0.934/0.860/0.923, neokitsch
0.981/0.904/0.937 → 0.554/0.782/0.747.

- [x] neomil: the SECURITY LEVEL badge row — bottom-**left** chamfer 6
  on 25px cells (no neomil cell cuts BR; the old bar did), RED_DEEP
  fill / RED_FILL 1.5 stroke, bold digits, BAND_TOP→BAND_BOTTOM glow
  band with the 1.5px RED_MID rule under it, barcode host tape, GO HOME
  panel's right-edge bar + echoes on the menu, arrow submenu marker.
- [x] entropism: the bar *is* the header strip — one 2px outlined frame
  x 6..1594 with 2px dividers, no cell gaps, one filled segment per run
  (workspace 3, clock), solid MID tape with ON_SOLID ink, alert = the
  source's `(!)` suffix with no ink change, disabled = OUTLINE ink.
- [x] kitsch: chevron workspaces (store nav, 40 wide on 46 pitch),
  stepped USER/DESCRIPTION boxes for tape and window label, r8 chips
  at 1.25px on .5 coordinates, weight 500, mint bracket fading along
  the bar foot (opacity 1 to x 264, 0 at 440), teal wave in the menu
  foot, chamfered two-piece selected rows.
- [x] neokitsch: violet haze ground clipped to y 0..31, r3 cells with a
  10x7 BL cut and a 22/16/4 VENEER_LIGHT tab, veneer + grain + seam for
  selection and tape, solid AMBER square alert plate, 8-strand wire
  band bridging the centre gap, unboxed 18px clock, chamfer-22 menu
  cards with four onion rings clipped at y 31.

Open:

- [x] **Decide the bar's corner before touching `bar.rs`.** Decided
  2026-09-02: per-corner cuts, see "SVG→iced pre-work". Three of
  four deltas want a corner the era's screens do not use (neomil BL
  chamfer vs `Corner::Chamfer` cutting one fixed corner; neokitsch r3+BL
  cut vs `ClipTopRight 30`; kitsch r8 unclamped vs pill). Either `Bar`
  in `style.rs` grows its own corner field, or `Corner` learns per-side
  cuts — the neomil agent notes the material cuts different corners per
  widget on every screen (badges BL, login cards TR 46 + BL 22, buttons
  BR 9, panels TR 8 + BL), so the second is the honest fix and affects
  every neomil screen, not just the bar.
- [x] **Neomil idle label contrast**: RED_FILL on RED_DEEP is faithful
  to the badges but the dimmest strip of the four at 14px. Checked on
  the real monitor 2026-09-06: reads fine at the desk. Keep RED_FILL;
  do not lift the idle ink.
- [x] **`bar.rs` follows the four deltas** — landed 2026-09-03:
  `bar.rs` rewritten against `Style::bar` tables (`Dress`, `BarGround`,
  `BarChrome`, `BarOrnament`, `WindowLabel`, `Tab`, `MenuRule`,
  `MenuMarker`, `PanelEcho`, `BarMenu`), goldens re-taken. G2i:
  entropism 100%, neomil 86%, kitsch 83%, **neokitsch 52% FAIL,
  accepted** — the extractor fragments the veneer plates and ring bands
  into cells the design does not have; measured ceiling 58.5% with the
  trace's own 52px-period zigzag grain drawn (the period is what seeds
  the extractor's cells; documented in `bar.rs`). Neokitsch labels were
  `Face::Medium` (Regular was visibly thinner than the design's
  dark-on-gold; decided by eye) until 2026-09-04, when the 600 file was
  embedded and they became the trace's `SemiBold`. Neokitsch
  stroke is 2.0 not the delta's 1.6 (AA). The tray diamonds render
  purple/orange because the example feeds sample SNI raster icons; the
  SVG's diamonds are stand-ins, not a fault. **The layer-shell bar is
  unverified until a switch**; only `bar-window` is captured.
  - [x] "Not followed, per era" — six deviations, filed 2026-09-03 as
    each needing "a widget, not a value". Spot-checked 2026-09-05: five
    of the six were values or a shape the existing widget could carry,
    and the sixth (the haze blue annulus) wanted the haze unification
    rather than a widget. All six closed by 2026-09-05; the entries
    stay for the reasoning.
    - Entropism selection inset 1px — *fixed*, and it was worse than an
      inset: the module faces were drawn flush to their cells over the
      strip's chrome, burying the inner half of every divider next to a
      filled cell and the frame's whole top and bottom edges under the
      tape, workspace 3 and the clock. Two changes in `bar.rs`: under
      `BarChrome::Frame` the module row is laid out from the frame's
      centreline (`frame_edge`, half a stroke inside the padding, which
      is also where the design measures its segments — tape 7..72, not
      6..71) and each face is inset half a stroke (`plate`). Pixel
      transitions now match the design across the left run.
    - Neomil window box 1.0 stroke — *fixed*: `WindowLabel::stroke:
      Option<f32>`, threaded through `face_canvas`.
    - Neokitsch window label at 400 — *fixed*: `WindowLabel::face:
      Option<Face>`; also the clock, which `bar.svg` §7 sets at 500 and
      the code set in the strip's 600: `clock_plain` is now
      `Option<(u16, Face)>` and the digits sit against the right of
      their reservation (the design right-aligns to 1594; the estimate
      ran ~10px wide of the face's advance).
    - Kitsch chevron shoulder — *fixed* with a new `Cut::Peak { x, y,
      brow }` in `widgets/surface.rs`: the rising edge to a peak, a
      drop onto a brow, and the top edge running *below* the box's top
      from there. `outline` and `span_at` both honour it (the top-right
      corner bites from the brow); top-left only, elsewhere it is the
      chamfer. Unit-tested against `#chev` scaled 25/46; the 13-of-25
      rise trips `extent`'s half-height clamp and is squeezed 12.5/13,
      under half a pixel.
    - Neokitsch highlight row 8px past the panel edge — *fixed*
      2026-09-05, and the premise recorded here was wrong. It did
      **not** need the panel canvas to draw the highlight, and no hover
      state had to reach `Panel`; that reading assumed the row's box
      was the panel's inner width and could not be anything else. The
      fix went the other way round: the rows' boxes were widened past
      the panel's outline and the panel was told to leave that strip
      undrawn. `BarMenu::row_overshoot` (neokitsch 8, zero elsewhere);
      `menu_panel` widens the root container by it, gives the rows'
      column back the `ring_inset()` it padded on the right, and hands
      each row the sum as `edge`; `menu_row` pads its content and
      insets the open face and the separator rule by `edge` again, so
      the highlight is the only thing that moves; `Panel::draw`
      subtracts `overshoot` from its width so its outline stays put.
      `bar::menu_overshoot` is the host's share — `bar-window`
      subtracts it from `MENU_MARGIN`, so the panel still ends on the
      design's x=1480 and the plate runs to 1488.
      Measured on the re-taken golden: the plate's last veneer pixel is
      1486 against a panel outline at 1478..1479 — the design's 8, off
      the outline the chain actually lands on. Only neokitsch sets
      `PanelEcho::Rings`, so `edge` is 0 in the other three and no
      other era's geometry moves; against the old golden the whole
      diff is one 121x26 box at (1366,111) — the row's right end plus
      the veneer seam and grain hairlines that re-space with it.
      A submenu's rows now take `edge = ring_inset()` and no
      overshoot, so a highlight there crosses the rings but stops at
      the panel outline. Nothing in the design says otherwise, and
      `bar-window` has no highlighted submenu row to show it.
      `cp-eras-ui-bar` needed no change: it places the chain by
      `menu_chain_width` inside a full-width surface, so the strip
      lands past the pointer rather than being clipped. The one edge
      case is a pointer within 8px of the output's right edge, where
      it would run off; the tray never sits that far over.
    - Neokitsch open-row (submenu parent) box — *found 2026-09-05
      while measuring the overshoot; resolved the same day, and the
      SVG path was the wrong half*. `bar.svg` drew the Devices outline
      at x 1287..1445.2 (line 335) while its own prose called that "6
      from the rings' inner edge", which is 1461.2 — the number the
      separator rule on line 341 uses, and what the code gives both
      (`open_inset.1` and the rule's own inset; the golden measured
      the rule 1460..1461 and the open row 1458.5). `git log -p
      515c2a0` settled it: 1445.2 is the pre-nesting 1458 shifted by
      the 12.8 the rings moved, from when the rows began at y 37
      inside the panel's 22px chamfer and the box had to stop at the
      chamfer's foot. The rows now start below the innermost ring's
      9.2px chamfer, which clears 1461.2 by 3px, so the constraint is
      gone. Box, tab and the `<` glyph moved in `bar.svg` (path and
      prose); `neokitsch.rs` unchanged. One thing the prose said that
      the code did not do: "the "<" glyph right-aligned … so it clears
      the tab" — the marker sat under the tab's footprint.
      `bar::menu_row` now pads the trailing marker by `tab.base +
      tab.inset + icon_gap` (38px) on an open face that carries a tab;
      only neokitsch's does, the other three bars are byte-identical.
      Golden bar-neokitsch re-taken (99.994% against the previous);
      G2i neokitsch bar still the accepted 14%.
    - ~~Neokitsch haze blue annulus — *still open*.~~ Landed
      2026-09-05 with the haze unification (the `[x]` item under
      "SVG→iced pre-work", findings that outlive the wave): the
      strip's ground is the dashboard's `HUB_GROUND` composited by
      `soft.rs`, blue and mask included, so the cast on the last
      ~150px is there (x 1520 #272b42 vs the trace's #292e41; x 1599
      exact). It had been a faint blue cast plus a mask-faded arc on
      the left that alpha-stop annuli with a horizontal fade could not
      draw without even-odd ring paths sliced in x.
    - Found on the way, **not fixed** (theme, not crate): entropism's
      published `tape` is `#9cb795` (`home/themes/entropism/palettes.nix`
      nexus), the *selection* sage, so on the live bar the host tape
      reads as a second selected cell. The crate's own palette and
      `bar.svg` want MID `#728f76` ("the dimmer of the two sage fills
      so it does not read as a selection next to workspace 3"). The
      goldens carry the theme's value. Filed under "Live inconsistency"
      below.
    - Goldens bar-{entropism,neomil,kitsch,neokitsch} re-taken (AE 0 vs
      host renders); matrix 21/21.
- [x] Stale claims — the four era `README.md`s were rewritten
  2026-09-03 (dashboard composites no longer described as real;
  numbers now point at the trace headers). Swept 2026-09-04 against
  the tree: the `Layout` fold had already taken most of the list with
  it (`OpsCharts`/`TileRow` are gone from `style.rs`, `charts.rs` is
  gone, entropism.rs :4-7 and kitsch.rs's overhang comment were
  corrected in place, the crate README's neokitsch double-gold frame
  is described as the invention it was). What was still wrong and is
  now fixed: `src/theme.rs` :7,10,189 and `src/eras/neomil.rs` :12
  cited `src/colors.rs`, deleted in `61db2ae` (now say so and point
  at `Theme::fallback`); the crate README said dashboard blocks could
  still be `&[]` (all four are transcribed) and listed a neokitsch
  BASKET panel "on the mailbox footer" as not yet done — no trace
  puts BASKET on the mailbox (it is the store's plate, drawn), and the
  mailbox notch it also named is `sel_notch`, drawn since 2026-09-03.
  `docs/sources.md` :26-29 is not stale: it says the READMEs' old run
  numbers are shifted by ten, which is still the fact.
  Unverified README claims the docs pass left in place: kitsch bezel
  #f08c1e and #fcbb15, neokitsch #c78948, the neomil `dashboard.svg`
  wordmark vs `dashboard.rs:225-234` "wordmark" (moot: the SVG is
  deleted), the entropism desktop-theme paragraph. The
  `target-components.svg` inventories are moot too: the sheets were
  rebuilt as `components.svg` 2026-09-03 and the README entries
  rewritten from the new sheets.
