# NK-07: the veneer as a generated texture (2026-11-13)

Owner decision 3 of [the 2026-11-13 review](../review-2026-11-13/DECISIONS.md):
the selected Neo-kitsch surfaces wear a **pre-rendered texture clipped to
their shapes**, not drawn vector grain. Accepted costs: an image asset, and
less clean scaling at odd sizes. [veneer-fit.md](veneer-fit.md) is the
history of the vector fits that did not converge.

The reference art is third-party and never redistributed, so the texture is
**generated**: `scripts/make-veneer.py` computes every texel from the
parameters below, reads no image, and writes `veneer/*.png`. The references
were used only to measure (numbers below) and to compare by eye.

## Surfaces

| Texture | Surface | Design box | Texels |
| --- | --- | --- | --- |
| `dashboard-email` | selected cascade card (`#ncardsel`), card-local, so a held card wears it too | 90.5 x 325.2 | 218 x 781 |
| `dashboard-panel` | detail panel body | 230.4 x 309 | 553 x 742 |
| `mailbox-row` | selected row bar (`#selclip`), row-local, follows the selection | 477 x 55 | 1145 x 132 |
| `store-nav` | selected category (SMG), button-local, used by every category | 200.6 x 38.6 | 482 x 93 |
| `store-card` | selected card body + socket band, card-local; the inferred held compact band reads the same texture at the same card-local position | 262.1 x 241.7 | 630 x 581 |
| `store-basket` | BASKET plate's lower band | 204.6 x 45 | 492 x 108 |

Not veneered: the login's ENTER / LOGIN bars. Measured, they are a flat
fill with fine noise only (green std 5.3 levels in clear windows, no line
structure; `#ffc470` mean), so a figured texture would contradict the
reference. The BASKET's upper half is likewise flat (std ~1). The widget
mail app (`panels/mail.rs`, not a reference screen) keeps its synthesised
`Fill::Veneer`.

## Measurements (references at 3840x2160, reported in 1600x900 design px)

Per surface: base = median of pixels above the 60th green percentile; line
core = mean line profile at its minimum; pitch = local-minimum spacing in
text-free windows (FFT where clean).

| Surface | Base | Fill median | Line core | Pitch | Between-line std (G) |
| --- | --- | --- | --- | --- | --- |
| dashboard EMAIL card | `#ffbf6c` | `#ebab5b` | `#d19348` | 1.9-2.7 | 6.3 |
| dashboard panel | `#ffc26e` | `#f6b664` | `#e2a354` | 2.12-2.18 (calm), 3.4-3.8 under the figure | 3.9 |
| mailbox row | `#ffcb84` | `#efb873` | `#d99f63` | 2.24 (1.4-3.7, bunched) | 6.0-7.3 |
| store SMG | `#fac883` | `#ecb670` | `#d49955` | 2.0 | 8.6 |
| store card | `#ffce84` | `#f6bc71` | `#e1a253` | 2.2-2.4 | 4.4 |
| store BASKET band | `#fab56b` | `#e89e5f` | `#d78854` | 2.0 | 8.0 |

Common to all: a dense field of thin dark lines (core about 1 native px,
30-35 green levels below the base), pitch about 2.2 design px (5.3 native),
fine pore noise, and figure made of seams where the lines bend, converge and
cross. That is what one generator with per-surface parameters serves.

Per surface:

- **Seams.** EMAIL: horizontal book-match about halfway down (local y ~160,
  design y ~545), the leaves converging to the right edge and crossing in a
  ~25 px band; a second crossing swirl at the top-left. Mailbox: vertical
  open-book seam at design x ~274 (local 239), lines fanning away from the
  bar's middle height into it. SMG: vertical seam at design x ~168. Store
  card: horizontal streak at design y ~531 (local 120), converging to the
  right edge, dark core ~4 design px tall at the right. BASKET: vertical
  seam at design x ~1329.
- **Figure.** Panel: a fold in the lower third, the ridge from local
  (184, 259) to (84, 299), lines leaning down-left above it. Mailbox: the
  left end's lines rise to the top edge.
- **Edge shading.** Weak: 1-9% L* over 5-10 design px (EMAIL darkens toward
  its right edge 0.768 -> 0.698; the mail row lightens to the left,
  0.826 -> 0.78). The panel shows no measurable top/bottom darkening.
- **Rim.** Edges are hard (1-2 native px) with a faint light line inside:
  +10..15 green levels for 1-2 native px on the mail bar's top and bottom
  and the card's right edge; none on the panel. Drawn as a 0.5 design px
  `#ffe2a6` at 0.55 inside the edge; EMAIL keeps its traced 1.2 `#f2b463`
  outline instead.
