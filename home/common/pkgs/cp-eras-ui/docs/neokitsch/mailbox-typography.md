# Mailbox typography fit (NK-05 subset)

Source: `images/neokitsch-mail.png` (#71, 3840×2160). The measured Iced baseline is `/tmp/cp-eras-round6/after-neokitsch-mailbox-3840x2160.png`; the C and D captures are `/tmp/cp-eras-completion/c-mailbox-neokitsch-3840x2160.png` and `/tmp/cp-eras-completion/d-mailbox-neokitsch-3840x2160.png`. The trace and component specimens use the same 1600×900 design coordinates as `mailbox()` in `src/eras/neokitsch.rs`.

The previous list-row pass kept each title at x 193 (row 2 at x 193.5), baseline offset 27.2, 18px Rajdhani 500 and horizontal scale 0.99 on a 60.2 row pitch. Sender lines used 13px Rajdhani 500 at offset 48.2 and scale 1.16. The selected title/sender/envelope inks are the separately measured #7b5438/#895f3b/#865c39. The veneer, tab, rule and envelope coordinates did not change.

The plain message keeps the same title, `FROM: MOM`, and 2+5+3 paragraph lines. Its heading is now 17.5px Rajdhani 600, scale 1.01 at (736,276); the sender is 13px/500, scale 1.16 at (738,297); the body is 17px/500, scale 1.054 at x 735.2, first baseline 333 after the [AX horizontal fit](text-fit.md). Paragraph baselines, line pitch and gaps remain fixed. The heading size follows the source cap height; a 15px heading matched width after stretching but was visibly too short.

At native resolution, a consistent gold-pixel mask (R > 145, R > 1.08G, G > 1.12B) on fixed text crops gives the following bounds. Counts include the source's photographic bloom, so they guide density rather than define an exact target for sharp Iced text.

| Crop | Source width / ink pixels | Old Iced | C Iced |
| --- | ---: | ---: | ---: |
| Row 1 title | 285 / 2,587 | 298 / 3,807 | 290 / 2,292 |
| Row 1 `FROM: JACKIE` | 192 / 1,217 | 147 / 541 | 195 / 1,067 |
| Heading | 411 / 5,331 | 355 / 4,169 | 415 / 3,943 |
| Panel `FROM: MOM` | 161 / 1,018 | 123 / 606 | 162 / 1,124 |
| First body line | 1,253 / 10,200 | 1,198 / 11,852 | 1,260 / 9,765 |

The C capture preceded the final heading size change and measured 27 native pixels high against the source's 33. The D native capture verifies the 17.5px heading at x 1769..2184, y 632..664, 4,787 core pixels; source is x 1769..2180, y 633..665, 5,331 pixels. Its height now matches and its width differs by 4 native pixels. The remaining density gap is about 10%, in a crop where the photo adds bloom. Row 3, long senders and the selected row were inspected at native size as held-out examples; the type fit keeps their text within the photographed list area. The source font is unknown. Rajdhani's narrow counters and its lack of photographic glow remain visible, especially in the selected row and long body lines; neither a width match nor an ink count proves an exact typeface match.

## 2026-09-29 list-row refinement

The source, fresh native SVG and `/tmp/cp-eras-completion/f-mailbox-neokitsch-3840x2160.png` were compared in separate row-title and sender crops. Ordinary gold masks use R>145, R>1.08G and G>1.12B; the selected brown glyphs use R<155, G<115 and B<95 within their text boxes. The source has bloom and veneer grain, so these masks measure primary extent and alignment rather than exact material density. Stricter ordinary thresholds at R>175 and R>205 preserve the width finding.

All seven titles keep Rajdhani Medium 18 and their content, but horizontal scale is 0.975. Source/SVG title right-edge errors of 3–6 native pixels become 0–1 pixel after that change. Text-only SVG baselines lift separately per row; no row frame, pitch or line break moves. Ordinary sender runs start at x193.4167: JACKIE and RACHEL ROSS use scale1.15, while 805000451 and JINX JINX STORE use scale1.13. Their old right-edge errors of 2–8 native pixels become 0–1 pixel. The selected `FROM: MOM` SVG keeps its prior x, baseline, scale and ink because tested SVG geometry changes reduced its dark-mask overlap. Its independent Iced calibration is recorded below.

| Row | SVG title baseline | SVG sender baseline | Rust title offset | Rust sender offset |
| --- | ---: | ---: | ---: | ---: |
| 1 | 275.5833 | 297.0 | 27.825 | 49.0333 |
| 2, selected | 334.95 | 357.2 unchanged | 27.4083 | 49.0333 |
| 3 | 395.15 | 416.5667 | 27.2 | 48.4083 |
| 4 | 455.7667 | 477.1833 | 27.2 | 48.2 |
| 5 | 515.1333 | 536.9667 | 26.7833 | 48.2 |
| 6 | 575.3333 | 596.75 | 26.575 | 47.575 |
| 7 | 635.1167 | 656.5333 | 26.1583 | 47.3667 |

Rust offsets were calibrated from the F native glyph centers, independently of the SVG lifts: Iced and librsvg differ by about two native pixels in several rows, and the source's brighter fringes leave roughly one pixel of fractional uncertainty. On the same source/SVG masks, title IoU rises from 0.389 to 0.754 on row 1, 0.122 to 0.620 on the selected row, and from 0.241/0.196/0.171 to 0.702/0.673/0.731 on held-out rows 3/5/7. Ordinary sender IoU on held-out rows 3/5/7 rises from 0.251/0.189/0.168 to 0.335/0.299/0.378. The row-2 title alone retains x193.5; a transient selection on another row uses that row's ordinary text geometry. Native R review confirms all seven title bounds within one source pixel. Ordinary sender endpoints are within one pixel; their typeface and bloom remain approximate. The selected sender alone was still two native pixels high, so S moves its runtime offset from 48.2 to 49.0333 without moving its SVG. Its native vertical bounds now match source y838–856, and dark-mask overlap improves .157→.227. First, middle and last selection transfers and held states are reviewed at fractional window dimensions. The integrated T check passes all 22 repository checks and all 27 visual cases exactly.


## Native RIFLES action-label calibration — AB

The SVG's 16px Medium labels with tracking were still rendered as 15px
Regular in Iced. All four source words span 109 native pixels at the common
gold threshold, while the old native runs span 92–93; source cap bounds
are y1678–1705 versus native y1678–1700. This is separate from the already
fitted list rows and message body.

Two isolated native trials fit Medium 16.75, scale1.050746 and baseline27.5
relative to the action frame. Independent local x origins are21.0,23.4667,
21.9833,20.0833; the existing frames and uniform192-unit pitch do not move.
The final native cap bounds are y1679–1704, with first/second word endpoints
matching the central source mask exactly and third/fourth right endpoints
one pixel short. Ink remains semantic Fg. These action drawings have no
pointer behavior; row selection and custom palettes are the relevant
existing state controls.

At native R>175, R>1.08G, G>1.12B, source/native F1 rises .217→.826,
.215→.756, .170→.796 and .263→.763. Every first/middle/last segment improves
on all four labels at red thresholds145/175/205. Fixed-crop RGB error falls
21.93→12.25,22.80→13.47,23.47→12.99,22.39→13.86. All6366 changed4K pixels
are confined to the labels. Source bloom and exact glyph contours remain
separate; higher photographic ink area is not an exact weight target.
The SVG is unchanged. Scratch trials and measurements are under
`/tmp/cp-eras-resume-20260929/ab-nk5-audit/`; the production4K frame matches the accepted trial byte-for-byte.
Fractional rest/held-row, custom foreground and opening captures are
reviewed. Both gates and the full AB check pass; see the integrated acceptance below.

### AB integrated acceptance

The source/native/state review is integrated: all 286 Rust tests and 22
repository checks pass, including 27 exact visual cases on their first
attempt. All 199 frozen file hashes match the Nix source. This closes the
bounded AB correction above; its stated photographic/glyph limits remain.
