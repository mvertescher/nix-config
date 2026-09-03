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
| `docs/neomil/login-trace.svg` | `images/img-06-private.png` (#59) — **G1 schematic**, 2026-09-02: the hub's header (CUSTOMER / LEVEL T1 badge, #NC488402 block, SECURITY LEVEL T1–T4 with T2 filled) over a hairline rule; three USER 01 cards at x 375..637 / 700..962 / 987..1250, y 312..675 with a 13px top-right chamfer and a notched left tab — card 1 solid red with a chip glyph, cards 2 and 3 dark translucent with a portrait and a CC35 micro-caption; under card 1 a `password:` label, a dotted field and a `Login` bar, both outlined red; rotated micro-text in both margins. Gate: PASS, 79% area (the source's four "diamonds" are the extractor fitting the portraits' faces and a card-fill fragment; below the class-share threshold). |
| `docs/neomil/mailbox-trace.svg` | `images/img-08-main.png` (#61) — **G1 schematic**, 2026-09-02: the same header plus a COMPUTER SYSTEMS / CONTENT tab row; a column of eight disc icons at x~100 with a message list beside it, rows `List of messages / Jackie` (first, filled red, with a NEW pill) then I'm worried man / Heist data sent to you alternating, three rows flagged NEW; the `Urgent Information (!)` heading over an outlined chamfered message panel x 940..1255, y 315..755 with four lorem paragraphs; a button row at y 760..790, `Switch Weapon` filled and three `Confirm / Jump` outlined; a small R/arrows glyph stack between list and panel. Gate: PASS, 86% area. |
| `docs/neomil/store-trace.svg` | `images/img-09-store.png` (#62) — **G1 schematic**, 2026-09-02: NO header row — a KIROSHI chip strip at y 31..47, the MASURAO logotype (kanji 益荒男 over a slanted MASURAO band) top-left, a filled CUSTOMER #NC488402 bar, LOYALTY DISCOUNT / LAST UPDATE lines, a five-row nav (VIDEO filled, AUDIO, GAMEPLAY, CYBERWARE, CONTROLLER) with a 16px bottom-left chamfer and a spine; four MAGNUM 650 HAND GUN cards at x 437 / 769 / 1096 / 1425, tops y 151, bottoms y 613, each with a bright right-edge bar y 266..416, three head/foot icons, a pistol drawing, DPS PNT ACC ROF over 86 30 5 5, a socket row; card 2 selected — taller (to y 800), lighter upper fill, solid pistol, 20 RECOIL / 22 SPERAD / 12 RANGE, BONUS +9 REFLEXES / +2 MODULES SLOTS, a second socket row; card 4 cut by the frame at x~1555. Gate: PASS, 71% area (second pass 2026-09-02; was FAIL 55 with the rifles as silhouettes). The rifles are now four layered symbols measured on a 10px grid — body, hatched forend, highlights, detail lines — card 2's solid and 14px further left as in the photo; the kanji is real Noto CJK text on the slanted band; card 4 ends in a plain cut at x 1557 (the chamfer and right-edge bar there were invented) and its socket glyph is the source's 24-cell scatter, not a finder-square QR. Only card 4 ever had a clipPath (fixed earlier the same day). |
| `docs/neomil/target-app.svg` | **Superseded by `mailbox-trace.svg` and `store-trace.svg` (2026-09-02).** **none — original composition, despite the old "traced from `img-08-main.png`" claim.** Opened side by side 2026-09-02: `img-08-main.png` is a message list (eight rows with a disc icon each, three flagged NEW) beside an "Urgent Information (!)" chamfered panel over four action buttons (Switch Weapon filled). The SVG is a NEOMIL OPS services table + sessiond panel sharing only the palette and the header grammar. Same failure as the old dashboard claims; treat as an original until retraced. |
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
| `docs/entropism/dashboard-trace.svg` | `images/entropism-store.png` (the hub — see the name-swap warning) — **G1 schematic**, rewritten 2026-09-01 from measured geometry the way the neomil one was; gated by `fidelity_check.sh --inventory entropism dashboard` (shapes mode, 92% area). The previous file was a fabrication that named the wrong source and concluded the hub's grid, sidebar and detail panel "are not in the frame"; the row above it saying so is retired. Checked by eye 2026-09-02; footer fixed the same day: only BUILD is right-anchored (text-anchor end at 1525), PROVIDED BY is left-anchored at 519 as on the mailbox — the trace had it 180px too far right. Still open: the header strip strings are ~8% short and ~40px left (font-size 14 vs the mailbox's 15). |
| `docs/entropism/login-trace.svg` | `images/entropism-login.png` (#39) — **G1 schematic**, 2026-09-02: the outlined header strip x 49..1547, y 43..69 with dividers at 465/1353; an empty upper two-thirds; `USERNAME:` (baseline 403) over an outlined field x 563..922, y 414..447 of eleven asterisks with a caret underline, and a solid NEXT button x 932..1037; a tall solid sage footer band x 36..1565, y 765..880 (12% of the frame) carrying dark INTERFACE LOADED / PROVIDED BY NEXUS NETWORK V10.8 / BUILD 6.47.48441.R15 — no outline box, unlike the other screens' thin footer strip. Gate: PASS, 83% area (the source's ten "chamfers" are header letterforms; below the class-share threshold). |
| `docs/entropism/mailbox-trace.svg` | `images/entropism-mail.png` (#41) — **G1 schematic**, 2026-09-02: header strip; A MAIL BOX / B MESSAGE / C ENCRIPTION LEVEL letter boxes at y 98..124; the list outlined x 84..451, y 205..686, seven rows on a 62px pitch, row 1 (YOU'LL REGRET THAT / FROM: JACKIE) filled solid, envelope glyph per row, titles URGENT INFORMATION (!) / HEIST DATA SENT TO YOU / I'M WORRIED MAN / SPECIAL OFFER TO YOU! / I'M WORRIED MAN / SPECIAL OFFER TO YOU! with FROM: MOM / 805000451 / RACHEL ROSS / JINX JINX STORE / BIALA ROBERTSON / LARIX & BETULA; the MESSAGE panel x 529..1279, y 205..686 with a filled title bar y 227..288 over three lorem paragraphs; a button row y 694..746, REPLY / FORWARD / DELETE outlined and REPORT SPAM filled; 2x2 badges of 69px at x 1341/1418, y 227/305 reading T1 T3 / T2 T4 with T2 filled; outlined footer strip y 847..873. Gate: PASS, 88% area (second pass 2026-09-02; was FAIL 60). Every outlined frame in the material photographs as a 2px stroke with a 1px near-black undershoot and a faint sage overshoot 2–4px out; drawn as three concentric strokes per section (`#25281d` 7px / `#0a0a02` 4px / `#709174` 2px) and recorded in the header as a photographic halo, not a designed glow — the iced side draws only the 2px stroke. Envelope glyphs rebuilt from 10x crops (closed 17x11 with flap V, open 17x16 with a diamond flap) at the measured row tops. The "double-fitted frame" was actually a text-halo blob in rows 2..7 bounded by the selected row; the added ring pairs with it by bbox coincidence. |
| `docs/entropism/store-trace.svg` | `images/entropism-dashboard.png` (#42, the store despite the name) — **G1 schematic**, 2026-09-02: header strip (DIGITAL DISTRIBUTION SOFTWAREV2); the 4ST logotype x 134..308, y 102..160 with 4S solid and T outlined, S T O R E beneath; an outlined CUSTOMER / #NC488402 box and LOYALTY DISCOUNT / LAST UPDATE lines; the nav x 112..330, y 301..740 — RIFLES, SMG (filled), SNIPER, SHOTGUN, PISTOL, then one tall empty cell; four MAGNUM 650 HAND GUN cards 265 wide on a 322 pitch at x 461/783/1105/1429 (the fourth clipped), each with a yellow PETROCHEM / BETTERLIFE TEC band y 315..335, a rifle, DPS/PNT/ACC/ROF over a filled 86/30/5/5 row y 469..494, a socket row and a CC35 micro-caption; the **first** card grown, header filled through the values row and continuing to y 720 with 20 Recoil / 22 Sperad / 12 Range, Bonus / +9 Reflexes / +2 Modules Slots; A and B letter boxes at y 779..805; outlined footer strip. Supersedes `target-app.svg`. Gate: PASS, 80% area. |
| `docs/entropism/target-app.svg` | **Superseded by `store-trace.svg` (2026-09-02); kept until the iced store screen is rebuilt from the trace.** **`images/entropism-dashboard.png`** (the store — see the name-swap warning) — right screen, **loose composite**, checked by eye 2026-09-02: the photo grows the **first** card (the SVG grows the second); every card carries a pale-filled header block from the title down through the DPS/PNT/ACC/ROF row, with a **yellow PETROCHEM / BETTERLIFE TEC band** at y~320 (the SVG has no pale blocks and no band); cards are ~265 wide at a 325 pitch with the fourth bleeding off the right edge (SVG: 228 @ 250, all on canvas); the nav has five rows plus a tall empty sixth cell. Rework against the photo before using it as a target. |
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
| `docs/kitsch/dashboard-trace.svg` | `images/kitsch-dashboard.png` — **G1 schematic**, written 2026-09-01 from measured geometry (PCA on the ink masks): two 3-blade fans of 162x50 r8 cards (the 190x60 r14 first drawn was wrong; re-measured 2026-09-02 in each card's own frame — ghosts are the same size as the solid card, stepping (+20,−20) in screen space, counts 6/7/6/6/5/6, fills fading 0.58→0.12), hubs ~(455,470) and ~(825,535), EVENTS solid yellow, ghost stacks per blade that all recede up-right (~-50 deg) in screen space — one shared depth direction, not each card's normal (fixed 2026-09-02; the previous per-normal version sent WEAPONS and EVENTS up-left and still scored 0.47); USER box, 01–04 badges, BRAINDANCE panel, A/B/C/D letter boxes. Gated by ink placement (`fidelity_check.sh --inventory kitsch dashboard`): faithful 0.59 (0.54 before the card resize and the mint footer — it had been drawn yellow) vs 0.07 for the old app composite. |
| `docs/kitsch/login-trace.svg` | `images/kitsch-login.png` (#50) — **G1 schematic**, 2026-09-02: rose bloom top-centre gone by y~420; 10:20 PM at x 781..852; a 2px mint bracket running the full frame height (fading into the bloom above y~130), left x 228.5, right x 611.5, whose left edge breaks at y 540 into a ~57° diagonal to (338,635), drops to 700 and rounds (r~30) into a bottom at y 731, with a dark-teal lobe outside the diagonal; three GUEST 7702 cards on a 393px pitch (x 258/651/1043) — 62x61 chip glyph, name, boxed A + ACCESS MANAGER micro-text; card 1 has a dark input field [257,413,335,51] and a solid mint ENTER bar y 470..497 stepping up at its right, cards 2/3 the same bar in dark teal reading PROTECTED; a barcode in the bracket foot x 377..590, y 632..683 with digits 12345678123456789; ARASAKA footer micro-text. Gate: PASS, inks 0.54 (0.47 first pass; barcode regenerated from the column profile to 50 bars, bracket 1.3px as measured, chip glyph drawn, footer to the shared mint size 9). |
| `docs/kitsch/mailbox-trace.svg` | `images/kitsch-mail.png` (#51) — **G1 schematic**, 2026-09-02: same bloom; boxed A/B/C at x 164/586/1216, labels USER / DESCRIPTION / SECURITY LEVEL, notched boxes GUES 7702 and MAILBOX (y 185..236), four 56x34 badges with 02 filled orange; a left bracket outline from x 120.5, y 268 down to a solid teal wave y 606..748 sweeping right to x~376; five rows on a 60px pitch — YOU'LL REGRET THAT / Jackie selected as a yellow icon cell + chamfered body y 313..351, then URGENT INFORMATION (!) / Mom, HEIST DATA SENT TO YOU / 805000451, I'M WORRIED MAN / Rachel Ross, SPECIAL OFFER TO YOU! / JINX JINX STORE; a solid yellow message tab x 575..1127, y 313..349 over a yellow-outlined body to y 748 with a top-left notch and ten lorem lines; four chevrons at x 1216 (DETAILS solid yellow; MODS / PRICE / DAMAGE teal outline). Gate: PASS, inks 0.62 (0.46 first pass; body text 16.8/500 to the measured line extents, panel outline 1.25px on .5 coordinates r8, footer fixed). |
| `docs/kitsch/store-trace.svg` | `images/kitsch-store.png` (#52) — **G1 schematic**, 2026-09-02: same bloom; 4S solid + outlined T logotype (x 155..318), S T O R E beneath; a rounded customer chip x 123..338, y 178..200 and loyalty/last-update lines; the nav bracket — top line y 186 S-bending round the customer block to y 268, r~30 corner into a left edge at x 106.5 that becomes a solid teal wave at y~580 (top edge y 617 to x 312, shoulder to x 345, bottom line y 718 fading by x~445); five 216x39 peaked chevrons at x 140 on a 60px pitch from y 297 (RIFLES, SMG solid #ffbe18, SNIPER, SHOTGUN, PISTOL); four 261-wide cards on a 320 pitch (x 484/804/1123/1443, fourth clipped), teal outline y 218..538 with a 24px top-right chamfer, MAGNUM 650 / HAND GUN, a solid yellow band y 277..312 with a 27px flag past the left edge, a mint gun, DPS PNT ACC ROF, a solid mint values bar y 450..480, a socket row y 492..538; card 2 amber-filled to a stepped bottom then amber-outlined to y 680 with 20 Recoil / 22 Sperad / 12 Range, Bonus / +9 Reflexes / +2 Modules Slots and a second socket row; boxed A at x 350 and C at x 1500 in the foot. Supersedes `target-app.svg`. Gate: PASS, inks 0.57 (0.55 first pass; gun silhouette from the mint-mask column profiles with evenodd holes, card 2's gun 14px higher as in the photo, band glyphs redrawn as distinct marks at measured positions). |
| `docs/kitsch/dashboard.svg` | **original composite, now with a known source it ignores** — drawn before the hub was found; its fan widget came from the fan scenes but its chrome matches the app, not `kitsch-dashboard.png`. Scores 0.07 on the ink gate. Rework against the trace. |
| `docs/kitsch/target-app.svg` | **Superseded by `store-trace.svg` (2026-09-02); kept until the iced store screen is rebuilt from the trace.** **`images/kitsch-store.png`** (`75b8de118663901.60e5fa669b49a.png`, screen #52) — the 4ST store. Store signature: rose bloom over the top, logotype block top-left, left category-nav list, four yellow-header product cards with the second held tall and amber-filled (selected & grown), teal button rows and footnote/footer marks below; `fd108d…` (#51) is the mail screen, not the store. Checked by eye 2026-09-02: the closest of the four `target-app` files, composition and grown card right. Gaps: nav rows are 220-wide chevrons inside a bracket outline that ends in a large teal wave (x~115..355, y~600..720) — the SVG has 160-wide rounded rects and a small blob; cards ~285 wide at a 320 pitch with the fourth off-canvas (SVG 250 @ 275); the yellow band is ~40px tall with a flag notch past the card edge (SVG ~20px, no notch). |
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
| `docs/neokitsch/dashboard-trace.svg` | `images/neokitsch-dashboard.png` — **G1 schematic**, written 2026-09-01 from measured geometry: six 93x327 cards (chamfer top-right + bottom-left, left-edge tab) in staircase triplets at (244,383)(347,284)(449,182) and (624,384)(724,284)(826,182), EMAIL solid gold, concentric onion outlines stepping up-left; detail panel at (1168,253) with solid body [1171,326,231,309]; wire band, letter boxes, T1–T4 badges. Gated by ink placement: 0.60 (0.51–0.54 before the halo filter shared with the three newer traces was ported in, 2026-09-02) vs 0.03 for the old app composite. |
| `docs/neokitsch/login-trace.svg` | `images/neokitsch-login.png` (#70) — **G1 schematic**, 2026-09-02: the hub's haze; ARASAKA stencil logotype x 93..278, y 59..88 with tagline and a two-cell outlined box (57ASD4AV15AA / COMBAT COLONIZATION – DEFENCE PROGRAM); 10:10 PM / NIGHT CITY / AREA top-right; two identical entry groups (second at +420 x): PRASE_6054012 label, an unoutlined chocolate field [417,361]..[762,403], a solid gold ENTER / LOGIN bar [417,414]..[761,441] with a 16x13 bottom-left cut, letter box A/B with micro-text; a 22-strand wire band as a wide trapezoid across the foot (outer plateaus y 727..809, S-bends mirrored about x=808 onto a centre plateau y 782..846), ends curling down at x 35/1564; centred footer line at y 875. Nothing else. Gate: PASS, inks 0.73. |
| `docs/neokitsch/mailbox-trace.svg` | `images/neokitsch-mail.png` (#71) — **G1 schematic**, 2026-09-02: the hub's header block verbatim (mean abs diff 2.7/255 against the hub photo); seven message rows on a 60.2 pitch from y 309 (title, FROM line, open/closed envelope glyph at x 429, hairline rule with a small tab at x 421..452), row 2 the selection as a solid gold plate [35,315]..[512,370] with a chamfered right end and a notch beneath; the message as plain text with no panel — URGENT INFORMATION (!) bold, FROM: MOM, three lorem paragraphs; four outlined RIFLES buttons 184x39 at y 684 on a 192 pitch with a bottom-left chamfer and a filled tab; letter boxes C (139,777) / D (735,777). Every glyph sits on a soft vertical glow, modelled as a blur filter. Gate: PASS, inks 0.68 (second pass 2026-09-02; was FAIL 0.61 with #c19867 unpaired). The source's gold is three-tiered — bright bar/tabs, mid text cores, dark outlines — and the first trace was one flat bright gold; the selection bar is now wood veneer (32 fine strokes, a seam at x 273), RIFLES outlines take the dark gold, list titles the mid gold, and all four source families pair (0.78 / 0.86 / 0.53 / 0.33). Still open: the halo is ~2.4x too abundant around body text. |
| `docs/neokitsch/store-trace.svg` | `images/neokitsch-store.png` (#72) — **G1 schematic**, 2026-09-02: 4S solid + T outlined logotype (x 109..277, y 66..117), S T O R E beneath; a BASKET plate [1292,20]..[1496,105] split by a hairline at y 60; an 8-strand header band rising through one S-bend (290→349, mirrored) onto a bridge line y 124 spanning 349..1230, letter boxes A (360,143) and C (1178,143); CUSTOMER / LOYALTY DISCOUNT / LAST UPDATE lines, then five outlined 200x39 buttons on a 60.7 pitch from y 358 (RIFLES, SMG solid gold, SNIPER, SHOTGUN, PISTOL) with a bottom-left cut and a solid tab; four 262-wide cards (x0 360.8 / 667.1 / 978.8 / 1288.8) with an r13 top-left, a 37° step up to a r18 top-right, and four fading echo strands round the top-right, side and bottom — rifle, DPS PNT ACC ROF / 620 30 5 5, socket row, MAGNUM 650 HAND GUN, a tab under the bottom edge; card 2 expanded (top −82, bottom +71) and solid gold across y 411..653 carrying 20 Recoil / 22 Sperad / 12 Range / Bonus / +9 Reflexes / +2 Modules Slots in dark text; letter box B (675,775). Supersedes `target-app.svg`. Gate: PASS, inks 0.60 (0.63 first pass; wood-grain strokes added on the SMG, card 2 and basket at the measured ~2.1px pitch and seams — k-means re-partitioned and the glow family fell 0.87→0.70, accepted because the overlay is closer). |
| `docs/neokitsch/dashboard.svg` | **original composite, now with a known source it ignores** — its cascade widget was credited to `7b4675…` (actually a login screen); the real hub went unread. Scores 0.03 on the ink gate. Rework against the trace. |
| `docs/neokitsch/target-app.svg` | **Superseded by `store-trace.svg` (2026-09-02); kept until the iced store screen is rebuilt from the trace.** **`images/neokitsch-store.png`** (`ca9fd2118663901.60901b230b10d.png`, screen #72) — the 4ST store. Store signature: BASKET box upper right, logotype top-left, left category-nav column, four product-card columns with the second tall and gold/veneer-filled (selected & grown); `a43e76…` (#70) is the login, not the store. Checked by eye 2026-09-02: right screen, but the SVG **invents a full-screen chamfered frame** the photo does not have — the photo's only frame is the stacked-hairline wire band with an S-bend at y~120..200; the cards lack the onion outlines that are the era's signature; cards are ~265 wide spanning x 360..1550 (SVG 200 wide, 420..1320); and the "ARASAKA CONSUMER TECHNOLOGY" footer is borrowed from kitsch — the neokitsch store has none. Rework against the photo before using it as a target. |
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
| neomil | `img-07-dashboard.png` | shapes | **PASS**, 94% area, median 1.4px | 0% (the old fabricated trace) |
| entropism | `entropism-store.png` (the hub — see the name-swap warning) | shapes | **PASS**, 92% area, median 1.5px | — |
| kitsch | `kitsch-dashboard.png` | inks | **PASS**, weighted IoU 0.59 (0.54 before the 2026-09-02 card resize) | 0.07 |
| neokitsch | `neokitsch-dashboard.png` | inks | **PASS**, weighted IoU 0.60 (0.54 before the halo was added, 2026-09-02) | 0.03 |

All sixteen screens, after the twelve login / mailbox / store traces
landed on 2026-09-02 (`fidelity_check.sh --inventory <era>` runs all
four). Every one of the twelve was checked by overlaying its render on
the photo at 50%. The "was" figures are the first-pass numbers; the
second pass the same day (one vision agent per era, see the crate TODO
§ Trace improvements) cleared the three by-number fails and lifted the
rest — neomil store by drawing the rifles as segmented line art and the
kanji as text, entropism mailbox by adding the measured three-stroke
edge profile every entropism frame photographs with, neokitsch mailbox
by splitting the gold into the source's bright/mid/dark tiers so
`#c19867` pairs. Neokitsch store dropped 0.03 when its wood-grain
strokes re-partitioned k-means; accepted, the overlay is closer:

| era | login | dashboard | mailbox | store |
|---|---|---|---|---|
| neomil (shapes, % area) | PASS 79 | PASS 94 | PASS 86 | PASS 71 (was FAIL 55) |
| entropism (shapes, % area) | PASS 83 | PASS 92 | PASS 88 (was FAIL 60) | PASS 80 |
| kitsch (inks, IoU) | PASS 0.54 (was 0.47) | PASS 0.59 (was 0.54) | PASS 0.62 (was 0.46) | PASS 0.57 (was 0.55) |
| neokitsch (inks, IoU) | PASS 0.73 | PASS 0.60 (was 0.54) | PASS 0.68 (was FAIL 0.61) | PASS 0.60 (was 0.63) |

Two `spec_diff.py` rules were recalibrated on this set, both documented
in the script: a rect↔chamfer pairing now counts as a match (a photo's
glow fits a rounded outline as a chamfer where a clean render fits a
rect — the corner detail separates photo from render, not right from
wrong; other class changes remain reclasses), and a class absent from
the candidate only gates when it holds ≥10% of source shape area (the
extractor fits faces in portraits, logotype glyphs and badge glow as
small diamonds and chamfers a trace rightly does not draw). Before the
recalibration only the four hubs passed; the six shapes-mode traces
scored 24–83% while overlaying exactly.

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