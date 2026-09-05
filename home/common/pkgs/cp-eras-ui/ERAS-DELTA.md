# Era tables vs traces: the IMPLEMENTATION DELTA boxes, classified

Written 2026-09-03 from the four `docs/<era>/components.svg` delta boxes
(`entropism/components.svg:532-568`, `kitsch/components.svg:688-704`,
`neokitsch/components.svg:1113-1131`, `neomil/components.svg:1293-1304`).
Every consumer below was found by grepping `src/`, `examples/`, `tests/`
this session; every trace value was read from the trace file named.
Line numbers in `src/eras/*.rs` are post-edit (this pass added comment
lines above them).

Classes:

- **a** consumed by a gated, passing screen (login / mailbox / store
  canvas programs, or the bar, all of which have goldens under
  `tests/golden/`).
- **b** consumed only by dashboard widgets or `bar.rs`-side widgets
  (`widgets::surface`, `menu`, `chrome`, `charts`, `bracket`,
  `ornament`), i.e. behind the undecided `Layout` fold.
- **c** consumed by nothing.
- **d** doc comment / string only.

Two things that bear on the meaning of class (a) for palette constants:

1. **The gated renders overlay the nix theme.** `default.nix:145-222`
   (`variantOf`, `eraCase`, `barCase`), `tests/visual.nix:56-136` and
   `scripts/render.sh --era` publish `home/themes/<era>/palettes.nix`
   (variant `nexus` for entropism, `reference` otherwise) into the
   sandbox `HOME`; each example (`examples/cp-eras-ui-login.rs:21-31`
   and siblings) then calls `Palette::with_theme` ->
   `Palette::with_roles` (`src/palette.rs:138-173`), which **replaces**
   `bg, panel, border, dim, fg, alert, tape` and every declared extra
   (`banner/onBanner`, `emphasis/onEmphasis`, `bevel/shade`,
   `ornament`, `inset`). Only `select`, `on_select`, `cta`, `bloom`
   and `banner_selected` survive from the era table. So for a const
   feeding one of the seven base roles or an overridden extra, the era
   value is **not what the golden shows**; the nix value is (they
   agree except: entropism panel `#181109` vs table `BG #110c07`;
   kitsch panel `#1c0f16` vs `BLOOM #a63355`; neokitsch panel `#16161f`
   vs `BLOOM #34344c`; neomil identical. Entropism tape and kitsch
   border disagreed too until 2026-09-05, when nix moved to the
   trace values `#728f76` and `#5fd6c2`). Changing such a const in the era file alone would not
   move a gated render; the marker "(a, overridden)" below means "a
   gated screen reads this role, but the golden's value comes from
   nix". The one golden with no theme is `dashboard-fallback` (neomil
   table used directly).
