# Mailbox typography fit (NK-05 subset)

Source: `images/neokitsch-mail.png` (#71, 3840×2160). The measured Iced baseline is `/tmp/cp-eras-round6/after-neokitsch-mailbox-3840x2160.png`; the C and D captures are `/tmp/cp-eras-completion/c-mailbox-neokitsch-3840x2160.png` and `/tmp/cp-eras-completion/d-mailbox-neokitsch-3840x2160.png`. The trace and component specimens use the same 1600×900 design coordinates as `mailbox()` in `src/eras/neokitsch.rs`.

The seven row titles keep their words, x 193 (selected x 193.5), baseline offset 27.2, 18px size and 60.2 row pitch. Their fitted weight is Rajdhani 500 and horizontal scale 0.99. All sender lines keep the original uppercase content and offset 48.2; they use 13px Rajdhani 500 and horizontal scale 1.16. The selected title/sender/envelope inks remain the separately measured #7b5438/#895f3b/#865c39. The veneer, tab, rule and envelope coordinates did not change.

The plain message keeps the same title, `FROM: MOM`, and 2+5+3 paragraph lines. Its heading is now 17.5px Rajdhani 600, scale 1.01 at (736,276); the sender is 13px/500, scale 1.16 at (738,297); the body is 17px/500, scale 1.06 at x 736, first baseline 333. Paragraph baselines, line pitch and gaps remain fixed. The heading size follows the source cap height; a 15px heading matched width after stretching but was visibly too short.

At native resolution, a consistent gold-pixel mask (R > 145, R > 1.08G, G > 1.12B) on fixed text crops gives the following bounds. Counts include the source's photographic bloom, so they guide density rather than define an exact target for sharp Iced text.

| Crop | Source width / ink pixels | Old Iced | C Iced |
| --- | ---: | ---: | ---: |
| Row 1 title | 285 / 2,587 | 298 / 3,807 | 290 / 2,292 |
| Row 1 `FROM: JACKIE` | 192 / 1,217 | 147 / 541 | 195 / 1,067 |
| Heading | 411 / 5,331 | 355 / 4,169 | 415 / 3,943 |
| Panel `FROM: MOM` | 161 / 1,018 | 123 / 606 | 162 / 1,124 |
| First body line | 1,253 / 10,200 | 1,198 / 11,852 | 1,260 / 9,765 |

The C capture preceded the final heading size change and measured 27 native pixels high against the source's 33. The D native capture verifies the 17.5px heading at x 1769..2184, y 632..664, 4,787 core pixels; source is x 1769..2180, y 633..665, 5,331 pixels. Its height now matches and its width differs by 4 native pixels. The remaining density gap is about 10%, in a crop where the photo adds bloom. Row 3, long senders and the selected row were inspected at native size as held-out examples; the type fit keeps their text within the photographed list area. The source font is unknown. Rajdhani's narrow counters and its lack of photographic glow remain visible, especially in the selected row and long body lines; neither a width match nor an ink count proves an exact typeface match.
