# Where each SVG reference comes from

`images/` is gitignored (the artwork is not ours to redistribute), so
this file is what keeps the pipeline reproducible: every committed SVG
names the source it traces, down to the Behance image id, or declares
itself an original. Re-fetch a source with `scripts/download_images.py`
or with the `mir-s3-cdn-cf.behance.net/project_modules/{max_1200|source}/<id>`
URL directly (browser User-Agent required). See `docs/PIPELINE.md` for
the gates.

## Run recovery, canonical positions

The Part-1 gallery (`Cyberpunk 2077 User Interface (Part 1)`,
`118663901`) holds 176 module images. Extracting every
`<id118663901.<hash>.<ext>` in first-occurrence (gallery) order gives
four contiguous era runs of nine screens, each opening with a
near-black title card:

| era | title card (pos) | screens (positions) |
|---|---|---|
| Entropism | `f5dd1e...` (33) | 34–42 |
| Kitsch | `c5cfd6...` (43) | 44–52 |
| Neo-militarism | — (opens black, `5707b7...` is the login) | 53–62 |
| Neo-kitsch | `fa1bb1...` (63) | 64–72 |

The run numbers in the era READMEs (`#23–32`, `#33–42`, `#43–52`,
`#53–62`) come from an earlier, smaller scrape and are shifted by ten
images; the Behance ids below are canonical. Runs were assigned to eras
by palette signature (see the READMEs and the last section of this
file), and the neomil ids were confirmed against the known
`img-00..09` set. `images/run-<era>/` holds one `max_1200` thumbnail
per screen, named `<position>-<id prefix>`.

## Neo-militarism (screens #53–62)

Run thumbnails in `images/run-neomil/`. Full-res sources in `images/`:

| file | Behance id | shown by |
|---|---|---|
| `images/img-06-private.png` | `6cfb20118663901.60901b2225a24.png` | PRIVATE screen |
| `images/img-07-dashboard.png` | `3fc4ef118663901.60e5fa6a7f2f7.png` | **OPS dashboard** |
| `images/img-08-main.png` | `c2e462118663901.60e5fa6a80470.png` | main console |
| `images/img-09-store.png` | `2ff48a118663901.60901b22249d2.png` | 4ST store |

| SVG | traced from |
|---|---|
| `docs/neomil/dashboard.svg` | `images/img-07-dashboard.png` |
| `docs/neomil/dashboard-trace.svg` | `images/img-07-dashboard.png` — **G1 schematic** of the photo: full-width cold-blue top band with red crest blocks, three big red chart cards side by side with dark slits, a vertical red rail on the right and a red corner block bottom-right. No sidebar logo column, no services table, no footer tape: the app's dashboard layout is not in the material. |
| `docs/neomil/target-app.svg` | `images/img-08-main.png` (NEOMIL OPS) |
| `docs/neomil/target-components.svg` | component set, sampled across the run |
| `docs/neomil/bar.svg` | **none — original composition** (no source shows a bar) |

## Entropism (screens #34–42)

Run thumbnails in `images/run-entropism/` (ids: `274a8b…60e5fa6097ef3`,
`264e0d…60901b1b6088f`, `a42bf7…60901b1b6176a`, `30ba36…60e5fa609903c`,
`3c1773…60e5fa60984dc`, `9c9903…60901b1b61cb4`, `a1de39…60e5fa609775f`,
`48e1d6…60e5fa6098aa5`, `360994…60901b1b6119b`). The run holds logins,
the mailbox tile menu, the message view and the 4ST store — all one
sage hue on a warm dark ground.

