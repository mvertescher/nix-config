# Reference campaign batch twenty-six — AO

AO addresses the extra title-rendering cost and intermittent unit tests
found during AN. No source SVG, artwork coordinates or golden changes
are intended in this batch.

## Repeated title outlines

Only the ordinary Neomil store-title echoes opt into `ReusableWide`.
During a single `Scene.draw`, a four-entry FIFO cache retains Iced's own
origin-relative glyph paths, keyed by content, complete font and exact
size bits. Position, nonuniform stretch, clip and current semantic ink
are applied for each impression. Each glyph retains its separate path
and the original nonzero fill rule. Primary `Wide` text is unchanged.

The cache accepts short, finite, bounded, start-anchored bold runs with
nonuniform stretch. Uniform text and unusual inputs keep `fill_text`.
An extraction color that cannot survive RGBA8 quantization distinguishes
outline callbacks from Iced's raster fallback; any raster callback
rejects reuse for that run. Failed extraction is remembered only within
the current draw. No persistent cache assumes fonts remain registered
across application lifetimes. Unit controls compare fresh and cached
glyph events at two sizes and verify the sentinel's quantization property.

A paired release benchmark uses three warmups and twenty timed 4K
`Scene.draw` calls per variant, dropping each geometry after timing.
Median CPU time falls 25.674→22.823 ms (2.851 ms, 11.1%); means are
25.693/22.863 ms. Both benchmark frames exactly match the reviewed AN
4K output. This recovers part of the added impression cost; it excludes
backdrop preparation, GPU work, presentation and continuous frame rate.

## Test font initialization

AN's first Nix package attempt exposed two changing measurements within
login assertions. One unrelated test registered Rajdhani Regular into
Iced's global font system while other tests measured carets and display
capacity. Individual paragraph locks did not make an entire assertion
atomic. The identical package retry passing did not repair the race.

Under tests, `run_width` now waits for one shared initialization of all
ten bundled application faces. Registration holds the font-system write
lock across the complete set. The tall-card-note test no longer mutates
fonts independently; all existing assertions remain unchanged. Production
font loading and input behavior are unchanged.

A fresh-process diagnostic with empty fontconfig reproduces the mechanism:
the same ten-mask width changes 247.370→110.750 when the font is loaded,
moving derived caret x635.795→499.175. Its initial fallback differs from
the Nix failure's x530.823, so these are mechanism controls, not identical
environment reproduction. The corrected 245 library tests then pass
with eight test threads in the same empty-font configuration. The full
local suite passes 290 tests, including the two outline regressions.

## Verification

The 4K benchmark images and all sixteen synthetic drawing-state captures
are pixel-identical to AN. These cover 1600 and fractional sizes, every
selected slot, ordinary/selected hover and held states, custom foreground
and background roles, and early opening. The tests exercise drawings;
they do not establish live desktop input behavior. Unchanged pixels and
SVGs retain AN's source-fidelity evidence and its stated limits.

All 22 repository checks and all 27 visual cases pass on their first
attempt. The Nix package passes all 290 tests without a retry. Direct
comparison finds 26 pixel-identical cases; the unchanged Neo-kitsch bar
retains its pre-existing one-level pixel at (749,20). No golden is changed.
All 292 frozen hashes match the Nix source, four original images are
unchanged, and a fresh packaged Neomil 4K capture exactly matches the
reviewed optimized preview. Final prose leaves all tested implementation,
SVG, vendor, script, test and golden content intact.

The Kitsch lower-print study remains unapplied. A selected-only vertical
shift improves a 22-pixel SVG crop, but its premise fails source-only
background controls: normalized selected/ordinary-card-3 ink centroids
are 730.450/730.459. A distinct selected contour is not established.
See [the source-control record](kitsch/store-art.md#ao-lower-print-phase-audit).

Evidence is under `/tmp/cp-eras-next/ao/`: `benchmark.json`,
`font-repro.log`, `empty-font-tests.log` and `rust-tests.log`.
Changes remain staged/uncommitted; no push or deployment is performed.
