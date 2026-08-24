# cyberpunk-ui

A Rust/[iced](https://iced.rs) UI toolkit for the four Cyberpunk 2077
interface eras: **entropism**, **kitsch**, **neo-militarism** and
**neokitsch**.

Formerly `neomil-ui`, which implemented one era. The rename came with the
generalisation described below. `entropism-ui`, the one-era crate beside
it, folded in here and has been deleted: its login, mail, store and
dashboard screens are `screens/` and `panels/` entries now, worn by all
four eras. Its `matrix` screen was dropped rather than ported — see
Status.

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

and screens are written once against it. `screens/` is the acceptance
test for that claim: **one implementation, four dresses.** There are
three so far — `store`, `login`, `mailbox` — and none of them contains
the word `Era`. If a fifth era cannot wear one without adding
`if era ==`, the abstraction is wrong, and those files are where it
shows.

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
  style.rs        Era, Style, Corner, Selection, Ground, Chrome,
                  Nameplate, Banner, Footnotes, Metrics
  palette.rs      Palette; rgb() for compile-time #rrggbb
  theme.rs        runtime palette published by the nix theme layer
  eras/           one table per era, sampled figures
  widgets/        surface, pill, card, banner, silhouette, glyph,
                  bracket, ornament, chrome, marker, ground, menu, text
  screens/        store, login, mailbox, dashboard — era-agnostic by
                  construction
  panels/         mail — the interactive counterpart to screens::mail
```

## The bar

`cyberpunk-ui-bar` is a wlr-layer-shell status bar built on
`iced_layershell` 0.13, which targets the same iced generation this
crate already pins -- no 0.14 migration needed.

It exists because neither waybar nor ashell can draw the eras. waybar
styles with CSS and ashell with a closed `Islands | Solid | Gradient`
enum whose corner radius is hardcoded, so on the one surface that is
always on screen, neomil's chamfer and neokitsch's clipped corner are
unreachable -- and so is entropism's square-everything. Here a bar
module is a `Surface`, so it wears whatever `Corner` the era declares.

`bar.rs` is a pure function of `Style` and `Readings`; the binary owns
the layer surface and gathering the readings, which keeps the
layer-shell dependency out of the library. Hyprland's IPC is spoken
directly rather than through `hyprland-rs`, which GitHub reports as
NOASSERTION -- no clear licence for a dependency that saves little.

Modules: hostname tape, workspaces, focused window, network, audio,
CPU, memory, date, clock, and a StatusNotifierItem tray. Every one of
them is the same `cell`, so none of them knows which era it is in.

The tray is both watcher and host, and lets the bus decide which is
live: it never asks for the name with `ReplaceExisting`, so it will not
take the tray from a waybar already serving one, and it reads the item
list back off the name rather than its own registry, so one code path
covers both cases. Icons come from a full freedesktop theme lookup, and
fall back to a short label when no theme has one. Two known gaps, both
in `iced_layershell` 0.13.7 rather than here: middle click arrives as
`Activate`, because every button but right maps to `Button::Left`; and
the scroll sign is unverified, since the raw `wl_pointer` axis runs
opposite to iced's own convention and nothing tested acts on it. Note
that a host setting no icon theme gets `hicolor` plus whatever items
ship themselves -- `--icon-theme` picks another.

The readings split by cost. Clock, CPU, memory and Hyprland's two
socket round trips are taken inline on the tick; audio and network get
a thread each under `examples/bar/`, publishing snapshots the bar reads
without ever waiting (`bar/sensor.rs`). That split is the rule, not an
optimisation: a PulseAudio handshake or a wireless driver can stall for
seconds, and a status bar that stops repainting is worse than one
missing a module. Both degrade to nothing -- no sound server means no
audio module, not a volume of zero.

## Running it

```sh
cyberpunk-ui-store                # follow the desktop theme
cyberpunk-ui-store --era kitsch   # force one
```

Comparing that against `docs/kitsch/target-app.svg` is the intended
workflow.

## Tests

    nix build .#...cyberpunk-ui.tests.store.kitsch
    nix build .#...cyberpunk-ui.tests.mailbox.neokitsch

`tests.<screen>.<era>` is a matrix over both: four screens (`store`,
`login`, `mailbox`, `dashboard`) times four eras, each rendered headless
and diffed against a golden. Every case publishes that era's
`theme/current.toml` into the sandbox HOME from
`home/themes/<era>/scheme.nix`, so it exercises the contract between
the theme layer and this crate rather than the compiled fallback.

`tests.bar.<era>` is the same idea for the status bar, at 1600x220 via
`tests/bar.nix`. It renders `examples/cyberpunk-ui-bar-window.rs`, a
plain iced window sharing the live bar's era resolution through
`examples/bar/style.rs` — weston has no `wlr-layer-shell`, so the real
binary cannot be rendered by this harness at all. It holds `bar()` still;
it says nothing about mapping a layer surface, the exclusive zone, or any
sensor producing a reading.

`tests.visual` keeps the fallback case: the dashboard with *no*
`theme/current.toml` in the sandbox, so it renders `Style::from_desktop`
falling back to neo-militarism's sampled table. It used to be a
1280x800 render of a neomil-only dashboard; that screen no longer
exists, so the case moved onto the matrix's 1600x900 geometry and its
own golden. It is worth keeping beside `dashboard.neomil` even though
the two renders are byte-identical today: that equality is the claim
that `home/themes/neomil/scheme.nix` and `eras/neomil.rs` still agree,
and only having both cases can catch it breaking.

If you drive the harness by hand rather than through nix, `unset
XDG_CONFIG_HOME` first, or you will render the "reference" screen in
whatever era your desktop is currently sitting in.

## Status

Version stays at 0.0.0: this is nowhere near release.

Implemented: the era abstraction, the shared widget vocabulary, all four
era tables, four screens (store, login, mailbox, dashboard) in all four
dresses, and a screen-by-era visual regression matrix. All four eras
also have desktop themes under `home/themes/`, so `Style::from_desktop`
has something real to follow.

Not yet done:
- **The menu is on no screen.** `Menu { Tiles | Fan | Diamonds |
  Cascade }` is in `style.rs`, every era declares one, and
  `widgets::menu` draws all four — but nothing calls it, so no golden
  covers it. Wiring it is a one-function change in
  `screens::dashboard`'s module grid. Note the diamond hub is inherited
  rather than sampled: neither of neomil's design sheets draws one,
  despite this file having called it that era's interaction model.
- `widgets::message_card` is unreachable — nothing has called it since
  the entropism-ui retirement replaced its last caller with the shared
  vocabulary. It now takes a `Style` and `colors.rs` is gone, but its
  geometry is still a hardcoded 8px neomil double chamfer while
  `widgets::surface` draws all four corner treatments. Fold it into
  `mail_row`/`Surface`, or drop it.
- Neokitsch's BASKET panel and its step-notch pill on the mailbox
  footer are in the design targets but not yet widgets. The fan, the
  cascade, the tiles, the ticket notch, the compliance caption and the
  nav-column outline that runs into the page-curl have all since
  landed.
- A **data table** widget. `entropism-ui`'s `matrix` screen was the
  reason to want one, and it was dropped rather than ported: it was
  never the node graph it had been described as — a 25x60 grid of
  generated status strings behind a fixed 12x22 viewport, scrolled with
  hjkl, with no hit-testing anywhere and exactly one clickable control
  — and no era's design target contains such a screen, so there was
  nothing to dress it against.
  `docs/neomil/target-components.svg` does list "table with selection +
  scrollbar", "log view" and "key-value rows", which is where the
  reusable part belongs; `panels::mail`'s markdown table is the seed.
- Fields are display-only. `widgets::input::field` draws the box and the
  value but takes no input; the screens it serves are design targets,
  and a real `text_input` needs per-era styling before it earns a place.
