# Reference SVG round sixty-four — cached ordinary rifle material

EM follows the controlled blending result from [round sixty-three](reference-svg-round63.md).
The ordinary Entropism rifle uses existing encoded-RGB `Prim::Soft`
composition instead of changing shared Iced color blending or font weights.
Its source colors are body `#779375`, bright `#b3ddb1`, EH seam `#a5cca4`
and EI band `#8bac89`. Body/detail paths and both accepted 78-vertex
stock contours retain their SVG coordinates. Selected inverse art and
all text keep their existing native rendering.

The first harness exposed a routing assumption: the published `nexus`
palette has panel `#181109`, whereas the standalone palette uses `#110c07`.
The failed harness is retained. EM2 recognizes those two complete palettes;
it does not rewrite palette roles. Its fresh library copies all 71 current
Rust sources and 15 font files, changing only the scratch store eligibility
predicate. All three Entropism baselines and the Kitsch baseline match
the packaged frames exactly. Custom-palette and Kitsch negative pairs
are also exact. EM3 reuses that verified library.

## Source fit and fourth-card placement

Cached composition improves both complete ordinary rifles at 3840×2160,
1600×900 and 1537×947. Small rail losses remain, substantially reduced
from EJ's native path recoloring. Accept the coherent full-rifle gains
with those local limits explicit; the source photo is not a perfect SVG.
The initial cached fourth copy still loses, prompting an independent
placement audit before accepting the candidate.

Source card-2/card-4 comparisons distinguish the rifle from its card:
frame, heading, stats and socket favor the existing card pitch; only
the rifle favors a two-design-pixel-left origin. Moving the entire card
is rejected. EN therefore keeps card origin x1429 and moves only its
ordinary rifle to x1427. The permanent crop still ends at x1564.6.
The SVG adds a fourth-card definition referencing the same rifle paths
inside that local translation. Existing definitions are unchanged; the
component's displayed ordinary specimen remains pixel-identical.

Fixed fourth-rifle RGB L1 against the uniformly projected original:

| Size | SVG before → corrected | Native before → cached/corrected |
| --- | ---: | ---: |
| 3840×2160 | 2,969,481 → 1,639,773 | 2,965,613 → 1,639,741 |
| 1600×900 | 461,376 → 247,794 | 457,085 → 256,479 |
| 1537×947 | 426,833 → 230,527 | 421,453 → 232,377 |

EN left-entry, rail and
body regions improve at all three sizes. Changed SVG pixels remain
inside the fourth rifle; native changes remain inside the three ordinary
rifles. Frames, text, selected art, alpha and crop exteriors are exact.

## Reuse and state behavior

EM3 has four static rifle layers in a fixed order after the shared ground.
Selection hides a layer with a zero-width clip, preserving its cache key
and ordered prefix. The original opening clip encloses all four layers.
The separate rifle layers have disjoint antialias coverage at all three sizes.
This keeps a card/category change from creating another material image.
No shared cache policy, renderer setting or font changes are needed.

Splitting EM2's combined layers changes twelve 4K pixels in cards 2/3
by at most one channel level. At both smaller sizes, those rifles remain
exactly EM2's pixels. This small compositing difference is included in
the final source measurements, not hidden behind rounded similarity.

Actual Store captures cover all four selected cards at 1600. Selected
drawings and every pixel outside ordinary rifles stay unchanged. The
zero-time frame is exact; a fractional 200 ms opening pair keeps changes
inside the moving clip. Seven paired hover/held/away captures also pass: selected and hovered
inverse art remain exact, including the fourth-card cut at 4K and
fractional scale. Ordinary pressed art exposes the same cached rifle
without drawing a semantic copy over it.