2. **`widgets::ground` (`Style::ground`, `palette.bloom`) reaches one
   golden, `mailbox-entropism`, and there as a flat `bg` fill.**
   Since 2026-09-05 every login ground is a `Prim::Soft` group
   composited by `scene::Backdrop` (`Access.backdrop`, transcribed from
   the login traces; until then `login.rs` drew an opaque wash from
   closures of its own), every mailbox but entropism's composites
   `Mailbox::backdrop` the same way (`src/screens/mail.rs:120-130`;
   kitsch `MAIL_BACKDROP` carries the trace's `cx .5 cy -.25 r .95`
   bloom, neomil's the glow and vignette), and every store and
   dashboard table opens with a full 1600x900 fill inside its own
   composited group (`STORE_GROUND`, `BACKDROP`, `HUB_BACK`,
   `HUB_GROUND`) over the `widgets::ground` the screen stacks under
   the canvas (`store.rs:82`, `dashboard.rs:75`). So at the goldens'
   size `Ground::Bloom` shows nowhere; its discs are visible only in
   the letterbox margins of a non-16:9 window and under `panels::mail`,
   the working client, which has no golden. (Until 2026-09-04 the
   mailbox took `widgets::ground` unless `mailbox.haze` was set and
   entropism's store painted no full rect; `30683d5` changed both.)

Gate status is as recorded in `TODO.md:571-592` (bar 100/86/83/52, login
28/96/72/89, mailbox 100/95/67/86, store 100/84/88/82 for
entropism/neomil/kitsch/neokitsch; dashboards 0% against the traces).
No gate was re-run in this pass.

## Entropism (`src/eras/entropism.rs`)

| delta item | trace says | code says | consumers | class | action |
|---|---|---|---|---|---|
| `metrics.stroke = 1.0`, doc "1px strokes" | 1.25px: `login-trace.svg` rect 49,43 1498x26 `stroke-width="1.25"`; `dashboard-trace.svg` same rect; `store-trace.svg` card 0,0 265x237. 2px: `mailbox-trace.svg` `#hdr-chrome`/`#a-chrome` `stroke-width="2"` (full-res probe: 3px core `#8fba97`, design stroke 2px) | `stroke: 1.0` :268; module doc :4 said "1px strokes" | value: `widgets/surface.rs:158,169,191`, `menu.rs:374`, `chrome.rs:129`, `bracket.rs:124`, `ornament.rs:135,151`, `card.rs:292`, `screens/dashboard.rs:685,721` -- all bar/dashboard paths. Canvas arms pass widths explicitly (`entropism.rs:122` bar 2.0; login :331 1.25; store `line_rect(.., 2.0)`) | value **b**; doc **d** | doc rewritten (:2-24) to state 1.25/2 with cites and why 1.0 stays; note added at :264-267 (comment above `stroke: 1.0` :268). Value untouched. |
| `SAGE_SOLID #9cb795` dimmer than trace | solid fill `#a6d3a7` (`dashboard-trace.svg` x114/x180 fills), `#a6d2a8` (`mailbox-trace.svg` selected row), `#a8d4a2` (`store-trace.svg` grown card, group translate(461,260)) | :44; `select` :65, `cta` :75 | `Ink::Select` mailbox :587, store :683..866 (x40); `Ink::Cta` unused by entropism arms; bar :144, :199 (`Ink::Select`) | **a** (select not overridden: `mailbox-entropism`, `store-entropism`, `bar-entropism` goldens) | none |
| `SAGE_TEXT #94bb94` dimmer | text `#acddb4` (`dashboard-trace.svg` text fills), `#a6d2a8`, `#a8d4a2` | :45; `fg` :59 | `Ink::Fg` mailbox :431..627, store :690..904 | **a**, overridden (nix `fg = "#94bb94"`, `home/themes/entropism/palettes.nix:32`) | none |
| `ON_SOLID #1f2a1c` | `#25281d` (mailbox), `#22301f` (`dashboard-trace.svg` dark-on-sage), `#35462e` (`store-trace.svg`) | :49; `on_select` :66 | `Ink::OnSelect` mailbox :590-592, store :699..819 (x20) | **a** (not overridden: `mailbox-entropism`, `store-entropism`) | none |
| `OUTLINE #5d7752` matches nothing | k-means outline centroids `#709174` / `#70926d` / `#739479` (1600 rescale); full-res core `#8fba97`; mailbox strokes drawn `Ink::Mid` (:396) | :47; `border` :57 | `Ink::Border` mailbox :396 (x1), store :682..901 (x21); bar; `widgets/surface.rs`, `bar.rs:1734` | **a**, overridden (nix `border = "#5d7752"`, palettes.nix:30) -- and a user decision (TODO OUTLINE item) | none |
| `BG #110c07` + `Ground::Flat` | radial lift: `login-trace.svg` `<radialGradient id="lift" cx="0.45" cy="0.4" r="0.8">` `#1a1810`/`#141107`/`#0f0a04`; mailbox `#1c1b11`/`#151207`/`#100a03`; store `#1e1d12`/`#151207`/`#100b03`; dashboard cx.45 cy.45 r.75 `#1c1a10`/`#141107`/`#0f0a03` | `BG` :43, `bg` :53, `panel` :56, `bloom` :76; `ground: Ground::Flat` :86 | `widgets/ground.rs:22-24` via `screens/mail.rs:158` (haze empty) and `screens/store.rs:88` (no full-frame fill in entropism's store); login wash is opaque (`login.rs:650-652`, own `LIFT` :602) | **a** (`mailbox-entropism`, `store-entropism` goldens; `bg` overridden with the same value, `Ground` not) | doc: module doc :19-25 now records the lift and that `Flat` is deliberate (mailbox block :385-396 already said so). Value untouched. |
| `Layout::TileRow` vs 3x2 grid + detail panel + badge column | `dashboard-trace.svg` (per box) | :249 | `screens/dashboard.rs:181-187` | **b** (Layout decision) | none |
| doc "Behance Part 1, doc #24-32" (not in box) | `docs/sources.md:19-27`: entropism 34-42, title card 33 | module doc :4-5 | -- | **d** | corrected :3-6, old number kept as note |

Counts: a 5 (of which 3 are nix-overridden roles), b 2, c 0, d 3
(stroke doc, lift doc, Behance range).

## Kitsch (`src/eras/kitsch.rs`)

| delta item | trace says | code says | consumers | class | action |
|---|---|---|---|---|---|
| `Corner::Round { radius: 16.0 }` | `rx="8"` `dashboard-trace.svg` `#card` 162x50 and `store-trace.svg` rect 124.5,178.5 213x21; `rx="2"` `login-trace.svg` rect 257,413 335x51 and `mailbox-trace.svg` `#badge`; `rx="1.5"` store 28.5,74; card top-left r6 (`store-trace.svg` `#card` `M 6.5,0.5 ... Q 0.5,0.5 6.5,0.5`) | :120 | `widgets/surface.rs:155,166,188` (`default_corners`), `bracket.rs:62,97,125`, `charts.rs:127-129`, `screens/login.rs:897` (matches only `ClipTopRight`, so `Round` falls to `pen.plate`) | **b** | module doc rewritten (:2-19); value untouched |
| doc "no chamfers anywhere" | `store-trace.svg` `#card` `L 261.5,24.5` (24px TR chamfer); band `V 71.5 L 235.5,94 Z` (45 deg); `#nav` chevron `M 0,39 V 19 L 18,0 ...`; `mailbox-trace.svg` row end (491,325)->(471,346), tab TR (1105,313)->(1127,337) | module doc :4 (old) | -- | **d** | corrected :7-19 with the five cites |
| `Ticket { reach: 18, drop: 15 }` | `store-trace.svg` `#nav` `M 0,39 V 19 L 18,0 L 27,3 H 214 Q 216,3 216,5 V 11 L 190,39 Z` (216x39, peaked chevron, 26x28 right chamfer) | :350-353 | `style.ticket` read only at `widgets/pill.rs:32,50` (`pill()`); `pill()` has no callers outside `pill.rs` (re-exported `widgets/mod.rs:65`, never called) | **c** | annotated "Unconsumed as of 2026-09-03; trace value would be ..." :338-349 |
| `Banner { overhang: 12, notch: 8 }` | `store-trace.svg` band `M -27,94 V 72 L -3,50 V 59 H 256 Q 258,59 258,61 V 71.5 L 235.5,94 Z`: 35 tall, 27 flag left, no notch, 45 deg right chamfer | :328-331 | `style.banner` read only at `widgets/banner.rs:97-98` (`banner()`) and `widgets/card.rs:246` (`product_card()`); neither function has a caller | **c** | annotated :318-327 |
| `Ground::Bloom { x: .82, y: 0, r: .75 }` (top right) | `login-trace.svg` and `store-trace.svg` `cx="0.5" cy="-0.25" r="0.95"`; `mailbox-trace.svg` r=0.9; `dashboard-trace.svg` cx.52 cy-.05 r.85 -- top centre | :123-127; `BLOOM #a63355` :39, `bloom` :112 | `widgets/ground.rs:48-64` via `screens/mail.rs:158` (kitsch `haze: &[]` :848) and `screens/dashboard.rs:185`; login wash opaque (`login.rs:666-677`), store covered by `fill_rect(0,0,1600,900, PAGE)` :1039 | **a** (`mailbox-kitsch` golden; `bloom` not overridden) | none |
| `metrics.stroke: 1.5` | 1.25 on .5 coords (`store-trace.svg` `#card` `stroke-width="1.25"`, mailbox x7, store x8); 2.0 letter boxes; 1.0 badges; `dashboard-trace.svg` fan cards 1.8 | :374 | same widget set as entropism (surface/menu/chrome/bracket/ornament/card/dashboard) | **b** | none |
| `SLAB #2bc4ac` vs fan `#2c9798` | `dashboard-trace.svg` `<use href="#card" fill="#2c9798" stroke="#a9e6df" stroke-width="1.8"/>` | :60; `relief: Some((SLAB, SLAB_SHADE))` :104 | `Palette::relief()` at `bar.rs:443`, `widgets/menu.rs:359`, `widgets/chrome.rs:123` | **b**, overridden (nix `bevel = "#2bc4ac"`, `home/themes/kitsch/palettes.nix:49`) | doc extended :55-59 |
| `BEZEL #f08c1e`: no traced screen has a bezel | `#f08c1e` appears only in `docs/kitsch/bar.svg:12,141`; the mailbox badge is `#f0a93c` (`mailbox-trace.svg:206`) | :45; `tape` :90 | `palette.tape` at `widgets/chrome.rs:299` (`Chrome::Tape`, neomil-only arm) and `screens/dashboard.rs:581`; `Ink::Tape` in the kitsch bar block (x1) | **b**, overridden (nix `tape = "#f08c1e"`, palettes.nix:40); bar golden reads the nix value | none |
| doc "doc #34-42" | `docs/sources.md:19-27`: kitsch 44-52, title card 43 | module doc :5 (old) | -- | **d** | corrected :4-6 |
| (TODO-only, not in box) `YELLOW_SHADE #f0a80a` "unsampled" and its doc's `store-trace.svg` claim | `#f0a80a` in no trace; `store-trace.svg` grown card (translate(804 218)) fills `#ffc233` (:350), band `#fec32f` (:244), grown band a 1.1px `#a4583a` outline (:357) | :80; `banner_selected: Some((YELLOW_SHADE, ON_YELLOW))` :97 | `Palette::banner_on_select` (`palette.rs:241`) -> `widgets/banner.rs:39-41` `banner_colors` -> `banner()`/`tag()`/`card::product_card`/`card::notice`, none called by any screen or the bar | **c** + false doc **d** | doc corrected and annotated :61-79 |

Counts: a 1, b 4, c 3, d 3 (chamfers doc, Behance range, YELLOW_SHADE
doc). The TODO's premise "YELLOW_SHADE #f0a80a unsampled" holds; the
const's own doc comment claiming a `store-trace.svg` sample was false.

## Neokitsch (`src/eras/neokitsch.rs`)

| delta item | trace says | code says | consumers | class | action |
|---|---|---|---|---|---|
| `Chrome::DeviceFrame`, doc "device frame itself part of the UI" | no full-screen frame in any of the four traces; `docs/neokitsch/README.md:57-63` "There is no device frame" | `chrome: Chrome::DeviceFrame` :138; module doc :3-4 (old) | `widgets/chrome.rs:267,355` (`top_bar`/`footer`, called from `screens/dashboard.rs:195,213` and `panels/mail.rs:166,176`); `bar.rs:658` reads `BarChrome`, not this | value **b**; doc **d** | module doc rewritten :3-16; value untouched |
| `Corner::ClipTopRight { cut: 30.0 }` | per-widget cuts; nothing clips its top-right by 30 (box); mailbox selection bar 22px TR chamfer `M 35,315 H 490 L 512,337` | :130 | variant: `screens/login.rs:897-898` (`badge_plate` matches `ClipTopRight { .. }`, uses hardcoded r=3 / fold=7, ignores `cut`); `cut` value: `widgets/surface.rs:155,166,188`, `bracket.rs`, `charts.rs:127-129` | variant **a** (`login-neokitsch` golden); `cut: 30.0` **b** | none |
| `FRAME #916424` sampled by nothing | outlines `#c5965a` (`store-trace.svg:135`, card outlines), `#bd8951` / `#a97c48` (`dashboard-trace.svg:221,241,285,327`), `#dab176` (store), `#e0b67a` (mailbox/store); `#916424` only in `bar.svg` / `components.svg` | :39; `border` :88 | `palette.border`: `widgets/surface.rs` (outlined/filled), `bar.rs:1734`; no `Ink::Border` in any neokitsch arm | **b**, overridden (nix `border = "#916424"`, `home/themes/neokitsch/palettes.nix:34`) | `FRAME_LIT` doc :51-59 now records the trace outline samples; `FRAME`/`FRAME_INNER` value untouched |
| `STRATA #634427`, doc "strata divider bunching into a wedge" | no strata divider in any trace; `#634427` absent from all four traces | :70; `ornament: Some(STRATA)` :117; doc :39 (old) | `style.ornament()` at `widgets/chrome.rs:372` (`Chrome::DeviceFrame` arm) and `bar.rs:1733` (`PanelEcho::Wave`, which is kitsch's echo, not neokitsch's `Rings` :356) | value **b**, overridden (nix `ornament = "#634427"`, palettes.nix:45); doc **d** | doc rewritten :61-69 |
| `FIELD #2c1c14` vs `#3c1c11` | `login-trace.svg:160` `<rect x="417" y="361" width="345" height="42" fill="#3c1c11"/>` | :48; `inset: Some(FIELD)` :119 | `Ink::Inset` login :477, :513 (`Style::ink_in` -> `ornaments.inset`) | **a**, overridden (nix `inset = "#2c1c14"`, palettes.nix:46; `login-neokitsch` golden shows the nix value) | none |
| `VENEER #e3af5f` vs `#f8c678` / `#f4c078` / `#f6c27a` | `mailbox-trace.svg` selection bar `fill="#f8c678"`; `store-trace.svg:333` `#f4c078`, :412 `#f6c27a` | :43; `tape` :94, `select` :95; `widgets/surface.rs:181` reads the const directly for `Selection::Veneer` | `Ink::Select` mailbox :1268 (`head_ink`); `Ink::Tape` mailbox :1066..1122 (x8); `surface.rs:178-184` (bar, dashboard) | **a**: `select` not overridden -> `mailbox-neokitsch` golden; `tape` overridden (nix `tape = "#e3af5f"`, palettes.nix:38) | none |
| `GOLD_TEXT #e7c686` vs cores `#f5bf75` / `#f3c583` / `#e9bd7a` | `login-trace.svg` `#f5bf75`; `mailbox-trace.svg` `#f3c583`; `store-trace.svg` `#e9bd7a` | :41; `fg` :90; `banner_selected: Some((ON_VENEER, GOLD_TEXT))` :108 | `Ink::Fg` login :449..507, mailbox :661..1311 (store block uses none) | **a**, overridden (nix `fg = "#e7c686"`, palettes.nix:36) | none |
| `AMBER #fcc474` agrees with CTA `#fbc171` | `login-trace.svg` CTA bar `#fbc171` | :47; `alert` :93, `cta` :121 | `Ink::Cta` login :482, :516; `Ink::Alert` mailbox :1055, :1221 | **a** (agreement, no delta) | none |
| `ON_VENEER #3a2410` vs mail veneer ink `#7b5438` / `#895f3b` | `store-trace.svg:537` `fill="#3a2010"` (dark-on-gold); `mailbox-trace.svg` text fills `#7b5438`, `#895f3b` (the "Urgent information" / "FROM: MOM" lines; file being edited by a sibling agent, cite by content) | :49; `on_select` :96 | `Ink::OnSelect` store :1683..1766 (x23); mailbox block uses none (its veneer text is `Ink::Fixed`) | **a** (not overridden; `store-neokitsch` golden) | none |
| `Ground::Bloom { r: 0.75 }`, one `BLOOM #34344c` disc vs four-stop haze | `<radialGradient id="haze">` stops `#574568` / `#3a3853` / `#16121a` / `#0e0a0d` in all four traces (`dashboard-trace.svg` `<radialGradient id="haze">`, line 110 at time of writing) | :133-137; `BLOOM` :38, `panel` :87, `bloom` :122 | `widgets/ground.rs` only via `screens/dashboard.rs:185`: mailbox draws its own haze (`haze: &HAZE` :1200 -> `mail.rs:1199-1206`), login wash opaque, store covered by `fill_rect(.., PAGE)` :1415 | **b** | none |
| doc "doc #54-62" (README "#53-62") | `docs/sources.md:19-27`: neokitsch 64-72, title card 63 | module doc :4 (old) | -- | **d** | corrected :2-5 (README is out of scope, still says `#53-62` at `docs/neokitsch/README.md:6`) |

Counts: a 6 (FIELD, VENEER, GOLD_TEXT, AMBER, ON_VENEER, ClipTopRight
variant; 3 of them nix-overridden roles), b 5 (DeviceFrame, cut 30,
FRAME, STRATA, Bloom), c 0, d 3 (device-frame doc, STRATA doc, Behance
range).

## Neomil (`src/eras/neomil.rs`)

| delta item | trace says | code says | consumers | class | action |
|---|---|---|---|---|---|
| `Corner::Chamfer { cut: 15.0 }`, one value one corner | 8 (`mailbox-trace.svg:47` panel; `dashboard-trace.svg:33` tile TR), 9 (button BR, `login-trace.svg:224`, `mailbox-trace.svg:141`), 12 (`mailbox-trace.svg:117` row BL), 13 (`store-trace.svg:29` card TR), 15 (`login-trace.svg:105` header badge BL), 16 (`store-trace.svg:129` nav BL), 22 (`login-trace.svg:47` avatar BL), 24 (`store-trace.svg:356`), 42 (`components.svg:1063` info panel BL), 51 (`login-trace.svg:196` card TR) | :94 | `widgets/surface.rs:155,166,188`, `bracket.rs`, `charts.rs:127-129` (`OpsCharts`), `screens/login.rs:897` (no `Chamfer` arm -> `pen.plate`) | **b** | none |
| `fg RED_FILL #de2e2e` darker than every bright ink; `dim RED_MID #a32226`, `border RED_DEEP #5e1112` likewise | bright inks `#f63333` (login), `#ef3333` (dashboard), `#e63132` (mailbox), `#df3131` (store); mids `#ae2729` / `#a8282b` / `#96282d`; deeps `#420f10` / `#671b21` / `#59171b` / `#5a1e22`; none of `#de2e2e` `#a32226` `#5e1112` in any trace | :42-44; `border` :68, `dim` :69, `fg` :70, `select` :75, `cta` :85 | `Ink::Fg` login :390..522, mailbox :613..839, store :979..1358 (x91); `Ink::Dim` login/mailbox/store; `Ink::Border` login :388-400, mailbox, store; `Ink::Select` mailbox :799, store :1285 | **a**: `fg`/`dim`/`border` overridden (nix identical, `home/themes/neomil/palettes.nix:28-30`); `RED_FILL` as `select`/`cta` **not** overridden (`mailbox-neomil`, `store-neomil`) | none |
| `tape OFF_WHITE #dedede`: traces sample no white | `#ffffff` / `#e3e3e3` / `#bababa` occur only as `glowv` mask stops (`login-trace.svg:78-81`, `mailbox-trace.svg:79-82`); no white design ink | :46; `tape` :72 | `palette.tape` at `widgets/chrome.rs:299` (`Chrome::Tape`, neomil's chrome :97, dashboard `top_bar`) and `screens/dashboard.rs:581`; no `Ink::Tape` in any neomil arm | **b**, overridden (nix `tape = "#dedede"`, palettes.nix:32) | none |
| `panel` / `bloom GLOW #001a33` vs `#14244e..#19274e`, band `#2a3a51 -> #101f3d` | `glowh` stops `#282824` / `#273743` / `#263953` / `#202b56` / `#1b2253` / `#171f51` / `#121f51` / `#0d1f4e` (identical in all four traces, `login-trace.svg:66-73`); ground probes `#14244e` (`dashboard-trace.svg:53`), `#19274e` (`store-trace.svg:57`). The band pair `#2a3a51 -> #101f3d` is **code** (`BAND_TOP`/`BAND_BOTTOM` :53-55), not a trace value | :41; `panel` :67, `bloom` :86 | as roles: `bloom` only via `widgets/ground.rs:48` under `Ground::Bloom` (era is `Flat`); `panel` only via `Ink::Inset` fallback (`style.rs:1077`), no neomil arm uses `Ink::Inset`. The const itself: `widgets/floppy_vector.rs:13,18,201` (floppy example, no golden) | roles **c**; const **b**-like (non-gated example) | annotated :30-40 "unconsumed as of 2026-09-03; trace value would be ..." |
| `Ground::Flat` deliberate, but all four traces draw the glow | `glowh`/`glowv` gradients present in all four traces (above) | :96 | `widgets/ground.rs` via `screens/mail.rs:158` (`haze: &[]` :739) -> `mailbox-neomil` golden; store covered by `fill_rect(.., GROUND)` :1314; login wash opaque (`login.rs:653-665` own `GLOW_H`/`GLOW_V`) | **a** (`mailbox-neomil` golden) | none |
| `Layout::OpsCharts` vs six-diamond hub | `dashboard-trace.svg:159` "the six-diamond menu" | :311 | `screens/dashboard.rs:181-187` | **b** (Layout decision) | none |
| doc "doc #44-52" (not in box) | `docs/sources.md:19-27`: neomil 53-62, no title card | module doc :4 (old) | -- | **d** | corrected :4-7 |
| (TODO-only, not in box) `Menu::Table` | -- | :307 | `widgets/menu.rs:61` via `screens/dashboard.rs:330` | **b** | none |

Counts: a 2 (reds; Ground::Flat), b 4 (Chamfer 15, OFF_WHITE,
OpsCharts, Menu::Table), c 1 (GLOW as roles), d 1 (Behance range). The
box's "band #2a3a51 to #101f3d" is the code's own `BAND_TOP`/
`BAND_BOTTOM`, not a measured trace pair; and the box's chamfer list
(8..46) omits the 51px login card chamfer `login-trace.svg:32,196`.

## Ready to apply once the Layout decision lands (class b)

**The Layout decision landed 2026-09-03 late: `Layout`, `Menu` and
`Style::{layout, menu}` were deleted and the dashboard became a
`Prim` table per era, so the two `layout` rows and the `menu` row
below are resolved by deletion.** The dashboard no longer reads any
of the remaining fields either — its ground, strokes and corners are
literal values in the `// --- dashboard ---` blocks — so every
remaining reader of these is bar-side or the widget layer under
`panels::mail`. Applying them is now purely the widget-vs-canvas
question in `TODO.md`, and they may move `bar-<era>` goldens where the
bar inherits the metric (surface stroke, corner) -- check
`tests/golden/bar-*.png` after.

| era | field | file:line | proposed value (trace) |
|---|---|---|---|
| entropism | `metrics.stroke` | `entropism.rs:268` | 1.25 (or 2.0 if the mailbox rule wins; README.md:58 says "the designed stroke is 2px") |
| entropism | `layout` | `entropism.rs:249` | whatever `Layout` fold replaces `TileRow`; trace is a 3x2 grid + detail panel + badge column |
| kitsch | `corner` | `kitsch.rs:120` | `Round { radius: 8.0 }` (dominant rx), with per-widget chamfers not expressible in one `Corner` |
| kitsch | `metrics.stroke` | `kitsch.rs:374` | 1.25 |
| kitsch | `SLAB` | `kitsch.rs:60` | `#2c9798` (and nix `bevel` at `home/themes/kitsch/palettes.nix:49`, which is what gated renders show) |
| kitsch | `BEZEL` (tape) | `kitsch.rs:45` | no trace value (no bezel traced); nix `tape` at palettes.nix:40 governs gated renders |
| neokitsch | `chrome` | `neokitsch.rs:138` | not `DeviceFrame`; trace chrome is the wire band (`README.md:63-66`) |
| neokitsch | `corner.cut` | `neokitsch.rs:130` | 22 (mailbox selection bar) / per widget |
| neokitsch | `FRAME` (border) | `neokitsch.rs:39` | `#c5965a` or `#bd8951` (outline samples); nix `border` at palettes.nix:34 governs gated renders |
| neokitsch | `STRATA` (ornament) | `neokitsch.rs:70` | none traced; nix `ornament` at palettes.nix:45 |
| neokitsch | `ground` / `BLOOM` | `neokitsch.rs:133-137`, `:38` | four-stop haze `#574568` / `#3a3853` / `#16121a` / `#0e0a0d` (already in `HAZE_*` :79-82); `Ground::Bloom` cannot express it -- and since 2026-09-05 need not: the haze is `BACKDROP` / `HUB_GROUND` composited under every neokitsch screen and the bar, and `Ground::Bloom` reaches no golden (point 2 above) |
| neomil | `corner` | `neomil.rs:94` | per widget (8/9/12/13/15/16/22/24/42/51); no single cut |
| neomil | `OFF_WHITE` (tape) | `neomil.rs:46` | no white in traces; nix `tape` at palettes.nix:32 governs gated renders |
| neomil | `layout` | `neomil.rs:311` | six-diamond hub (`dashboard-trace.svg:159`) |
| neomil | `menu` | `neomil.rs:307` | (TODO-only) follows the Layout decision |

## Would change a passing golden (class a)

Applying any of these moves at least one golden under `tests/golden/`.
"Overridden" means the golden currently shows the nix value, so the
era const alone is inert there and the nix palette must move with it.

| era | item | file:line | goldens affected | note |
|---|---|---|---|---|
| entropism | `SAGE_SOLID` (select/cta) | `entropism.rs:44` | `mailbox-entropism`, `store-entropism`, `bar-entropism` | not overridden |
| entropism | `ON_SOLID` (on_select) | `entropism.rs:49` | `mailbox-entropism`, `store-entropism` | not overridden |
| entropism | `SAGE_TEXT` (fg) | `entropism.rs:45` | all four entropism goldens via `Ink::Fg` | overridden; move `home/themes/entropism/palettes.nix:32` too |
| entropism | `OUTLINE` (border) | `entropism.rs:47` | `store-entropism` (x21 `Ink::Border`), `mailbox-entropism` (x1), bar | overridden (palettes.nix:30); **user decision** per TODO OUTLINE item |
| entropism | `BG` / `Ground::Flat` | `entropism.rs:43,86` | `mailbox-entropism` (since 2026-09-04 the store's ground is `STORE_GROUND`, composited, with the lift as a `Prim::Lobe`) | `bg` overridden (same value); `Ground` not; a mailbox lift would be a `Mailbox::backdrop` group, not a `Ground` variant |
| kitsch | `Ground::Bloom { x: .82, y: 0, r: .75 }` / `BLOOM` | `kitsch.rs:123-127,39` | none since 2026-09-04: `mailbox-kitsch` composites `MAIL_BACKDROP`, which carries the trace's `cx .5 cy -.25 r .95` | not overridden; the `Ground` value is now inert in every golden (point 2 above) |
| neokitsch | `ClipTopRight` variant | `neokitsch.rs:130` | `login-neokitsch` (`login.rs:897` badge fold) | changing the *variant* changes login badges; changing only `cut` does not |
| neokitsch | `FIELD` (inset) | `neokitsch.rs:48` | `login-neokitsch` | overridden (palettes.nix:46); trace `#3c1c11` |
| neokitsch | `VENEER` (select) | `neokitsch.rs:43` | `mailbox-neokitsch` (head_ink), bar/dashboard veneer via `surface.rs:181` | `select` not overridden; `tape` overridden |
| neokitsch | `GOLD_TEXT` (fg) | `neokitsch.rs:41` | `login-neokitsch`, `mailbox-neokitsch` | overridden (palettes.nix:36) |
| neokitsch | `ON_VENEER` (on_select) | `neokitsch.rs:49` | `store-neokitsch` | not overridden; store trace `#3a2010`, mailbox `#7b5438`/`#895f3b` -- one value cannot match both |
| neomil | `RED_FILL` (fg/select/cta) | `neomil.rs:44` | all neomil goldens via `Ink::Fg`; `mailbox-neomil`, `store-neomil` via `Ink::Select` | `fg` overridden (palettes.nix:30), `select`/`cta` not |
| neomil | `RED_MID` (dim), `RED_DEEP` (border) | `neomil.rs:43,42` | `login-`, `mailbox-`, `store-neomil` | overridden (palettes.nix:28-29) |
| neomil | `Ground::Flat` | `neomil.rs:96` | none since 2026-09-04: `mailbox-neomil` composites `MAIL_BACKDROP`, glow and vignette included | inert in every golden (point 2 above) |
