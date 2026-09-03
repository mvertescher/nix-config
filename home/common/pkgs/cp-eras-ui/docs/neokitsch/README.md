# Neokitsch design targets

Sampled from the Behance Part 1 neokitsch run: the nine screens after
the "NEO KITSCH — SUBSTANCE AND STYLE" title card — the ARASAKA login,
the module hub, the mail screen and the 4ST store. An earlier revision
numbered the run "doc #53–62"; those positions come from a smaller
scrape and are ten low. `docs/sources.md` § "Run recovery, canonical
positions" is canonical: title card 63, screens 64–72. See
`../kitsch/README.md` for how the four era runs were recovered and for
the attribution warning: this champagne-gold-on-black system is
neokitsch; the pink/teal/yellow one is kitsch, not the reverse.

**Which file is authoritative: read `docs/PIPELINE.md` and
`docs/sources.md` first.** The four `*-trace.svg` — `login-trace.svg`,
`dashboard-trace.svg`, `mailbox-trace.svg`, `store-trace.svg` — are
measured schematics of the four sourced screens, each gated by
`scripts/fidelity_check.sh --inventory neokitsch <screen>`, and each
carries a header comment narrating its source region by region with
measurements. Read those headers, not this file, for geometry. The
app-shaped `target-app.svg` and `dashboard.svg` compositions that
predated them were deleted 2026-09-03; the notes at the end say what
they were. `target-components.svg` remains, by-eye and unverified.

## Sampled palette

Sampled off the reference and carried as the era consts in
`src/eras/neokitsch.rs`:

```
bg          #0a0a0a       true black outside the frame
bloom       #34344c mid   violet haze, top-centre
frame       #916424 outer / #5e3414 inner
gold text   #e7c686       logotype, headings
champagne   #d3b279       bands, secondary text
veneer      #f4c474 → #d8a558   wood-grain fill on selected elements
amber CTA   #fcc474 → #c78948   ENTER / LOGIN bars
field       #2c1c14       input fills
strata      #634427       fine-line layered dividers
```

The traces sample the photos' gold as three tiers rather than one —
bright bars and tabs (`#f5c379`/`#f0bf7b`), mid text cores and veneer
grain (`#c19867`), dark outlines and strands (`#c5965a`) — plus a
smeared glow family (`#38261a`/`#39281b`) that is as much ink as the
gold itself. Note `FRAME_LIT #c69a55`: the source's button outlines
sample there, and never at `FRAME #916424` (`bar.svg`'s header records
the const-by-const mapping).

Role mapping: `bg`=bg, `panel`=bloom field, `border`=frame gold,
`fg`=gold text, `dim`=#8a7048, `tape`=veneer.

## Observed era rules

- Gold line-work on black under a violet haze; quieter than kitsch —
  no page-curl, no shelf bands, far fewer captions.
