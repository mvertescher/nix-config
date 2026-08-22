# Kitsch design targets

Sampled, second attempt. The first version of these mockups was drawn
from the era's *description* (maximalist → gilded, damask, filigree)
and looked nothing like the source. These are drawn from the actual
Behance references.

## Provenance

The Part 1 gallery (`Cyberpunk 2077 User Interface (Part 1)`,
Vilimovský) contains 146 images. The four era explorations are
document-order runs of ten images each, opening with a title card:

| era | title card at | run |
|---|---|---|
| Entropism | doc #23 | 23–32 |
| **Kitsch** | doc #33 | 33–42 |
| Neo Militarism | doc #43 | 43–52 |
| Neo Kitsch | doc #53 | 53–62 |

The kitsch run: guest logins, extruded fan menus, the braindance
callout, the mailbox, and the 4ST store.

**Attribution warning:** aesthetic priors are unreliable here. The
pink/teal/yellow system is **kitsch**; the champagne-gold-on-black
system with wood-veneer card fills that *looks* gaudy-luxurious is
**neokitsch**. The first draft of these mockups had them inverted.
Anyone drawing neokitsch next: its run is doc #53–62, gold outline
cards with clipped corners over a violet bloom.

## Sampled palette

Pixel reads off the 1400px modules (`p1-072` store, `p1-129` fan menu):

```
bg           #0b0b07     warm near-black
bloom        #a63355 → #6c1c3d   rose radial, heavy vignette
teal         #7ddec8     strokes, titles, product art
teal solid   #1cb39b     page-curl, chips, PROTECTED bars
mint         #87f4d9     stat-highlight fill
yellow       #fcc428 / #fcbb15   shelf bands, selection fills
on-yellow    #37220f
bezel        #f08c1e     rounded CRT frame on device screens
```

Role mapping: `bg`=bg, `panel`=the bloom field, `border`/`fg`=teal,
`alert`=yellow, `tape`=bezel orange. Note the inversion: in kitsch,
yellow is *selection*, not alarm.

## Observed era rules

- Everything rounded; no chamfers anywhere.
- Teal line-work carries all structure; yellow is always a solid fill
  and always means "the selected thing".
- One solid teal page-curl per screen, at the foot of the container
  outline.
- Card shelf-bands poke past the card's left edge and carry compliance
  glyphs plus a brand tag.
- 3D slabs (fan menus) get stacked-outline extrusion receding up-right.
- A rose bloom vignettes every screen from a corner.
- Tiny dim-teal captions everywhere; boxed A/B/C footnote markers.
- Device screens sit inside an orange rounded bezel.

## What this does to the crate decision

The sampled kitsch **weakens** the ornament worry recorded in the
repo TODO. Real kitsch is not additive filigree — it is rounded
silhouettes, solid fills, one curl motif, and a bloom background. All
of that is parameterisable: corner radius, a curl decoration, a
background treatment. The genuinely new widget is the extruded fan
menu, which is an interaction-model difference (like neomil's diamond
menu) — a per-era widget module, not a crate boundary.

The heavy-ornament question does not disappear; it moves to
**neokitsch**, whose selected cards are filled with a wood-veneer
*texture* — the first raster asset in any era. That, not gilding, is
the thing the toolkit abstraction should be tested against.

## Files

- `target-components.svg` — the widget sheet: ticket-pill nav with
  page-curl container, product card in both states, fan menu, callout
  panel, mail list, device bezel, guest card, security strip, sampled
  swatches.
- `target-app.svg` — the 4ST store screen, tracking the reference
  closely enough to judge fidelity by eye against `p1-072`. The
  acceptance test: when this can be built from library widgets,
  kitsch is feature-complete.

Render with Rajdhani on fontconfig:

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 target-app.svg -o /tmp/kitsch-app.png
```