| file | Behance id | shown by |
|---|---|---|
| `images/entropism-dashboard.png` | `360994118663901.60901b1b6119b.png` (screen #42) | **module hub / tile menu** — a row of four menu tiles, the second filled solid sage (selection), caption strips beneath each tile, boxed-letter header top-left |
| `images/entropism-store.png` | `a1de39118663901.60e5fa609775f.png` (screen #40) | 4ST store — top meta strip, left category nav with five stacked rows, four product-card columns with the second filled solid sage and grown, footnote markers and footer at the foot |

| SVG | traced from |
|---|---|
| `docs/entropism/dashboard.svg` | `images/entropism-dashboard.png` |
| `docs/entropism/dashboard-trace.svg` | `images/entropism-dashboard.png` — **G1 schematic**: dim-olive top field, four tiles in a row (T2 solid bright sage = selected), caption strips, thin build-rule at the foot. The app's 3-per-row tile grid, sidebar and detail panel are not in the frame. |
| `docs/entropism/target-app.svg` | **`images/entropism-store.png`** (`a1de39118663901.60e5fa609775f.png`, screen #40) — the 4ST store. Store signature: top meta strip, left category-nav column, four product-card columns with the second solid-sage and grown (matches the traced layout of `target-app.svg`); `9c9903…` (#39) and `48e1d6…` (#41) are other scenes (photo-like sage field / twin-pane screen), not the store |
| `docs/entropism/target-components.svg` | component set, sampled across the run |
| `docs/entropism/bar.svg` | **none — original composition** |

## Kitsch (screens #44–52) — **no dashboard material**

Run thumbnails in `images/run-kitsch/` (ids: `42fe63…60e5fa6699a09`,
`546b01…60e5fa669baeb`, `227dc1…60e5fa669ad54`,
`f37b23…60901b211e32e`, `5d67ea…60e5fa669931e`,
`e6ea35…60e5fa669c12d`, `0bf802…60e5fa669a019`,
`fd108d…60e5fa669a7cd`, `75b8de…60e5fa669b49a`). The run holds a
guest login, rose-bloom fan-menu scenes (`546b01…`, `e6ea35…`), the
mailbox two-pane (`f37b23…` / `5d67ea…`, near-duplicates) and
teal-panel screens (`fd108d…`, `75b8de…`). **No screen in the run is a
module hub / dashboard: none shows a grid of similar-sized panels with
one highlighted.** The app's `dashboard.svg` fan-menu widget is drawn
from the fan scenes, but the surrounding dashboard chrome (top bar,
sidebar, badges, detail, footer) has no single source.

| SVG | traced from |
|---|---|
| `docs/kitsch/dashboard.svg` | **no dashboard source in run #44–52 — original composite** (fan widget per the fan scenes; no `dashboard-trace.svg`) |
| `docs/kitsch/target-app.svg` | **`images/kitsch-store.png`** (`75b8de118663901.60e5fa669b49a.png`, screen #52) — the 4ST store. Store signature: rose bloom over the top, logotype block top-left, left category-nav list, four yellow-header product cards with the second held tall and amber-filled (selected & grown), teal button rows and footnote/footer marks below; `fd108d…` (#51) is a different scene (broad yellow band over text rows), not the store |
| `docs/kitsch/target-components.svg` | component set, sampled across the run |
| `docs/kitsch/bar.svg` | **none — original composition** |

## Neo-kitsch (screens #64–72) — **no dashboard material**

Run thumbnails in `images/run-neokitsch/` (ids: `f06cd1…60e5fa6e2f469`,
`7b4675…60eb41268d8c7`, `3b756e…60e5fa6e2ddd7`,
`6767b7…60901b230c2a3`, `da8eac…60e5fa6e2ea42`,
`17a5c4…60e5fa6e30417`, `a43e76…60901b230a734`,
`f1104d…60e5fa6e2fce8`, `ca9fd2…60901b230b10d`). The run holds the
washed-out login, the card-cascade scene (`7b4675…`), gold strip +
highlighted-panel inboxes (`3b756e…`/`17a5c4…`, near-duplicates), the
two-pane mailbox (`6767b7…`, `da8eac…`, `f1104d…`) and a busy panel
screen (`ca9fd2…`). **No module hub / dashboard screen exists in the
run.** The app's `dashboard.svg` card-cascade widget is drawn from the
cascade scene, but the dashboard as a whole has no single source.

| SVG | traced from |
|---|---|
| `docs/neokitsch/dashboard.svg` | **no dashboard source in run #64–72 — original composite** (cascade widget per `7b4675…`; no `dashboard-trace.svg`) |
| `docs/neokitsch/target-app.svg` | **`images/neokitsch-store.png`** (`ca9fd2118663901.60901b230b10d.png`, screen #72) — the 4ST store. Store signature: amber logotype block upper right, left category-nav column, four product-card columns with the second tall and gold/veneer-filled (selected & grown), gold button rows and footnote/footer marks below; `a43e76…` (#70) is a twin-gold-panel scene, not the store |
| `docs/neokitsch/target-components.svg` | component set, sampled across the run |
| `docs/neokitsch/bar.svg` | **none — original composition** |

## G1 measurements (source photo → trace)

`scripts/compare_ref.py <source> <trace render> --regions`, sources
scaled to the trace's 1600x900 geometry:

| era | source | trace | layout | palette |
|---|---|---|---|---|
| neomil | `img-07-dashboard.png` | `dashboard-trace.svg` | **0.560** | 0.447 |
| entropism | `entropism-dashboard.png` | `dashboard-trace.svg` | **0.552** | 0.318 |

The pipeline's "unrelated scenes" baseline is ~0.15, so both traces
read; edge correlation stays low (~0.13–0.15) because the photos carry
text and texture a schematic cannot. App-vs-material numbers live in
the audit report; both identified sources diverge from the app's
dashboard per region (the source's menu area is a 3-up red chart strip
for neomil and a 4-tile sage row for entropism — not the app's
table/tile grid with sidebar and detail panel).

## Identifying a dashboard source by palette signature

Each era's dashboard is the same module-hub screen in that era's dress.
If a candidate source is ambiguous, the dominant colour tells you which
era it is: entropism is one sage hue, kitsch teal+yellow (+rose bloom),
neomil three reds (+cold blue glow), neokitsch gold+wood (+violet
haze). Amber vs rose is the trap — gold/wood reads red at a glance
until you separate hue from value: `#f4c474` and `#e7c686` are
neokitsch, `#a63355`/`#6c1c3d` rose is kitsch, `#e03030` is neomil.