- **There is no device frame.** The old rule here ("the device frame is
  part of the UI: double gold stroke, stepped corner tabs top and
  bottom, strata wedge at the foot") describes an invention of
  the since-deleted `target-app.svg` composite; the photos have no
  full-screen frame. The era's
  actual chrome is the **stacked-hairline wire band**: many fine gold
  strands running in plateaus and rising through mirrored S-bends onto
  a single bridging line — a wide trapezoid across the foot of the
  login, and a header band on the hub, mail and store screens.
- **Selection is a material, not a colour**: the chosen button, card or
  mail row is filled with wood veneer — fine wavy grain lines over a
  light gold base, chevroning into a book-match seam and arcing into
  the chamfered corner. In these SVGs it is a fill plus grain strokes;
  a real implementation should treat it as a texture asset — the first
  raster asset in any era, and the stress test the repo TODO flags for
  the toolkit abstraction.
- Corners are cut per widget rather than by one rule: the hub's cascade
  cards chamfer top-right *and* bottom-left, the store's product cards
  round their top-left and top-right around a stepped top edge and cut
  the bottom-left, and the nav and RIFLES buttons take a small
  bottom-left cut. Product cards carry their name at the card's foot,
  not in a header, with a solid tab under the bottom edge.
- Outlines come in onion layers: the hub's cascade cards and detail
  panel trail concentric outline copies, and the store's cards are
  shadowed by four fading echo strands round the step, the top-right,
  the right side and the bottom.
- Boxed letter markers (A/B/C/D) sit beside the wire band and in the
  foot, as small rounded plates with a folded corner.
- The security-level badge is a ringed folder tab, outlined rather than
  filled, with T2 carrying the tab; the basket is a solid veneer plate
  with its bottom-left corner cut and a hairline splitting it.
- Solid gold bars with a bottom-left cut are the only strong CTAs (the
  SVGs draw them as an amber gradient; the photos read as a flat fill).
- Every glyph and stroke sits on a soft vertically-smeared glow — all
  four traces draw it once as a blurred copy of the content group under
  the crisp content.

## Files

- `login-trace.svg` — `images/neokitsch-login.png` (#70): the ARASAKA
  stencil logotype with its tagline and two-cell box, the clock and
  NIGHT CITY / AREA at the right, two identical PRASE_6054012 entry
  groups (label, unoutlined chocolate field, solid gold ENTER / LOGIN
  bar with a bottom-left cut, letter box and micro-text), the wire band
  as a wide trapezoid across the foot, and a centred footer line.
  Nothing else — the screen is sparse by design. Gate: PASS, inks 0.73.
- `dashboard-trace.svg` — `images/neokitsch-dashboard.png` (#69), the
  **module hub**: the header (CUSTOMER / LEVEL T1, SECURITY LEVEL
  T1–T4 with T2 as a ringed badge, the wire band with boxed A and B),
  **six cascade cards in two staircase triplets** (EMAIL solid gold as
  the selection, then MATRIX, BRAINDANCE, PRIVATE, SECURITY SYSTEMS,
  DEVICES) each trailing concentric onion outlines, a mirrored detail
  panel with a solid gold body, and boxed C and D in the foot. Gate:
  PASS, inks 0.60.
- `mailbox-trace.svg` — `images/neokitsch-mail.png` (#71): the hub's
  header block verbatim; a seven-row message list with a rule and small
  tab under each row and row 2 the selection as a wood-veneer bar with
  a top-right chamfer and a notched bottom edge; the message as **plain
  text with no panel outline**; four outlined RIFLES buttons with a
  bottom-left chamfer and a filled tab; boxed C and D in the foot; no
  footer line. Gate: PASS, inks 0.68.
- `store-trace.svg` — `images/neokitsch-store.png` (#72): the 4ST
  logotype over S T O R E, the BASKET plate at the top right, the
  header wire band bridging the width with boxed A and C beside it, the
  customer / loyalty / last-update lines, five outlined nav buttons with
  SMG the veneer selection, and four weapon cards with echo strands,
  the second expanded and solid gold across its middle. Gate: PASS,
  inks 0.60. Supersedes `target-app.svg`.
- `target-components.svg` — folder-tabs, strata divider, login field
  and CTA, nav pills, step-notch pill, basket panel, product card in
  both states, card cascade, mail list, detail text, the device frame
  in miniature (an invention — see the era rules above), sampled
  swatches.
- `bar.svg` — the status bar: host tape, workspaces, tray, the
  wired/audio/CPU/MEM modules and the clock, at the 1600x220 geometry
  the bar golden tests render. The bar has no photo source, so this is
  an original composition, redrawn 2026-09-02 from the four traces'
  chrome (the haze clipped to the strip, r3 cells with a bottom-left
  cut and a veneer tab, veneer + grain + seam for selection, the wire
  band bridging the centre gap). **It is no longer "exactly as `bar()`
  composes it"**: it is the design target and `bar.rs` has not followed
  yet (crate TODO.md § "Bar restyle"), so read the SVG's
  IMPLEMENTATION DELTA block, not the current render.

## Deleted composites (2026-09-03)

Two app-shaped drawings used to sit beside the traces; `docs/sources.md`
keeps a row per file saying what each got wrong. In short:

- `target-app.svg` — the 4ST store as a loose composite, superseded by
  `store-trace.svg`. Right screen, but it invented the full-screen
  chamfered device frame (`widgets::chrome::DeviceFrame` was built to
  it and is still worn by the widget dashboard and the bar's mail
  panel), lacked the onion outlines that are the era's signature, and
  borrowed kitsch's ARASAKA footer.
- `dashboard.svg` — the six-module hub composite `screens::dashboard`
  still assembles under `Layout::ModuleHub`. Its cascade widget was
  credited to a screen that is actually a login, the real hub went
  unread, and it scored 0.03 on the ink gate against
  `neokitsch-dashboard.png` (the trace scores 0.60). Until the `Layout`
  decision in the crate TODO.md the dashboard screen has no SVG that
  agrees with it — G2i now compares it against the trace.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/nk-bar.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 900 dashboard-trace.svg -o /tmp/nk-dash.png
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 store-trace.svg -o /tmp/nk-store.png
```
