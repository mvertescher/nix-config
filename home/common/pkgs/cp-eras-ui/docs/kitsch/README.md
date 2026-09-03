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
they were. `target-components.svg` remains, by-eye and unverified.

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

Role mapping: `bg`=bg, `panel`=the bloom field, `border`/`fg`=teal,
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
  footer line. Gate: PASS, inks 0.54.
- `dashboard-trace.svg` — `images/kitsch-dashboard.png` (#49), the
  **module hub**: boxed A USER / C SECURITY LEVEL / D DESCRIPTION
  headers, a notched GUES 7702 box, four badges with the second filled
  amber, **two three-blade fans** of rounded cards about two hubs
  (VEHICLES, WEAPONS, PRODUCTS / PRODUCTS, EVENTS, LOCATIONS) with
  EVENTS solid yellow as the selection and a ghost stack behind each
  blade, and the BRAINDANCE detail panel under a yellow header tab.
  Gate: PASS, inks 0.59.
- `mailbox-trace.svg` — `images/kitsch-mail.png` (#51): the header's
  three boxed letters with stepped USER / DESCRIPTION boxes and the
  four security badges, the list bracket forking into its solid teal
  wave, five message rows with the first selected as a two-piece solid
  yellow row, the yellow message panel (solid tab, flag band, outlined
  body, three lorem paragraphs) and four chevron tabs at the right with
  DETAILS solid. Gate: PASS, inks 0.62.
- `store-trace.svg` — `images/kitsch-store.png` (#52): the logotype
  block, the customer chip and loyalty lines, the nav bracket wrapping
  the customer block and ending in the wave, five chevron nav tabs with
  SMG solid yellow, and four product cards (yellow flag band, mint gun
  drawing, stat row, solid mint values bar, socket row) with the second
  amber-filled, grown, and continued in an amber outline. Gate: PASS,
  inks 0.57. Supersedes `target-app.svg`.
- `target-components.svg` — the widget sheet: ticket-pill nav with
  page-curl container, product card in both states, fan menu, callout
  panel, mail list, device bezel, guest card, security strip, sampled
  swatches. Sampled across the run.
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
  still assembles under `Layout::ModuleHub`. Drawn before the hub
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
