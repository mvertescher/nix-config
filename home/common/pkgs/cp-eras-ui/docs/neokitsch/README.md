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
they were. `components.svg` is the widget sheet, rebuilt from the traces
the same day.

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
  cards chamfer top-right and run a 45-degree diagonal across the whole
  foot (re-measured 2026-09-03; the traces used to draw it as a second
  chamfer), the store's product cards
  round their top-left and top-right around a stepped top edge and cut
  the bottom-left, and the nav and RIFLES buttons take a small
  bottom-left cut. Product cards carry their name at the card's foot,
  not in a header, with a solid tab under the bottom edge.
- Outlines come in onion layers: the hub's cascade cards and detail
  panel carry echo outlines (nested inside the card edge on the hub,
  stepping down and inward as they fade), and the store's cards are
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

## Motion

Boot-ins, annotated on the traces as SMIL per `docs/PIPELINE.md`
§ "Motion" (the rest frame is the trace; rsvg draws nothing of this,
`scripts/frame.sh --at <s>` does). The era comes up one way on every
screen: the plated or cascaded elements are *wiped on from the left*
in reading order (a `<clipPath>` whose rect grows in width, 0.33 1 0.68
1 = EaseOutCubic), and the one solid veneer body on the screen then
*fades* in (opacity 0 -> 1 over 0.3 s from 0.4 s, 0.61 1 0.88 1 =
EaseOut, with a `<set>` hold before it begins). Everything is frozen by
0.7 s. The login has no motion: its field draws no caret in the photo.

| screen | id | what | trace element(s) | timing |
|---|---|---|---|---|
| dashboard | `#cards-open` | wipe | the six cascade cards, labels, captions; clip rect (180,160) 760x630 | 0 s, 0.5 s |
| dashboard | `#panel-fade` | fade | the detail panel group (rings, outline, body and its grain, paragraph text, tape, label) | 0.4 s, 0.3 s |
| store | `#shelf-open` | wipe | the four weapon cards in `#lines` and their labels in `#text` (one clipPath, two groups); clip rect (340,205) 1240x530 | 0 s, 0.5 s |
| store | `#body-fade` + `#body-fade-text` | fade | card 2's gold body: fill, grain, socket rules, QR (`#lines`) and its dark printing (`#text`) — one fade, two `<animate>`s because glyphs are haloed separately | 0.4 s, 0.3 s |
| mailbox | `#list-open` | wipe | the seven rows: rules and tabs, envelopes (`#lines`), titles and FROM: lines (`#text`); clip rect (15,240) 520x460 | 0 s, 0.5 s |
| mailbox | `#bar-fade` + `#bar-fade-text` | fade | the veneer selection bar, its grain, the inverted tab and outline, the dark envelope (`#lines`) and the dark title / FROM: MOM (`#text`) | 0.4 s, 0.3 s |
| mailbox | `#buttons-open` | wipe | the four RIFLES buttons (`#lines`) and labels (`#text`); clip rect (715,660) 810x90, held at 0 until it begins | 0.3 s, 0.4 s |

The store's and mailbox's fading bodies were moved out of the wipe
groups in the trace (painted after / before them, over elements they
do not overlap, so the rest pixels are unchanged) so that no fade nests
inside a wipe. Each pair of `*-fade` / `*-fade-text` is one
`Prim::Motion` on the iced side; the per-prim ink fade shows the
fill / grain / glyph stack for those 0.3 s, as the dashboard's panel
does. Verified 2026-09-07: rsvg renders of each trace before and after
annotating differ by 0 pixels, and `frame.sh --at 2.4` of each matches
the unannotated file's frame at 8-level fuzz on 0 pixels (the halos
are inside every clip).

## Hover and press

