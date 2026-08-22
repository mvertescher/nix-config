# Neokitsch design targets

Sampled from the Behance Part 1 neokitsch run: the ten images after the
"NEO KITSCH — SUBSTANCE AND STYLE" title card (doc #53–62 in gallery
document order — arasaka login, card cascades, mailbox, 4ST store).
See ../kitsch/README.md for how the four era runs were recovered and
for the attribution warning: this champagne-gold-on-black system is
neokitsch; the pink/teal/yellow one is kitsch, not the reverse.

## Sampled palette

Pixel reads off the 1400px modules (`p1-061`, `p1-136`):

```
bg          #0a0a0a       true black outside the frame
bloom       #34344c mid   violet haze, top-centre
frame       #916424 outer / #5e3414 inner   double gold stroke
gold text   #e7c686       logotype, headings
champagne   #d3b279       bands, secondary text
veneer      #f4c474 → #d8a558   wood-grain fill on selected elements
amber CTA   #fcc474 → #c78948   ENTER / LOGIN bars
field       #2c1c14       input fills
strata      #634427       fine-line layered dividers
```

Role mapping: `bg`=bg, `panel`=bloom field, `border`=frame gold,
`fg`=gold text, `dim`=#8a7048, `tape`=veneer.

## Observed era rules

- Gold line-work on black under a violet haze; quieter than kitsch —
  no page-curl, no shelf bands, far fewer captions.
- The device frame is part of the UI: double gold stroke, stepped
  corner tabs top and bottom, strata wedge at the foot.
- **Selection is a material, not a colour**: the chosen tab, pill,
  card or mail row is filled with wood veneer. In these SVGs the
  veneer is a gradient plus grain strokes; a real implementation
  should treat it as a texture asset — the first raster asset in any
  era, and the stress test the repo TODO flags for the toolkit
  abstraction.
- Product cards clip the top-right corner and carry their name in a
  footer band, not a header.
- Strata dividers: many fine lines bunching into a wedge, with boxed
  A/B/C/D footnote markers sitting on them.
- Folder-tab chips for security levels and the basket.
- Amber gradient bars are the only strong CTAs.

## Files

- `target-components.svg` — folder-tabs, strata divider, login field
  and CTA, nav pills, step-notch pill, basket panel, product card in
  both states, card cascade, mail list, detail text, the device frame
  in miniature, sampled swatches.
- `target-app.svg` — the 4ST store screen inside the full device
  frame, tracking `p1-061`'s right-hand screen. Acceptance test: when
  this can be built from library widgets, neokitsch is
  feature-complete.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 target-app.svg -o /tmp/nk-app.png
```
