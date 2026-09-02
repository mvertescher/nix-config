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
| `images/img-06-private.png` | `6cfb20118663901.60901b2225a24.png` | #59 **login** — three user cards (USER 01 with a password field + ENTER, two PROTECTED) |
| `images/img-07-dashboard.png` | `3fc4ef118663901.60e5fa6a7f2f7.png` | #60 **hub** — six-diamond menu + GO HOME panel |
| `images/img-08-main.png` | `c2e462118663901.60e5fa6a80470.png` | #61 **mail** — eight-row message list, Urgent Information panel, four action buttons |
| `images/img-09-store.png` | `2ff48a118663901.60901b22249d2.png` | #62 **store** |

| SVG | traced from |
|---|---|
| `docs/neomil/dashboard.svg` | `images/img-07-dashboard.png` |
| `docs/neomil/dashboard-trace.svg` | `images/img-07-dashboard.png` — **G1 schematic** of the photo, rewritten 2026-09-01 from measured geometry (`scripts/extract_spec.py`): a broad blue glow over near-black that is gone by y~420 (not a band with an edge); a header of CUSTOMER + one chamfered LEVEL badge, the "next TECHNOLOGY" logotype, and SECURITY LEVEL + four chamfered LEVEL badges with the second filled; a hairline rule at y=187; a tab row (COMPUTER SYSTEMS, DESCRIPTION); **a six-diamond staggered menu**, half-diagonal 104, centres (334,460) (530,460) (725,460) / (431,593) (628,592) (822,592), labelled above row 1 and below row 2; a chamfered GO HOME info panel at x 1128..1358, y 313..756 with a scrollbar rail; rotated micro-text down both margins and a footer tape. |
| `docs/neomil/target-app.svg` | **none — original composition, despite the old "traced from `img-08-main.png`" claim.** Opened side by side 2026-09-02: `img-08-main.png` is a message list (eight rows with a disc icon each, three flagged NEW) beside an "Urgent Information (!)" chamfered panel over four action buttons (Switch Weapon filled). The SVG is a NEOMIL OPS services table + sessiond panel sharing only the palette and the header grammar. Same failure as the old dashboard claims; treat as an original until retraced. |
| `docs/neomil/target-components.svg` | component set, sampled across the run |
| `docs/neomil/bar.svg` | **none — original composition** (no source shows a bar) |

## Entropism (screens #34–42)

Run thumbnails in `images/run-entropism/` (ids: `274a8b…60e5fa6097ef3`,
`264e0d…60901b1b6088f`, `a42bf7…60901b1b6176a`, `30ba36…60e5fa609903c`,
`3c1773…60e5fa60984dc`, `9c9903…60901b1b61cb4`, `a1de39…60e5fa609775f`,
`48e1d6…60e5fa6098aa5`, `360994…60901b1b6119b`). The run holds logins,
the mailbox tile menu, the message view and the 4ST store — all one
sage hue on a warm dark ground.

> **The two entropism files are named the wrong way round.** Verified
> 2026-09-01 by opening them: `entropism-dashboard.png` is the 4ST store
> and `entropism-store.png` is the module hub. The descriptions below are
> what each file actually contains; the *names* are left alone because
> `src/style.rs`, `src/screens/dashboard.rs` and `scripts/fidelity_check.sh`
> all refer to them, but nothing in this repo should be trusted to have
> read them. The Behance ids in this table were assigned from the same
> unread state and are **not** verified — only the file contents are.

