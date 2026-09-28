# Entropism derived references, 2026-09-27

`components.svg` is a derived specimen sheet, not a fifth photographic source. Its first two bands quote the four `*-trace.svg` files; band C documents a later interaction reading. The store and hub photo filenames are swapped, as `../sources.md` records. `bar.svg` has no Behance source. It borrows the header-strip and segmented-cell grammar but is an original status-bar design.

## Parent excerpt comparison

The comparison parsed SVG XML and compared each complete definition by tag, sorted attributes, text and ordered children. It excludes indentation and comments. These source and component definitions match exactly:

| Parent | Definition | Elements, parent / sheet | Result |
| --- | --- | ---: | --- |
| `dashboard-trace.svg` | `caption` | 5 / 5 | Equal |
| `mailbox-trace.svg` | `env`, `env-open` | 3 / 3 each | Equal |
| `store-trace.svg` | `rifle`, `rifle-detail`, `rifle-selected`, `rifle-selected-core`, `qr`, `card` | 1 / 1 each, 26 / 26, 34 / 34 | Equal |

The login trace has one ten-star mask (`**********`) and no eleven-star mask. The sheet uses the same ten-star mask in its main specimen and field states; no eleven-star mask remains. The main field is 359 × 33 at 1.25px stroke, the caret is a 19px underline, and the parent and sheet both use Rajdhani Regular 25 at horizontal scale 0.94045. The sheet's main mail list shows four rows excerpted from the parent's seven-row list. It keeps the 62px row pitch and fitted 22.25px subject / 16.25px sender typography. Card 4's crop is explicitly documented as a local x135.6 cut; the complete card specimen remains card 2. The parent store trace defines that plain card at 265 × 237 plus a 265 × 49 socket row, which the identical `card` definition above preserves. The plain rifle has two nested source-native contours and the selected rifle has two directly traced dark contours, so the old 5-element envelope count is historical; see [store-art.md](store-art.md).

The sheet's isolated examples intentionally relocate excerpt geometry with translations. The interaction band is a separate interpretation: source stills show the reverse-video fill and open-envelope state, while pointer movement and held states are inferred. This pass retained those drawings and labels. The photo-residue swatch is diagnostic only; it is excluded from the design specimens.

## Strokes and color roles

The old universal 2px rule was incorrect. Login, hub and store traces draw primary 1.25px lines in their 1600px frames. The mailbox trace draws 2px `#709174` frames and dividers at its rescaled size. Its 7px `#25281d` and 4px `#0a0a02` passes carry `class="photo"`; the native-source audit finds no dark undershoot. The dark ring in the rescale is a Lanczos negative lobe. Faint bright overshoot is source residue, not another designed stroke. The bar's 2px width is its own design decision.

The accepted shared roles in `src/eras/entropism.rs` are `OUTLINE #8fba97` and `SAGE_SOLID #a6d3a7`; `home/themes/entropism/palettes.nix` also uses `#8fba97` for the `nexus` border. The old `#5d7752` / `#9cb795` values are history, not current source samples. The bar now uses the accepted role colors on its frame, dividers, selection cells and menu. Its XML has zero occurrences of either old value. Local trace ink still varies: the login field is `#75967b`, the store card frame `#93bd95`, and the mailbox's rescaled line `#709174`. Recoloring those excerpts to the shared role would erase their trace provenance.

## Current implementation versus historical notes

The sheet's old delta described the 2026-09-03 implementation. Since then, the Rust bar configuration uses `BarChrome::Frame`, zero cell gap, a 2px bar stroke, selected fill through `Ink::Select`, and the six-tile hub trace. `metrics.stroke` remains 1.0 for widgets that inherit it, while screen canvas primitives specify their own widths. The four screen grounds now share their measured models with the SVGs; local text-ink fitting remains separate. The bar SVG uses `MID #728f76` for disabled-menu text, matching current `BarMenu`. This disabled state remains an original inference because the source shows no disabled control. The status-bar drawing itself remains original, and its old implementation-delta block is historical.

## Render review

The measurements in this paragraph describe the earlier 2026-09-27 sheet/bar consistency pass. The E4 store artwork and printing update above was reviewed separately in [store-art.md](store-art.md); these pixel totals were not recalculated for E4.

Rendered both sheets with the pinned librsvg converter and Rajdhani font config. The 1920 × 1400 component render keeps all specimens, notes and interaction examples visible; the corrected delta sits within its original column. The 1600 × 220 bar render keeps a continuous one-frame strip, two selected cells, menu panels and the original geometry. Against the supplied `before` SVGs, direct RGB comparison changes 30,897 of 2,688,000 component pixels (1.15%; bounding box x281..1630, y11..670) and 22,397 of 352,000 bar pixels (6.36%; bounding box x6..1594, y3..146). The component changes are title and explanatory text in the provenance/rules area; bar changes are its expected outline and selected-cell colors. These renders verify the derived sheets' legibility and scope, not fidelity of the four parent traces to the photos.
