# Kitsch design targets

Sampled, second attempt. The first version of these mockups was drawn
from the era's *description* (maximalist → gilded, damask, filigree)
and looked nothing like the source. These are drawn from the actual
Behance references.

**Which file is authoritative: read `docs/PIPELINE.md` and
`docs/sources.md` first.** The four `*-trace.svg` — `login-trace.svg`,
`dashboard-trace.svg`, `mailbox-trace.svg`, `store-trace.svg` — are
measured schematics of the four sourced screens, each gated by
`scripts/fidelity_check.sh --inventory kitsch <screen>`, and each
carries a header comment narrating its source region by region with
measurements. Read those headers, not this file, for geometry. The
app-shaped `target-app.svg` and `dashboard.svg` compositions that
predated them were deleted 2026-09-03; the notes at the end say what
they were. `components.svg` is the widget sheet, rebuilt from the traces
the same day.

## Provenance

The Part 1 gallery (`Cyberpunk 2077 User Interface (Part 1)`,
Vilimovský) holds 176 module images. The four era explorations are
contiguous runs of nine screens, each opening with a title card.
`docs/sources.md` § "Run recovery, canonical positions" is canonical;
the positions this file used to give (kitsch at doc #33–42, and the
same shift for the other three) came from an earlier, smaller scrape
and are ten low:

| era | title card at | screens |
|---|---|---|
| Entropism | 33 | 34–42 |
| **Kitsch** | 43 | 44–52 |
| Neo Militarism | — (opens black) | 53–62 |
| Neo Kitsch | 63 | 64–72 |