- **Ink on the fill** (text box, ink = darkest 3%, fill = median of the
  brightest 40%): panel copy CR 7.32 on fill Y 0.605; mail title 5.64 / Y
  0.603; card "Recoil" 6.67 / Y 0.670; SMG 6.25 / Y 0.622; BASKET fine print
  3.94 / Y 0.511.

## The generator

`scripts/make-veneer.py`, numpy only. A texture is the iso-lines of scalar
fields `phi(s, t) = s + D(s, t)` (`s` across the grain, `t` along), one line
per `pitch`. A surface holds one or more line *families*, each with a window
along `t`; a family can be the mirror of the field about `t = m` (the
book-match), and where windows overlap both are drawn and cross. `D` sums an
anisotropic fBm wander (evaluated at the reflected position for a mirrored
leaf, so the leaves meet instead of doubling), `bend` terms (exponential, or
a tent that turns lines at a knee, its amount varying across the grain),
`fold` (the panel figure) and `fan` (the open book). Lines are gaussian
cores from |phi - nearest| / |grad phi| on a 3x3 supersample, so converging
lines merge into a darker band; then pore noise stretched along the grain, a
faint blotch, a mild edge shade, and a three-stop sRGB ramp (light, base,
dark) stored as the PNG's palette. Each surface has a fixed seed (PCG64).

Output: 8-bit indexed PNG, 2.4 texels per design px: 3840x2160 samples it
1:1, and nothing above 4K is a target. Total 942 KB on disk (1.21 M texels).

Reproducible: hand-written PNG encoder (fixed filter, zlib 9), float64,
fixed seeds. Two runs are byte-identical; `default.nix`'s
`passthru.tests.veneer` re-runs the generator in the sandbox and `cmp`s each
file against the committed one. Regenerate with

    nix-shell shell.nix --run 'python3 scripts/make-veneer.py docs/neokitsch/veneer'

## Drawing

`Prim::Veneer { x, y, texture, shape }` (`src/style.rs`) and the mailbox's
`Veneer::texture`. `src/veneer.rs` decodes the PNG (indices + palette, plus
2x2 box mips so 1600x900 does not moire), and rasterises the surface's own
fill prims with `soft.rs`'s coverage, sampling the texture per pixel (bilinear
at 4K, a mip plus a small box below 1:1), with the rim. The result is a
premultiplied image of just the surface's pixel box, cached per surface,
position and scale (64 MiB / 64 entries).

A canvas draws images above every mesh in its layer, so the scene is painted
in layers (`Pass` in `screens/scene.rs`, `widgets/layered.rs`): veneer `i` is
the last thing in layer `i`; a fill touching an earlier veneer's pixel box
(or an earlier raised fill) goes above it; everything else stays in layer 0
so its antialiasing is unchanged. The mailbox draws its veneer in a layer
under the sheet. A scene without a veneer is one layer, exactly as before.

## Containment, legibility, cost (2026-11-13)

- Golden matrix, rendered headlessly: 22 of 25 cases are pixel-identical.
  The three Neo-kitsch cases differ only inside the veneered boxes (plus a
  1 px rim), except one pixel: (1455, 19) in the store, the top of the BASKET's
  vertical hairline, which joins the band's hairline and so moves to the
  band's layer; its antialiased top pixel against the plate edge resolves
  separately (delta 78 on one pixel).
- Fill under text, 1600x900, Y (encoded lightness `srgb(Y)`): panel 0.556
  (0.78), mail 0.638 (0.82), card 0.597 (0.80), SMG 0.593 (0.80), BASKET
  0.484 (0.73). The ink correction assumes 0.78. Contrast moved toward the
  reference everywhere (mail title 3.80 -> 4.35 vs 5.64, card 620 7.26 ->
  8.90 vs 7.90), except the panel copy, 6.84 -> 6.52 vs 7.32.
- Binary: +0.98 MB per binary (dashboard 30.61 -> 31.60 MB release).
- Cold, release, CPU: decode + mips of all six 11.5 ms; per image at
  1600x900 1-11 ms, at 3840x2160 2-29 ms (panel the largest, 553x742 px,
  1.6 MiB RGBA). Retained: at 4K about 3.4 MiB of images per screen plus
  ~1.6 MiB of decoded indices and mips; GPU atlas the same image sizes.
  No full-screen image is added.
- Per frame: one extra walk of the display list per veneer on screen (store
  3, dashboard 2), each drawing only its own leaves. Not timed.
- Real-GPU cost is unmeasured; all numbers are software Vulkan / CPU.

## Open

- The look is the owner's call. Known differences: the store streak's dark
  core merges into a soft band where the reference keeps resolvable lines;
  the EMAIL top swirl and panel fold are simpler than the reference's; the
  mail bar keeps its traced chamfered end, not the reference's tapered one.
- Three store-card variants (calmer / fit / stronger) were rendered for the
  owner; the generator takes any of them as a recipe change.