| file | Behance id | actually shows |
|---|---|---|
| `images/entropism-dashboard.png` | `360994118663901.60901b1b6119b.png` (#42) | **the 4ST store**, despite the name: top status strip (DIGITAL DISTRIBUTION SOFTWAREV2 / STORE ACCESS SCREEN / FLAIR TRS 5MMP), the "4ST STORE" logotype block, a CUSTOMER / #NC488402 info block, a left category nav of five rows (RIFLES / **SMG**, filled sage / SNIPER / SHOTGUN / PISTOL) and four MAGNUM 650 HAND GUN product cards — yellow PETROCHEM / BETTERLIFE TEC band, weapon illustration, DPS/PNT/ACC/ROF stat row, socket row — with the **first** card grown to show recoil/spread/range and bonus lines. Footer tape: INTERFACE LOADED / PROVIDED BY NEXUS NETWORK V10.8 / BUILD 6.47.48441.R15 |
| `images/entropism-login.png` | `9c9903118663901.60901b1b61cb4.png` (#39) | **login**, fetched 2026-09-02: the top status strip, a lone USERNAME field with a filled NEXT button at mid-height, and a tall solid-sage footer band (y~830..950 of 1080) carrying INTERFACE LOADED / PROVIDED BY NEXUS NETWORK V10.8 / BUILD. `264e0d…` (#35) is the same screen photographed at an angle, `a42bf7…` (#36) the annotated variant |
| `images/entropism-store.png` | `a1de39118663901.60e5fa609775f.png` (#40) | **the module hub**, despite the name: the same top status strip, an `A` MAIL BOX heading over a **3x2 grid of six tiles** (EMAILS, MATRIX, BRAINDANCE / SECURITY SYSTEMS, PRIVATE, DEVICES) with BRAINDANCE filled solid sage as the selection and a two-cell caption strip along each tile's foot; a `B` MESSAGE detail panel to its right holding the BRAINDANCE heading and two lorem paragraphs; and a `C` SECURITY LEVEL column of four badges T1–T4 with T2 filled. Same footer tape |
| `images/entropism-mail.png` | `48e1d6118663901.60e5fa6098aa5.png` (#41) | **mail**, fetched 2026-09-02: `A` MAIL BOX list of seven rows (envelope glyph, title, FROM: line) with the first row filled sage; `B` MESSAGE panel with a filled URGENT INFORMATION (!) title bar over three lorem paragraphs and a REPLY / FORWARD / DELETE / **REPORT SPAM** (filled) button row beneath; `C` ENCRIPTION LEVEL 2x2 badges T1 T3 / T2 T4 with T2 filled. Same footer tape |

| SVG | traced from |
|---|---|
| `docs/entropism/dashboard.svg` | `images/entropism-dashboard.png` |
| `docs/entropism/dashboard-trace.svg` | **fabricated — do not trust, not yet rewritten.** It claims to trace `images/entropism-dashboard.png` as "four tiles in a row, caption strips, a build-rule at the foot", and concludes "the app's 3-per-row tile grid, sidebar and detail panel are not in the frame". Every part of that is wrong: the file it names is the *store*, and the hub material (`entropism-store.png`) does show a 3-per-row tile grid **and** a detail panel **and** a badge column. Rewrite it against `entropism-store.png` the way `docs/neomil/dashboard-trace.svg` was, then gate it with `fidelity_check.sh --inventory`. |
| `docs/entropism/target-app.svg` | **`images/entropism-dashboard.png`** (the store — see the name-swap warning) — right screen, **loose composite**, checked by eye 2026-09-02: the photo grows the **first** card (the SVG grows the second); every card carries a pale-filled header block from the title down through the DPS/PNT/ACC/ROF row, with a **yellow PETROCHEM / BETTERLIFE TEC band** at y~320 (the SVG has no pale blocks and no band); cards are ~265 wide at a 325 pitch with the fourth bleeding off the right edge (SVG: 228 @ 250, all on canvas); the nav has five rows plus a tall empty sixth cell. Rework against the photo before using it as a target. |
| `docs/entropism/target-components.svg` | component set, sampled across the run |
| `docs/entropism/bar.svg` | **none — original composition** |

## Kitsch (screens #44–52) — the hub was here all along

Run thumbnails in `images/run-kitsch/` (ids: `42fe63…60e5fa6699a09`,
`546b01…60e5fa669baeb`, `227dc1…60e5fa669ad54`,
`f37b23…60901b211e32e`, `5d67ea…60e5fa669931e`,
`e6ea35…60e5fa669c12d`, `0bf802…60e5fa669a019`,
`fd108d…60e5fa669a7cd`, `75b8de…60e5fa669b49a`).

> **Correction (2026-09-01, by opening the run).** This section used to
> assert *"no screen in the run is a module hub / dashboard"*. False:
> `e6ea35…` (#49) **is the module hub** — two three-blade fans of
> rounded cards naming six modules (VEHICLES, WEAPONS, PRODUCTS /
> PRODUCTS, EVENTS, LOCATIONS) with EVENTS solid yellow as the
> selection, an A USER box, C SECURITY LEVEL badges 01–04 with 02
> filled, and a D DESCRIPTION panel titled BRAINDANCE. `227dc1…` (#46)
> is the same screen with presentation annotations in the margin, and
> `546b01…` (#45) is the fan laid nearly flat. The old text was written
> without opening the images; the claim survived because nothing
> checked it. The full-res source is downloaded as
> `images/kitsch-dashboard.png` (`e6ea35118663901.60e5fa669c12d.png`).

The rest of the run: guest login (`0bf802…`, #50), two shots of the
UI on a wall-mounted screen pair (`f37b23…` #47 hub+store, `5d67ea…`
#48 hub+mail), the **mail screen** (`fd108d…`, #51 — an earlier note
called it "a broad yellow band over text rows"; it is the mailbox,
see the table), and the 4ST store (`75b8de…`, #52).

Full-res sources in `images/` (all `scripts/download_images.py`):

| file | Behance id | shows |
|---|---|---|
| `images/kitsch-dashboard.png` | `e6ea35118663901.60e5fa669c12d.png` (#49) | **hub** — the two fans, see the correction above |
| `images/kitsch-login.png` | `0bf802118663901.60e5fa669a019.png` (#50) | **login**, fetched 2026-09-02: 10:20 PM top centre; three GUEST 7702 cards in a row (teal chip glyph + name + micro-text), the first inside a tall teal bracket outline that runs from y~470 down past a filled ENTER bar to a barcode at y~780..820, the other two closed by a dark PROTECTED bar; ARASAKA CONSUMER TECHNOLOGY footer line |
| `images/kitsch-mail.png` | `fd108d118663901.60e5fa669a7cd.png` (#51) | **mail**, fetched 2026-09-02: A USER (GUES 7702), B DESCRIPTION (MAILBOX), C SECURITY LEVEL 01–**02**–03–04 boxes in the header; a five-row message list inside the same bracket-and-wave outline the store's nav uses (wave at bottom-left), the selected row a solid yellow chevron; a yellow-outlined message panel with a solid yellow title bar and three lorem paragraphs; a DETAILS yellow chevron over MODS / PRICE / DAMAGE outlined chevrons at the right |
| `images/kitsch-store.png` | `75b8de118663901.60e5fa669b49a.png` (#52) | **store** — see the `target-app.svg` row |

| SVG | traced from |
|---|---|
| `docs/kitsch/dashboard-trace.svg` | `images/kitsch-dashboard.png` — **G1 schematic**, written 2026-09-01 from measured geometry (PCA on the ink masks): two 3-blade fans of 190x60 rounded cards, hubs ~(455,470) and ~(825,535), EVENTS solid yellow, ghost stacks per blade that all recede up-right (~-50 deg) in screen space — one shared depth direction, not each card's normal (fixed 2026-09-02; the previous per-normal version sent WEAPONS and EVENTS up-left and still scored 0.47); USER box, 01–04 badges, BRAINDANCE panel, A/B/C/D letter boxes. Gated by ink placement (`fidelity_check.sh --inventory kitsch dashboard`): faithful 0.54 vs 0.07 for the old app composite. |
| `docs/kitsch/dashboard.svg` | **original composite, now with a known source it ignores** — drawn before the hub was found; its fan widget came from the fan scenes but its chrome matches the app, not `kitsch-dashboard.png`. Scores 0.07 on the ink gate. Rework against the trace. |
| `docs/kitsch/target-app.svg` | **`images/kitsch-store.png`** (`75b8de118663901.60e5fa669b49a.png`, screen #52) — the 4ST store. Store signature: rose bloom over the top, logotype block top-left, left category-nav list, four yellow-header product cards with the second held tall and amber-filled (selected & grown), teal button rows and footnote/footer marks below; `fd108d…` (#51) is the mail screen, not the store. Checked by eye 2026-09-02: the closest of the four `target-app` files, composition and grown card right. Gaps: nav rows are 220-wide chevrons inside a bracket outline that ends in a large teal wave (x~115..355, y~600..720) — the SVG has 160-wide rounded rects and a small blob; cards ~285 wide at a 320 pitch with the fourth off-canvas (SVG 250 @ 275); the yellow band is ~40px tall with a flag notch past the card edge (SVG ~20px, no notch). |
| `docs/kitsch/target-components.svg` | component set, sampled across the run |
| `docs/kitsch/bar.svg` | **none — original composition** |

## Neo-kitsch (screens #64–72) — the hub was here all along

Run thumbnails in `images/run-neokitsch/` (ids: `f06cd1…60e5fa6e2f469`,
`7b4675…60eb41268d8c7`, `3b756e…60e5fa6e2ddd7`,
`6767b7…60901b230c2a3`, `da8eac…60e5fa6e2ea42`,
`17a5c4…60e5fa6e30417`, `a43e76…60901b230a734`,
`f1104d…60e5fa6e2fce8`, `ca9fd2…60901b230b10d`).

> **Correction (2026-09-01, by opening the run).** This section used to
> assert *"no module hub / dashboard screen exists in the run"*, and it
> misdescribed the scenes: `3b756e…`/`17a5c4…` are not "gold strip
> inboxes" — **they are the module hub**: six cascade cards in two
> staircase triplets naming EMAIL (solid gold, the selection), MATRIX,
> BRAINDANCE, PRIVATE, SECURITY SYSTEMS, DEVICES, with an EMAIL detail
> panel and LEVEL T1–T4 badges, T2 filled. And `7b4675…` is not "the
> card-cascade scene" — it is a phase/login screen with two entry bars
> over a circuit ground. `17a5c4…` (#69) is the clean full-bleed hub,
> `3b756e…` (#66) the annotated variant. The full-res source is
> downloaded as `images/neokitsch-dashboard.png`
> (`17a5c4118663901.60e5fa6e30417.png`).

The rest of the run: the era overview board (`f06cd1…`, #64), a login
over a circuit ground (`7b4675…`, #65), two shots of the UI on a
wall-mounted screen pair (`6767b7…` #67 login+store, `da8eac…` #68
hub+mail), the **clean login** (`a43e76…`, #70 — an earlier note called
it "a twin-gold-panel scene"; it is the ARASAKA login, see the table),
the **mail screen** (`f1104d…`, #71) and the 4ST store (`ca9fd2…`, #72).

Full-res sources in `images/` (all `scripts/download_images.py`):

| file | Behance id | shows |
|---|---|---|
| `images/neokitsch-dashboard.png` | `17a5c4118663901.60e5fa6e30417.png` (#69) | **hub** — the six cascade cards, see the correction above |
| `images/neokitsch-login.png` | `a43e76118663901.60901b230a734.png` (#70) | **login**, fetched 2026-09-02: ARASAKA logotype top-left, 10:10 PM / NIGHT CITY AREA top-right, a purple-to-black glow across the top; two PRASE_6054012 entry groups side by side at mid-height (label, dark field, filled ENTER / LOGIN bar, A/B letter box + micro-text); the stacked-hairline wire band forming a wide trapezoid across the foot (y~890..1035 at 1080p, the same band the mail and store screens use as their header); ARASAKA CONSUMER TECHNOLOGY footer line |
| `images/neokitsch-mail.png` | `f1104d118663901.60e5fa6e2fce8.png` (#71) | **mail**, fetched 2026-09-02: CUSTOMER #NC488402 + LEVEL T1 at top-left, SECURITY LEVEL T1–**T2**–T3–T4 top-right with T2 carrying the tab-badge; the wire band with A and B letter boxes runs under the header; a seven-row message list (title, FROM:, envelope glyph at the right) with the second row a solid gold chamfered bar; URGENT INFORMATION (!) heading and three lorem paragraphs in plain gold text, no panel outline; four RIFLES outlined buttons along the foot; C and D letter boxes bottom-left |
| `images/neokitsch-store.png` | `ca9fd2118663901.60901b230b10d.png` (#72) | **store** — see the `target-app.svg` row |

| SVG | traced from |
|---|---|
| `docs/neokitsch/dashboard-trace.svg` | `images/neokitsch-dashboard.png` — **G1 schematic**, written 2026-09-01 from measured geometry: six 93x327 cards (chamfer top-right + bottom-left, left-edge tab) in staircase triplets at (244,383)(347,284)(449,182) and (624,384)(724,284)(826,182), EMAIL solid gold, concentric onion outlines stepping up-left; detail panel at (1168,253) with solid body [1171,326,231,309]; wire band, letter boxes, T1–T4 badges. Gated by ink placement: faithful 0.51 vs 0.03 for the old app composite. |
| `docs/neokitsch/dashboard.svg` | **original composite, now with a known source it ignores** — its cascade widget was credited to `7b4675…` (actually a login screen); the real hub went unread. Scores 0.03 on the ink gate. Rework against the trace. |
| `docs/neokitsch/target-app.svg` | **`images/neokitsch-store.png`** (`ca9fd2118663901.60901b230b10d.png`, screen #72) — the 4ST store. Store signature: BASKET box upper right, logotype top-left, left category-nav column, four product-card columns with the second tall and gold/veneer-filled (selected & grown); `a43e76…` (#70) is the login, not the store. Checked by eye 2026-09-02: right screen, but the SVG **invents a full-screen chamfered frame** the photo does not have — the photo's only frame is the stacked-hairline wire band with an S-bend at y~120..200; the cards lack the onion outlines that are the era's signature; cards are ~265 wide spanning x 360..1550 (SVG 200 wide, 420..1320); and the "ARASAKA CONSUMER TECHNOLOGY" footer is borrowed from kitsch — the neokitsch store has none. Rework against the photo before using it as a target. |
| `docs/neokitsch/target-components.svg` | component set, sampled across the run |
| `docs/neokitsch/bar.svg` | **none — original composition** |

## G1 measurements (source photo → trace)

**A grid-correlation score cannot tell you a trace is a fabrication.**
The neomil row below used to read 0.560 layout — comfortably over the
~0.15 "unrelated scenes" baseline — for a trace that drew three red
chart cards, a right rail and a corner block against a photo that holds
a six-diamond menu and a chamfered info panel. Nothing in the source is
a chart card. A blue mass on top and red mass in the middle is all it
takes to score 0.56, which is why these numbers are directional only and
why `scripts/spec_diff.py` now exists.

Prefer the inventory gate, which has a pass/fail:

    scripts/fidelity_check.sh --inventory <era> dashboard

Two
modes, picked automatically per era: **shape inventory** for
axis-aligned design languages, **ink placement** where rotated,
overlapping or translucent geometry (kitsch's fans, neokitsch's
cascades) fragments unstably under the shape templates — there the
verdict rides per-colour-family 80x45 occupancy IoU instead. Both
modes are calibrated against a known-wrong drawing (the old app-style
`dashboard.svg` composites):

| era | source | gate | faithful trace | wrong drawing |
|---|---|---|---|---|
| neomil | `img-07-dashboard.png` | shapes | **PASS**, 92% area, median 1.4px | 0% (the old fabricated trace) |
| entropism | `entropism-store.png` (the hub — see the name-swap warning) | shapes | **PASS**, 92% area, median 1.5px | — |
| kitsch | `kitsch-dashboard.png` | inks | **PASS**, weighted IoU 0.54 | 0.07 |
| neokitsch | `neokitsch-dashboard.png` | inks | **PASS**, weighted IoU 0.51 | 0.03 |

**Every pre-2026-09-01 dashboard claim in this file was wrong when
checked against the material.** Neomil's and entropism's traces were
invented; kitsch's and neokitsch's "no dashboard material" claims were
false — both runs hold a full module hub. All four eras show the same
screen grammar in four dressings: a menu of six modules with one
selected (diamonds / tiles / fan blades / cascade cards), a detail
panel describing the selection, and a security-level badge row with
the second badge filled. Nothing in this file that predates these
corrections should be trusted without opening the image it describes.

## Identifying a dashboard source by palette signature

Each era's dashboard is the same module-hub screen in that era's dress.
If a candidate source is ambiguous, the dominant colour tells you which
era it is: entropism is one sage hue, kitsch teal+yellow (+rose bloom),
neomil three reds (+cold blue glow), neokitsch gold+wood (+violet
haze). Amber vs rose is the trap — gold/wood reads red at a glance
until you separate hue from value: `#f4c474` and `#e7c686` are
neokitsch, `#a63355`/`#6c1c3d` rose is kitsch, `#e03030` is neomil.