Read 2026-09-07 from the whole run, still by still: `images/run-neokitsch/`
64 (mood board, no UI), 65 (login over the circuit ground), 66 (the
annotated hub), 67 and 68 (wall pairs login + store, hub + mail),
69..72 (the four screens), plus the four 3840x2160 masters
`neokitsch-dashboard.png` / `-login.png` / `-mail.png` / `-store.png`
cropped at 1:1 and 2:1 on every RIFLES button, store nav cell, product
card, mail row, login entry, T1..T4 badge and cascade card, and on the
ground between them. Drawn as band 11 of `components.svg`
(`nk-button-{rest,hover,press}`, `nk-field-{rest,hover,press}`,
`nk-row-{rest,hover,press}`, `nk-card-{rest,hover,press}`, each
group's comment citing the trace and still or saying INFERRED).

**The run shows no hover and no press.** No still carries a pointer, a
held cell or a second ink on any sibling: the four RIFLES buttons on
#71 measure the same to a quarter of a level, the store's nav cells
and cards 1 / 3 / 4 are one outline, the five unselected cascade cards
one ring stack, both login entries identical on #65, #67 and #70 (flat
chocolate, sd < 2.1: no caret, no text). The one cell per list that
differs is the **selection**, and it always differs the same way — the
veneer (SMG, mail row 2, EMAIL, product card 2, the panel body), or on
the badge row an outline with echo rings (T2). Neokitsch does *not*
have entropism's cursor tell: mail row 2, the veneer, is also the
message the panel shows (Urgent information (!) / FROM: MOM, on #71
and #68), and the envelope glyph is not tied to it (row 2 is closed;
rows 1, 3, 7 open). So every hover and the field's focus on the sheet
is **inferred**, every rest and press **sourced**.

**The reading: hover = echo, press = veneer.** The era draws emphasis
on one ladder, every rung sourced: plain type on the ground (T1, a
plain row); an outline with a bright tab (RIFLES, nav cell, product
card); the outline *echoed* as fading hairlines (cascade card, six
inside; product card, four; T2, seven *outside*, fanning up and right
— the only cell in the run marked against plain siblings by an echo
alone); the *veneer* — bright base, grain, book-match seam, outline
and rings dropped, tab kept, dark ink (SMG, row 2, EMAIL, card 2); and
above it all the flat amber CTA, which is an action, not a selection.
Hence:

- *Rest* — sourced for all four: RIFLES #71 (`nk-button-rest`), the
  login plate #70 (`nk-field-rest`, and neokitsch is the one era whose
  field is photographed at rest rather than focused), the plain mail
  row #71 drawn with row 2's content (`nk-row-rest`), MATRIX #69
  (`nk-card-rest`).
- *Hover* — inferred. The cell's edge echoes **outward on T2's
  recipe**: seven rings `#a97c48` 0.7, opacity 0.85 innermost to 0.55
  outermost, each ring 2.1 up / 1.6 right / 0.6 left / 0.3 down; face,
  tab, label unchanged. The button and the field take it as is; the
  plain row first gains the selection's own silhouette as an outline
  (`#e8c186` 1.1, the step T2 takes against T1) and echoes that. The
  cascade card already echoes and its neighbours sit 100 apart, so an
  outer fan would cross them: its rings and outline lift one tier of
  the three-tier gold instead (`#bd8951` → `#e8ab66`, `#e8ab66` →
  `#f2b463`). On the field the stack's top reaches the label 11 above
  the plate; the label is painted over the rings.
- *Press* — the veneer, the cell's sourced selected look spliced
  verbatim: SMG #72 on the button (as a press: inferred; release
  returns the outline), row 2 #71 on the row, EMAIL #69 with its 42
  grain strands on the card. A press that selects simply stays. The
  field takes no veneer (its plate is already a slot of the era's
  ink); its press is **focus**, inferred: a 2x24 caret in the label
  gold `#f5bf75` at the label's inset. Adopting that in the crate
  means a later vision pass adds `#caret-blink` to `login-trace.svg`,
  which has none because the photo shows none.

