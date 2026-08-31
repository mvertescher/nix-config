# cp-eras-ui

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
four — `store`, `login`, `mailbox`, `dashboard` — and none of them
branches on era. If a fifth era cannot wear one without adding
`if era ==`, the abstraction is wrong, and those files are where it
shows.

The alternative, a crate per era, was rejected once the sampling showed
how much the eras share. The genuinely era-specific things left are
*interaction models* and one *layout*, not dressed rectangles: kitsch's
extruded fan menu, neokitsch's card cascade, entropism's tiles, and
neomil's dashboard. Both kinds live behind a choice on `Style` rather
than an era test in a screen — the `Menu` enum for what a menu is, and
the `Layout` enum for what a dashboard *is*: `Layout::ModuleHub`, the
six-module hub the two hub eras wear; `Layout::OpsCharts`, neomil's
ops-charts screen straight off `docs/neomil/dashboard-trace.svg`, which
is what the material's `img-07` actually shows; `Layout::TileRow`,
entropism's four-tile row straight off
`docs/entropism/dashboard-trace.svg`, which is what its Behance screen
#42 actually shows. So a screen picks a menu and a dashboard without
naming an era, and a fifth era cannot wear either without adding data —
which is the same discipline that forbids `if era ==` in `screens/`.

Neomil's arm was a **cut-diamond hub** until the table landed, and the
story is worth keeping because it is the shape this crate's mistakes
take. The hub was inherited from the pre-generalisation crate and was in
*neither* neomil sheet -- `target-app.svg` puts a services table where
the dashboard puts its menu -- so it was kept as an admitted stand-in,
with both `style.rs` and the widget's own header saying it was the first
thing to reconsider when the table arrived. It arrived; the hub is
deleted. Nothing was lost with it: the hit-testing it was credited with
is `mouse_area` in `panels::mail` and `bar`, and a table built out of
layout needs none.

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
  window decoration. The top and bottom rails are the stepped
  double-gold-stroke device frame from `docs/neokitsch/target-app.svg`
  — lit outer stroke at the top, flat frame gold at the foot, the
  shaded `FRAME_INNER` line inside both — with a strata wedge at the
  foot; the meta line and the screens' contents sit inside it.

## Palette resolution

Colours come from two places. The nix theme layer publishes seven
semantic roles to `$XDG_CONFIG_HOME/theme/current.toml` on every
`switch`; `theme.rs` parses it dependency-free. Each era also carries a
reference-sampled fallback so the crate runs standalone with no nix in
sight.

`Style::from_desktop()` reads the published era *and* overlays its
palette, so apps re-dress themselves when the desktop switches era —
no rebuild. `select`/`on_select` are era-owned and never come from the
theme file, because across the four eras selection is not one colour
with four values but four different ideas; `emphasis` is instead
theme-sourced, overlaid from the optional roles when the theme declares
them.

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
                  bracket, ornament, chrome, marker, ground, menu, table,
                  text
  screens/        store, login, mailbox, dashboard — era-agnostic by
                  construction
  panels/         mail — the interactive counterpart to screens::mail
```

## The bar

`cp-eras-ui-bar` is a wlr-layer-shell status bar built on
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

A per-era SVG reference at the golden geometry lives in
`docs/<era>/bar.svg` — the same 1600x220 frame the `tests.bar.<era>`
goldens render into, so a capture sits directly beneath the drawing
for a by-eye fidelity check. Render it with the same `rsvg-convert`
invocation the era READMEs document:

```sh
cd docs/<era>
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 -h 220 bar.svg -o /tmp/bar.png
```

The dashboard gets the same treatment: `docs/<era>/dashboard.svg`
sits at the 1600x900 frame the `dashboard.<era>` goldens render, and
each era's README lists both alongside `target-app.svg`.

The tray is both watcher and host, and lets the bus decide which is
live: it never asks for the name with `ReplaceExisting`, so it will not
take the tray from a waybar already serving one, and it reads the item
list back off the name rather than its own registry, so one code path
covers both cases. Icons come from a full freedesktop theme lookup
(PNG, SVG, ARGB32 pixmaps and XPM), and fall back to a short label when
no theme has one. Note that a host setting no icon theme gets `hicolor`
plus whatever items ship themselves -- `--icon-theme` picks another.

Right-click draws the item's `com.canonical.dbusmenu`, submenus and row
icons included, on a **second `Overlay` layer surface anchored to all
four edges** with `exclusive_zone: Some(0)`. That shape is forced:
`layershellev` 0.13.7 never calls `xdg_popup.grab()` and the bar takes
no keyboard focus, so a popup or a menu-sized surface could only ever be
dismissed by being clicked. Output-sized gives real click-outside
dismissal *and* placement below every bar without this code asking where
any bar is. A submenu chain is drawn *inline in that same surface*
rather than stacking another -- a second overlay would cover the parent
and stop its rows answering -- and it opens leftwards, because the tray
is the last group on the right. Childless `Submenu` rows still answer a
click, since for a lazily-populated menu that click is what sends
`AboutToShow`.

Two known gaps are upstream in `iced_layershell` 0.13.7 rather than
here: middle click arrives as `Activate`, because every button but right
maps to `Button::Left`; and the scroll *sign* is unverified -- not
because the axis is forwarded raw, which an earlier version of this file
claimed and which is wrong (0.13.7 negates on all four paths and already
matches iced's convention), but because nothing on this desktop acts on
`Scroll` at all. There is no hover-to-open and no keyboard navigation,
both deliberate on a surface with no grab.

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
cp-eras-ui-store                # follow the desktop theme
cp-eras-ui-store --era kitsch   # force one
```

