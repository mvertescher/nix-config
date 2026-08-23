# cyberpunk-ui

A Rust/[iced](https://iced.rs) UI toolkit for the four Cyberpunk 2077
interface eras: **entropism**, **kitsch**, **neo-militarism** and
**neokitsch**.

Formerly `neomil-ui`, which implemented one era. The rename came with the
generalisation described below; `entropism-ui` folds in here too.

## Why this shape

All four eras were sampled from the published reference material rather
than described from memory (`docs/<era>/README.md` records provenance,
pixel-read palettes and observed rules). Laying the four design targets
side by side settled the crate's architecture, because they dress **the
same screens**. Every era's `target-app.svg` is the 4ST weapons store:
same logotype, same customer meta, same category nav with one item
selected, same four product cards with one selected and grown, same
footnote markers, same footer.

So an era is **data**, not a trait to implement:

```rust
pub struct Style {
    pub era: Era,
    pub palette: Palette,     // 7 published roles + select/on_select/emphasis
    pub corner: Corner,       // Square | Chamfer | Round | ClipTopRight
    pub selection: Selection, // Solid | Veneer
    pub ground: Ground,       // Flat | Bloom
    pub chrome: Chrome,       // Segmented | Tape | Caption | DeviceFrame
    pub nameplate: Nameplate, // Header | Footer
    pub metrics: Metrics,
}
```

and screens are written once against it. `screens::store` is the
acceptance test for that claim: **one implementation, four dresses.** If
a fifth era cannot wear it without adding `if era ==` to that file, the
abstraction is wrong — which is exactly what we want to find out.

The alternative, a crate per era, was rejected once the sampling showed
how much the eras share. The genuinely era-specific things left are
*interaction models*, not dressed rectangles: neomil's diamond menu,
kitsch's extruded fan menu. Those stay per-era widget modules inside the
one crate.

## Where the eras actually differ

| | entropism | kitsch | neomil | neokitsch |
|---|---|---|---|---|
| corner | square | round r16 | chamfer 15 | clip top-right |
| selection | solid sage | solid yellow | solid red | **wood veneer** |
| ground | flat | rose bloom | flat | violet bloom |
| chrome | segmented bar | centred caption | tape | device frame |
| nameplate | header | header | header | **footer** |
| hues | **one** | teal + yellow | three reds | gold |

Two entries carry most of the risk:

- **`Selection::Veneer`** — neokitsch fills the selected element with a
  *material*, the only non-flat fill in any era. `widgets::surface`
  synthesises it (base tone, banded warp, grain lines) rather than
  shipping a raster asset, and clips the grain with `span_at` instead of
  relying on renderer path clipping.
- **`Chrome::DeviceFrame`** — neokitsch's frame is part of the UI, not a
  window decoration.

## Palette resolution

Colours come from two places. The nix theme layer publishes seven
semantic roles to `$XDG_CONFIG_HOME/theme/current.toml` on every
`switch`; `theme.rs` parses it dependency-free. Each era also carries a
reference-sampled fallback so the crate runs standalone with no nix in
sight.

`Style::from_desktop()` reads the published era *and* overlays its
palette, so apps re-dress themselves when the desktop switches era —
no rebuild. `select`, `on_select` and `emphasis` are era-owned and never
come from the theme file, because across the four eras selection is not
one colour with four values but four different ideas.

Note kitsch's inversion: yellow is *selection*, not alarm. `alert` and
`select` are the same colour there and different everywhere else, which
is why they are separate roles.

## Layout

```
src/
  style.rs        Era, Style, Corner, Selection, Ground, Chrome, Metrics
  palette.rs      Palette; rgb() for compile-time #rrggbb
  theme.rs        runtime palette published by the nix theme layer
  eras/           one table per era, sampled figures
  widgets/        surface, pill, card, chrome, marker, ground, text
  screens/        store — era-agnostic by construction
```

## Running it

```sh
cyberpunk-ui-store                # follow the desktop theme
cyberpunk-ui-store --era kitsch   # force one
```

Comparing that against `docs/kitsch/target-app.svg` is the intended
workflow.

## Status

Implemented: the era abstraction, the shared widget vocabulary, all four
era tables, and the store screen in all four dresses.

Not yet done:
- `panels/`, `top_bar.rs`, `background.rs` and the neomil widget set
  predate the generalisation and still hardcode neomil colours; the
  dashboard and mail screens want rewriting against `screens`.
- The visual regression test (`tests/visual.nix`) covers one era. It
  wants a per-era matrix, writing `theme/current.toml` into the sandbox
  HOME.
- Kitsch's extruded fan menu and page-curl, neokitsch's card cascade,
  and entropism's menu tiles are in the design targets but not yet
  widgets.