The production route uses compile-time scene arrays and fixed static
rifle slices. Named Entropism `nexus` and canonical `reference` variants
accept only the exact built-in or published palette; direct built-in and
forced-era styles also qualify. Other variants, unknown names and edited
roles use the semantic scene. Tests cover every published role, added
ornaments, direct edits/restoration, all twenty selections, hit/navigation
parity and cache image identity. All 305 local and packaged Rust tests pass.
The full check passes 22 repository checks and all 28 matrix cases on their
first attempt. Direct comparison finds 26 of 27 visual cases RGBA-exact;
the existing one-pixel, one-level Neo-kitsch bar residual is unchanged.
The remaining case verifies the source sampler. All 346 frozen tracked
files match the tested Nix source. A fresh packaged 4K Store capture is
byte-exact to the reviewed candidate (`defd89c2…`), as are the production
captures at all three sizes. Only verification prose changes afterward.

## Cache cost and measured drawing time

Actual cache instrumentation verifies five retained images (ground plus
four rifles), exactly five initial raster requests, then zero new requests
across all twenty selection pairs. Keys and image handles remain identical.
Retained RGBA payload is 45,711,360 / 7,987,200 / 7,383,748 bytes at the
three sizes: about 43.6 MiB total at 4K, of which roughly 12 MiB is added
rifle data. GPU copies and temporary compositor buffers are separate.
Observed first 4K cache preparation is 106.7–108.4 ms; this is a cold
cost, not a whole-window startup measurement. All fifteen instrumented
captures remain byte-exact to their reviewed baseline or candidate.

The initial separate-process timing run is rejected for insufficient
warm samples and large external CPU-load drift. Its logs are preserved.
EM4 instead draws both prototype variants through the same actual renderer: eight
warm-up pairs followed by thirty ABBA cycles, timing `Backdrop::draw`
and foreground `Scene::draw` through geometry creation. Instrumented
screenshots are exact at all three sizes.

| Viewport | Baseline foreground median | Cached foreground median | Median paired difference |
| --- | ---: | ---: | ---: |
| 3840×2160 | 16.644 ms | 13.243 ms | −3.404 ms |
| 1600×900 | 17.401 ms | 13.841 ms | −3.539 ms |
| 1537×947 | 31.226 ms | 22.081 ms | −7.461 ms |

At 4K this is a 20.4% reduction in median foreground CPU draw, with
all thirty cycles favorable and little within-run drift. Backdrop CPU
work adds a median paired 2 µs at 4K and 1600. Fractional foreground
pairs all improve, but substantial ambient-load drift prevents using
its magnitude as a stable speedup estimate. These timings exclude GPU
upload, presentation, frame scheduling and startup. Fonts, text layout,
shared renderer color behavior and cache policy are unchanged.

EM5 separately verifies the final production implementation at 4K. A
fresh copy of all 72 Rust sources and 15 font-directory files changes only
the diagnostic timing hook; the app uses the actual theme loader and
production Store material. Pinned compiler/dependency hashes and the copied
sources are checked before and after compilation. Its screenshot is
byte-exact to the accepted candidate. Eight warm pairs and thirty ABBA
cycles yield foreground medians **16.541→13.146 ms**, a **20.5%** reduction;
the median paired difference is −3.404 ms and all thirty pairs improve.
Baseline first/last-ten medians are 16.538/16.499 ms. Backdrop medians
increase 2.5→4.5 µs, with a paired difference of +2 µs. Astra independently
recomputes all thirty cycles and verifies the instrumentation and artifacts.
This confirms the production CPU gain without inferring GPU, frame-time
or startup performance from it.

Evidence: `/tmp/cp-eras-next/em2-entropism-soft/`,
`/tmp/cp-eras-next/em3-entropism-cache/`,
`/tmp/cp-eras-next/en-entropism-fourth/` and
`/tmp/cp-eras-next/em3-cache-perf/`, `/tmp/cp-eras-next/em4-paired-perf/`
and `/tmp/cp-eras-next/em5-production-perf/`.
Original images remain unchanged.
E4's broader fine printing and all other broad scopes remain open.