Comparing that against `docs/kitsch/target-app.svg` is the intended
workflow.

## Tests

    ./scripts/run_test_matrix.sh              # all 21 cases
    ./scripts/run_test_matrix.sh store        # only cases matching /store/

That is the whole invocation. The script fetches this repo, takes
`pkgs` from the flake's `out` escape hatch, walks `passthru.tests` and
prints a pass/fail table. It retries a case once before calling it
failed: the harness gives each render a fixed 15s to settle and that is
not always enough under load. `tests/matrix.nix` is the door it goes
through, and takes `pkgs` if you want the cases in an expression of
your own.

This section used to show a `nix build .#...` line that was never a
real command — the package is created once in `lib/overlays.nix` and
exposed as `pkgs.cp-eras-ui`, and this repo still exports no
configurations to hang one off — so anyone who needed a
golden wrote a throwaway instantiation under /tmp. If you write one
anyway: fetch this repo with `git+file:`, **never** `path:`. The reason
is at the top of `scripts/run_test_matrix.sh`, it cost 1.8 TB of disk,
and the script now refuses to build if its source path is too big.

`tests.<screen>.<era>` is a matrix over both: four screens (`store`,
`login`, `mailbox`, `dashboard`) times four eras, each rendered headless
and diffed against a golden. Every case publishes that era's
`theme/current.toml` into the sandbox HOME from
`home/themes/<era>/scheme.nix`, so it exercises the contract between
the theme layer and this crate rather than the compiled fallback.

`tests.bar.<era>` is the same idea for the status bar, at 1600x220 via
`tests/bar.nix`. It renders `examples/cp-eras-ui-bar-window.rs`, a
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
dresses, each hub era's own menu on the dashboard, and a screen-by-era
visual regression matrix. All four eras also have desktop themes under
`home/themes/`, so `Style::from_desktop` has something real to follow.

Since 2026-08-31 neomil's dashboard follows its own material rather
than the hub: `Layout::OpsCharts` draws the ops-charts screen from
`docs/neomil/dashboard-trace.svg` (the cold band, the three chart
cards, the right rail) on the `screens::dashboard` OpsCharts arm, with
the new `widgets::charts` chart-card behind each cell. Neomil keeps
`menu: Menu::Table` — the services-table hub arm is **retained
dormant** for any era or host that wants a table in the menu slot; the
`OpsCharts` arm simply never consults the menu.

Entropism's dashboard followed on the same day, the same way:
`Layout::TileRow` draws the four-tile row from
`docs/entropism/dashboard-trace.svg` (Behance screen #42 — the
dim-olive top field with its boxed [A] TILE MENU header, four tiles
with T2 `LOCATIONS` solid sage and selected, the caption strips, the
thin build-rule at the foot) on the `screens::dashboard` TileRow arm.
Entropism keeps `menu: Menu::Tiles` — the tile-grid hub arm is
**retained dormant**, exactly as `Menu::Table` is for neomil; the
TileRow arm never consults the menu either.

Not yet done:
- **`widgets::table` has no scroll rail**, which both neomil sheets draw.
  Left out on purpose: a rail asserts rows exist off-screen, this widget
  shows every row it is handed, and its one caller sat in a column with
  slack below it. That caller went dormant with the ops-charts layout,
  and the rail stays a no-caller decoration until a table hub has a
  caller again — which is the trap the audit below exists to close.
- Neokitsch's BASKET panel and its step-notch pill on the mailbox
  footer are in the design targets but not yet widgets. The fan, the
  cascade, the tiles, the table, the ticket notch, the compliance
  caption and the nav-column outline that runs into the page-curl have
  all since landed.
- Fields are display-only. `widgets::input::field` draws the box and the
  value but takes no input; the screens it serves are design targets,
  and a real `text_input` needs per-era styling before it earns a place.