Not drawn, because nothing in the run shows it: a pointer glyph, a
brightened or thickened outline alone, an underline, a dimmer second
fill, any change of geometry (selection never moves a cell; card 2's
growth is its selection). The CTA bar, the store nav cell and the
product card take the button's pair (echo outside, then the SMG / card
2 veneer) and are not drawn separately. The canvas grew from 1080 to
1560 for the band; the top 1080 rows render pixel-identical to before.
Not yet plumbed: `catalog` gives neokitsch buttons and fields no
hover/press treatment, and the transitions (echo in, veneer poured)
are not annotated as SMIL anywhere — this is the destination design
only, per `PIPELINE.md` § "Motion".

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
  DEVICES) each carrying nested onion outlines, a detail panel with a
  **stepped top edge** (a shoulder at the left climbing through an
  S-curve to the top line — not the cascade card mirrored, which the
  first pass drew), four onion outlines nested *inside* it, a veneer
  body carrying two paragraphs of text (the inbox's selected message,
  re-wrapped to the panel's measured line ends), a two-line micro-text
  tape and the EMAIL label, and boxed C and D in the foot. Both solid
  fills, EMAIL's card and the panel body, carry wood-veneer grain since
  2026-09-07 (vertical strands at the photo's 2.1 / 2.7 pitches, EMAIL's
  swinging into its two book-match seams), drawn as the store's and
  mailbox's grain is; until then they were flat and the paragraphs,
  captions and tape were bars. Gate: PASS, inks 0.64 (0.68 with the
  bars: k-means bins the dark family by glyph area now, and the
  photo-vs-trace layout / edge correlations went 0.958 / 0.809 to
  0.980 / 0.942). G2i against the crate reads 34% by shape inventory
  since the grain (94% before, 89% with the panel grain hidden): the
  extractor segments a 2.7-pitch striped body one way from rsvg and
  another from iced although the two renders agree in mean and spread
  to half a level, so the number is the extractor's, not the screen's;
  the same pair by compare_ref is 0.999 / 0.989 / 0.869.
- `mailbox-trace.svg` — `images/neokitsch-mail.png` (#71): the hub's
  header block verbatim; a seven-row message list with a rule and small
  tab under each row and row 2 the selection as a wood-veneer bar with
  a top-right chamfer and a notched bottom edge; the message as **plain
  text with no panel outline**; four outlined RIFLES buttons with a
  bottom-left chamfer and a filled tab; boxed C and D in the foot; no
  footer line. Gate: PASS, inks 0.73.
- `store-trace.svg` — `images/neokitsch-store.png` (#72): the 4ST
  logotype over S T O R E, the BASKET plate at the top right, the
  header wire band bridging the width with boxed A and C beside it, the
  customer / loyalty / last-update lines, five outlined nav buttons with
  SMG the veneer selection, and four weapon cards with echo strands,
  the second expanded and solid gold across its middle. Gate: PASS,
  inks 0.66 (third pass 2026-09-03; was 0.60). Supersedes `target-app.svg`.
- `components.svg` — the widget sheet, rebuilt 2026-09-03 from the
  four traces and `bar.svg` (it was `target-components.svg`, drawn by
  eye: a device-frame miniature, a strata divider, step-notch pills and
  150x24 nav pills, none of which any trace contains). Trace geometry
  spliced in verbatim and translated only, each component cited to
  file and coordinates in an XML comment and captioned on-sheet: the
  ARASAKA and 4ST logotypes, the clock block, CUSTOMER/LEVEL, SECURITY
  LEVEL with the ringed T2 folder badge, the BASKET plate with grain
  and QR, letter box, the cascade card pair both states with onion
  rings, the store nav cell plain and veneer-selected, the mailbox
  RIFLES button, the product card plain and expanded (66 grain paths),
  the login entry group, hub detail panel, message panel, mail rows
  plain and veneer-selected, three 300px windows onto the wire bands,
  store meta lines, then 32 sampled palette values, typography, ground,
  observed era rules and an implementation-delta box listing where
  `src/eras/neokitsch.rs` still disagrees with the traces (DeviceFrame,
  ClipTopRight 30, FRAME/STRATA unsampled, Bloom, the "#54-62" doc).
  Band 11 (2026-09-07, canvas 1920x1560) adds rest / hover / press
  siblings for the RIFLES button, the login field, the mail row and
  the cascade card — see "Hover and press" above for what is sourced
  and what is inferred. Not gated — the traces are.
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
  assembled under `Layout::ModuleHub` until the fold of 2026-09-03 late (the screen is now a `Prim` table transcribed from the trace, G2i 92%). Its cascade widget was
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
