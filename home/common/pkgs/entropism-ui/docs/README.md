# Entropism design targets

Sampled from the Behance Part 1 entropism run: the nine screens after
the "ENTROPISM — NECESSITY OVER STYLE" title card (doc #24–32 in
gallery document order — logins, mailbox tile menu, message view, 4ST
store). See neomil-ui/docs/kitsch/README.md for how the four era runs
were recovered from the gallery.

## Sampled palette

Pixel reads off the 1400px modules (`p1-086` login, `p1-036` store):

```
bg          #110c07   warm dark olive-brown (#1a140c upper, #0d0603 lower)
sage solid  #9cb795   selection fills, footer band
sage text   #94bb94   labels, titles
mid         #728f76   top-bar text, secondary
outline     #5d7752   1px strokes
dim         #3d4d38   faint rules, captions
on-solid    #1f2a1c   dark text on sage fills
```

**One hue.** The era is a single sage green on a warm dark ground —
a monochrome terminal that somebody keeps repairing.

## Observed era rules

- Square everything; 1px strokes; no glow, no gradients, no rounding.
- Selection is a solid sage fill — tiles, rows, buttons, T-levels.
- Segmented top bar of outlined boxes; a build-string footer on every
  screen (`INTERFACE LOADED · PROVIDED BY NEXUS NETWORK V10.8 ·
  BUILD 6.47.48441.R15`); login screens swap the footer for one huge
  solid band.
- Boxed-letter section headers ([A] MAIL BOX, [B] MESSAGE …).
- Menu tiles carry tiny caption strips beneath them.
- Dense small maintenance captions throughout.

## Toolkit divergence (handoff)

The crate in this directory does not match the era it is named for:

- `src/colors.rs` carries **twelve** colours, including cybr's red
  `#F75049`, cyan `#5EF6FF`, mint `#1DED83`, violet, orange and gold.
  The reference has one hue. `COLOR_GREEN_ACCENT #8CBC88` is close to
  the sampled `#94bb94` and is the only keeper.
- `src/glow.rs` implements a radial glow. The reference has none —
  flatness is the aesthetic.
- The desktop theme (`home/themes/entropism`) diverges too: its
  default `burn-in` variant is amber, an invention; of its three
  variants only `salvage-phosphor` is near the reference, and it is
  still cooler and darker than the sampled sage. Suggest adding a
  reference-sampled variant (working name `nexus`, after the build
  strings) and making it the default.
- When this crate folds into `cyberpunk-ui`, reduce it to the one-hue
  system; the extra colours are not "entropism with more range", they
  are a different era's palette.

## Files

- `target-components.svg` — login, footer strips, boxed headers,
  security levels, button segments, menu tiles, mail list, message
  detail, product card in both states, sampled swatches, and the
  divergence notes rendered on-sheet.
- `target-app.svg` — the 4ST store (tracking `p1-036`). Acceptance
  test: when this can be built from library widgets, entropism is
  feature-complete.

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 target-app.svg -o /tmp/en-app.png
```