The kitsch run: the **module hub** (#49, and #46 the same screen with
presentation annotations, #45 the fan laid nearly flat), the guest
login (#50), two shots of the UI on a wall-mounted screen pair (#47,
#48), the mail screen (#51) and the 4ST store (#52). `docs/sources.md`
used to assert that no screen in this run was a hub; that was written
without opening the images and is false — #49 is the hub, and
`dashboard-trace.svg` traces it.

**Attribution warning:** aesthetic priors are unreliable here. The
pink/teal/yellow system is **kitsch**; the champagne-gold-on-black
system with wood-veneer card fills that *looks* gaudy-luxurious is
**neokitsch**. The first draft of these mockups had them inverted.
Anyone drawing neokitsch next: its run is screens #64–72, gold outline
cards with clipped corners over a violet haze.

## Sampled palette

Sampled off the reference and carried as the era consts in
`src/eras/kitsch.rs`:

```
bg           #0b0b07     warm near-black
bloom        #a63355 → #6c1c3d   rose radial
teal         #7ddec8     strokes, titles, product art
teal solid   #1cb39b     wave, chips, PROTECTED bars
mint         #87f4d9     stat-highlight fill, the login bracket
yellow       #fcc428 / #fcbb15   shelf bands, selection fills
on-yellow    #37220f
bezel        #f08c1e     rounded CRT frame on device screens
```

The traces sample the same families off the photos (see each trace
header's palette block, and `bar.svg`'s header for the const-by-const
mapping): bracket and line-work peak around `#80e4d0`, the wave reads
`#1bb6a3`/`#1db5a4`, the SMG and DETAILS selections `#ffbe18`/`#e6c020`,
the amber `02` badge `#f0a93c`.

Role mapping: `bg`=bg, `panel`=the bloom field, `fg`=teal, `border`=the
outline teal `#5fd6c2` the store and mailbox traces sample off frames,
`alert`=yellow, `tape`=bezel orange. Note the inversion: in kitsch,
yellow is *selection*, not alarm.

## Observed era rules

- Rounded corners everywhere — **but not "no chamfers"**. The traces
  measure chamfers on most solid or outlined bodies: the store card's
  24px top-right chamfer and its shelf-band's bottom-right, the
  mailbox's two-piece selected row (icon cell chamfered bottom-right,
  body chamfered at its right end), the message tab's top-right, and
  the chevron tabs' chamfered right ends.
- Teal line-work carries most of the structure. Yellow means "the
  selected thing", but it is not always a *fill*: the mailbox message
  panel is a yellow outline under a solid yellow tab, and the store's
  grown card is amber-filled above and amber-*outlined* below.
- One solid teal wave (the "page-curl") at the foot of the container
  outline on the mail and store screens, where the left edge of the
  bracket forks into it; the login's bracket instead encloses a dark-teal
  lobe outside its diagonal, and the hub has no bracket at all.
- Card shelf-bands poke past the card's left edge as a flag and carry
  compliance glyphs plus a brand tag (PETROCHEM in a dark box,
  BETTERLIFE TEC at the right).
- 3D slabs (fan menus) get stacked-outline extrusion. All six blades'
  ghost stacks recede in **one shared screen-space direction**
  (up-right), not along each card's own normal.
- A rose bloom sits over the **top of every screen, brightest at top
  centre** and gone by y~420 — not a corner vignette. The far left of
  the frame reads grey-green rather than black on the login, mailbox
  and store.
- Tiny dim-teal captions everywhere; boxed A/B/C footnote markers. The
  single centred line at the foot of every screen is *bright mint*, not
  dim teal (all four trace headers record this; an earlier drawing had
  it dim, and the hub's had it yellow).
- Device screens sit inside an orange rounded bezel.

## Motion

Boot-ins, annotated 2026-09-07 as SMIL on the traces under the
`docs/PIPELINE.md` § "Motion" convention (the rest frame is the trace;
rsvg and every static gate see nothing). Kitsch comes up as **depth**:
its one signature the other eras lack is the extrusion — every fan
blade trails a ghost stack receding (+20,−20) up-right from its solid
face, and run image #45 shows the fan as a physical rotor laid on a
table. So the solid thing (the blades; the nav with SMG selected; the
selected mail row) stands from frame 0, and what belongs to it
*extrudes out to its right* under a width-growing `<clipPath>` —
never neomil's top-down panel wipe or neokitsch's fade. Every
transition is 0.45 s EaseOutCubic (`keySplines="0.33 1 0.68 1"`),
`fill="freeze"`, frozen well before `motion::REST` (2.4 s):

| id | trace | group | clip rect (rest) | animates | begin |
|---|---|---|---|---|---|
| `#fan-left-extrude` | `dashboard-trace.svg` | the left fan's ghosts (VEHICLES, WEAPONS, left PRODUCTS) | x 296 y 206 w 484 h 436 | width 0 → 484 | 0 s |
| `#fan-right-extrude` | `dashboard-trace.svg` | the right fan's ghosts (right PRODUCTS, EVENTS, LOCATIONS) | x 663 y 217 w 465 h 418 | width 0 → 465 | 0 s |
| `#panel-extrude` | `dashboard-trace.svg` | the BRAINDANCE panel (tab, tape, body, paragraphs) | x 1166 y 255 w 272 h 418 | width 0 → 272, 0.35 s, with a `<set>` hold | 0.25 s |
| `#cards-extrude` | `store-trace.svg` | the four product cards | x 450 y 210 w 1150 h 505 | width 0 → 1150 | 0 s |
| `#message-extrude` | `mailbox-trace.svg` | the message panel and the DETAILS / MODS / PRICE / DAMAGE tabs | x 534 y 300 w 850 h 456 | width 0 → 850 | 0 s |

Each `<animate>` carries a comment in its trace saying what moves and
why. Verified 2026-09-07: rsvg renders of each trace before and after
the annotation differ on 0 pixels, and `scripts/frame.sh --at 2.4` of
the annotated trace equals the same frame of the unannotated one on 0
pixels (no fuzz), so no clip nicks a ghost stack or a stroke at rest.
`frame.sh --at 0.08 / 0.2 / 0.4 kitsch dashboard`, `--at 0.1 / 0.25`
for the store and mailbox, are the moments to look at.

Not yet transcribed into `src/eras/kitsch.rs`. One thing the coding
side will meet: the hub's ghosts sit in `HUB_BACK`, one `Prim::Soft`
group with the ground and bloom, and `screens/soft.rs` takes no
`Prim::Motion` inside a `Soft` group (a composited group is rasterised
once and cached), so the two fans' ghosts need their own group(s) —
each clip rect above contains its fan's whole stack, as `PIPELINE.md`
asks.

## Hover and press

**Not sourced.** Read 2026-09-07 for the crate TODO's "what a hover
*is* per era" question: all nine stills of the run (`images/run-kitsch/`
#44–52) and the four full-res screens, cropped to the nav column, the
cards, the mail rows and tabs, the login field and bars and both fans.
No still carries a cursor, and every unselected sibling is drawn
identically to its neighbours (RIFLES / SNIPER / SHOTGUN / PISTOL are
four equal outlines, MODS / PRICE / DAMAGE three, mail rows 2–5 four,
cards 1 / 3 / 4 three, badges 01 / 03 / 04 three). The states the
material carries are **rest**, **selected** (yellow), **disabled**
(PROTECTED) and, on the field, **focus** (the caret in #50). Hover and
press are therefore *inferred*, and `components.svg` band 11 says so on
every group and caption.

**The reading**, derived from the era's own two emphasis devices — depth
and fill. The hub extrudes every blade as a ghost stack stepping
(+20,−20) in screen space, and #45 (`45-546b0111`, the fan laid flat)
renders the same fan as physical slabs standing on a table with the
*selected* one standing tallest: sampled down column x 470 of the
thumbnail, NETWORK's yellow face runs y 336..370 and its body fades out
over y 372..426 (54 px); down x 700, EVENTS' teal face runs y 320..358
and its body over y 362..398 (36 px), so the chosen slab stands about
1.5x taller than its neighbours. Selection, meanwhile, never moves or
grows a cell: it keeps the silhouette and swaps outline for fill (era
rule 5). Hence:

- **Hover = lift.** The cell keeps its silhouette exactly; where it was
  an outline its face becomes the fan blade's idle slab (fill `#2c9798`,
  stroke `#a9e6df` 1.8, ink `#123c38` — the one measured solid-teal
  face), and *one* ghost appears behind it at (+20,−20), the first step
  of the measured ramp (fill `#0f9f80` at .58, stroke `#6cc4bd` 1.2 at
  .80). A face that is already solid (the mint ENTER bar, the dark field
  plate) keeps its fill and only gains the ghost, as EVENTS keeps its
  teal ghosts under a yellow face. On the 216x39 nav the ghost's top
  lands 1 px under the cell above (60 px pitch, 21 px gap); on the
  53.5-pitch tabs it runs 12.5 px under the neighbour, as the fans do.
- **Press = the selection arriving, flat.** The ghost collapses and the
  face takes the element's selected fill with no ghost (`#ffbe18` /
  ink `#5a3a08` on the chevron and the button, `#e8c21f` / `#4a3a05` on
  the row): the slab pushed into the surface, and the yellow is what it
  leaves behind. For a cell that becomes the selection nothing changes
  on release. On the field, press is **focus**, and that one *is*
  sourced: the plate gains the 2x22 `#8af0d8` caret (`#caret-blink`).
- **Rest** is sourced for all four (ENTER #50, field plate #50, RIFLES
  #52, mail row 2 #51), except that the field's caret-*less* state is
  itself inferred: #50 shows the field focused only.

Groups on the sheet: `k-button-{rest,hover,press}`,
`k-field-{rest,hover,press}`, `k-chevron-{rest,hover,press}`,
`k-row-{rest,hover,press}`, each with an XML comment giving the still,
the photo region and the sourced/inferred flag. The canvas grew from
1080 to 1392 for the band; the top 1080 rows render pixel-identical to
before. The tab chevron (161x46) and the fan blade take the same three
states and are not drawn separately. Not yet plumbed: `catalog` still
gives buttons and fields no hover/press treatment, and the transitions
(lift in, collapse on press) are not annotated as SMIL anywhere — this
is the destination design only, per `PIPELINE.md` § "Motion".

## What this does to the crate decision

The sampled kitsch **weakens** the ornament worry recorded in the
repo TODO. Real kitsch is not additive filigree — it is rounded
silhouettes, solid fills, one wave motif, and a bloom background. All
of that is parameterisable: corner radius, a curl decoration, a
background treatment. The genuinely new widget is the extruded fan
menu, which is an interaction-model difference (like neomil's services
table) — a per-era widget module, not a crate boundary.

The heavy-ornament question does not disappear; it moves to
**neokitsch**, whose selected cards are filled with a wood-veneer
*texture* — the first raster asset in any era. That, not gilding, is
the thing the toolkit abstraction should be tested against.

## Files

- `login-trace.svg` — `images/kitsch-login.png` (#50): the bloom, the
  clock, the full-height mint bracket whose left edge breaks into a
  diagonal and rounds into a bottom edge, three GUEST 7702 cards (chip
  glyph, name, boxed A + micro-text) with card 1 carrying an input
  field and a solid mint stepped ENTER bar while cards 2 and 3 read
  PROTECTED in dark teal, a barcode in the bracket foot, and the mint
  footer line. Gate: PASS, inks 0.63 (third pass 2026-09-03; was 0.54).
- `dashboard-trace.svg` — `images/kitsch-dashboard.png` (#49), the
  **module hub**: boxed A USER / C SECURITY LEVEL / D DESCRIPTION
  headers, a notched GUES 7702 box, four badges with the second filled
  amber, **two three-blade fans** of rounded cards about two hubs
  (VEHICLES, WEAPONS, PRODUCTS / PRODUCTS, EVENTS, LOCATIONS) with
  EVENTS solid yellow as the selection and a ghost stack behind each
  blade, and the BRAINDANCE detail panel under a yellow header tab.
  Gate: PASS, inks 0.69 (third pass 2026-09-03; was 0.59).
- `mailbox-trace.svg` — `images/kitsch-mail.png` (#51): the header's
  three boxed letters with stepped USER / DESCRIPTION boxes and the
  four security badges, the list bracket forking into its solid teal
  wave, five message rows with the first selected as a two-piece solid
  yellow row, the yellow message panel (solid tab, flag band, outlined
  body, three lorem paragraphs) and four chevron tabs at the right with
  DETAILS solid. Gate: PASS, inks 0.67 (third pass 2026-09-03; was 0.62).
- `store-trace.svg` — `images/kitsch-store.png` (#52): the logotype
  block, the customer chip and loyalty lines, the nav bracket wrapping
  the customer block and ending in the wave, five chevron nav tabs with
  SMG solid yellow, and four product cards (yellow flag band, mint gun
  drawing, stat row, solid mint values bar, socket row) with the second
  amber-filled, grown, and continued in an amber outline. Gate: PASS,
  inks 0.73 (third pass 2026-09-03; was 0.57). Supersedes `target-app.svg`.
- `components.svg` — the widget sheet, rebuilt 2026-09-03 from the
  four traces and `bar.svg` (it was `target-components.svg`, drawn by
  eye: 160-wide rounded pills where the traces have 216x39 peaked
  chevrons, a 20px band with no flag where the traces have 35px with a
  27px flag, extruded slabs and an orange device bezel that no traced
  screen has). Every component is a translate-only copy of a trace
  element, cited to file and coordinates in an XML comment and
  captioned on-sheet: the 4ST logotype, customer chip, letter boxes,
  the stepped USER box, 01–04 badges, the full nav container (bracket,
  wave, five chevrons) plus chevrons and tabs isolated in both states,
  the fan blade and ghost stack idle and selected, mail rows, the
  product card plain and grown, the BRAINDANCE and message panels, the
  guest card, input field, ENTER/PROTECTED bars, barcode and socket
  glyph, then the sampled palette, typography, ground, observed era
  rules and an implementation-delta box listing where
  `src/eras/kitsch.rs` still disagrees with the traces (Round 16,
  "no chamfers", Ticket 18/15, Banner 12/8, top-right Bloom, stroke
  1.5, SLAB/BEZEL, YELLOW_SHADE unsampled). Band 11 (2026-09-07, canvas
  1920x1392) adds rest / hover / press siblings for the button, field,
  nav chevron and list row — see "Hover and press" above for what is
  sourced and what is inferred. Not gated — the traces are.
- `bar.svg` — the status bar: host tape, workspaces, tray, the
  wired/audio/CPU/MEM modules and the clock, at the 1600x220 geometry
  the bar golden tests render. The bar has no photo source, so this is
  an original composition, redrawn 2026-09-02 from the four traces'
  chrome (chevron workspaces, stepped USER/DESCRIPTION boxes, the mint
  bracket fading along the bar foot, the teal wave in the menu foot).
  **It is no longer "exactly as `bar()` composes it"**: it is the
  design target and `bar.rs` has not followed yet (crate TODO.md §
  "Bar restyle"), so read the SVG's IMPLEMENTATION DELTA block, not the
  current render.

## Deleted composites (2026-09-03)

Two app-shaped drawings used to sit beside the traces; `docs/sources.md`
keeps a row per file saying what each got wrong. In short:

- `target-app.svg` — the 4ST store as a loose composite, superseded by
  `store-trace.svg`. The closest of the four `target-app` files, but
  its nav rows were rounded pills where the photo has 216x39 chevrons
  above the teal wave, and its band was half height with no flag notch
  — `src/style.rs` `Ticket` and `Banner` still carry those numbers
  (the open kitsch item in the crate TODO.md).
- `dashboard.svg` — the six-module hub composite `screens::dashboard`
  assembled under `Layout::ModuleHub` until the fold of 2026-09-03 late (the screen is now a `Prim` table transcribed from the trace, G2i 31% — a layout match by eye; the number is renderer alpha-blending plus extractor hole-fill, see the crate TODO.md). Drawn before the hub
  screen was found: its fan widget came from the fan scenes but its
  chrome matched the app, and it scored 0.07 on the ink gate against
  `kitsch-dashboard.png` (the trace scores 0.59). Until the `Layout`
  decision in the crate TODO.md the dashboard screen has no SVG that
  agrees with it — G2i now compares it against the trace.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/kitsch-bar.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 900 dashboard-trace.svg -o /tmp/kitsch-dash.png
```

Render with Rajdhani on fontconfig:

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 store-trace.svg -o /tmp/kitsch-store.png
```
