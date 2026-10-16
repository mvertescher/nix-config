# Reference SVG round seventy-four — ordinary Neomil Store frame echoes

The ordinary cards were missing two faint frame impressions visible outside
their primary edges. Fixed affine copies now supply that printing, with
separate world-positioned geometry for the inferred ordinary second card.
The lower-right primary stems and adjoining bars are adjusted together so
the combined impression follows the source without a fan-shaped gap.

## Source and rendering

The [frame-echo study](neomil/store-frame-echo.md) records the original
cross-sections, independent ground controls, rejected trials and accepted
bounded reconstruction. The new model uses white nested coverage followed
by one color application per impression. This avoids the weak output seen
when many nearly transparent colored strokes were individually quantized
over opaque artwork. The cropped fourth card has separate soft terminations;
its existing primary drawing and hit cut remain x1558.3.

All 150 fixed source-region comparisons improve or remain exact in each
renderer against its own production baseline. At 4K the native whole-card
RGB L1 decreases by 794,022 / 743,228 / 246,247 for cards one, three and four.
All three whole cards also improve at the smaller viewports. Four narrow
native junction rows retain a worse metric: first-card 1600 y405/y409 RMS
+0.1052/+0.0120 with L1 −12/−11; fractional y405 L1 +4 with RMS −0.4932,
and y412 RMS +0.0965 with L1 −4. These losses remain explicit within improved
enclosing regions. The result is not a unique recovered source paint recipe.

SVG cleanup preserves the accepted candidate byte-for-byte at 3840×2160,
1600×900 and 1537×947. Unused experimental filter definitions are removed;
the component's ordinary-card example shares the exact masks and primary/bar
geometry. Its two captions are synchronized and kept within their panel.
Original images, fonts, text metrics and accepted glyph outlines are intact.

## Cost and interaction

The literal prototype costs substantially more to prepare than existing
artwork. A shared software-stroke optimization unions conservative active
segment column intervals, retaining the exact distance, antialias and float
blend calculations. Overlaps paint once; large or unusual inputs preserve
the legacy traversal. Reference tests compare both float and RGBA buffers,
and native resting/state captures match the literal prototype byte-for-byte.

In twelve paired fresh-process runs, 4K first preparation falls from 804 to
635 ms, 1600 from 192 to 152 ms, and fractional from 168 to 149 ms. The
subsequent twenty card/category choices fall from 4259/1016/950 to
3304/797/754 ms. These sums exclude the initial preparation. All variants
retain ten cache entries and identical payload bytes/band counts; warm cache
calls remain submicrosecond. The earlier production 4K result was about
400 ms in a separate session, so the added artwork still has a cold cost.
These measurements cover CPU preparation, not GPU or total startup latency.

Ten reference states and three custom states, plus a disabled-custom-hook
control, preserve literal/optimized image parity. Selected artwork, content
interiors and alpha remain exact. Changes stay inside the fixed echo/primary
support, including the opening wipe, hover/held and inferred ordinary C2.
Custom palettes use the generic material route and only inherit the coupled
primary/bar geometry. A first scoring control omitted C2's translated support;
the preserved result has 383 flagged pixels, and the corrected control adds
exactly the first-card box translated +332, without changing any image.

Warm CPU drawing is also checked with the real Backdrop and Scene methods:
four fresh ABBA processes, five warmups and twenty timed pairs per state/size,
960 records total. At 4K, Scene medians span 19.02–20.07 ms for the previous
artwork and 18.83–19.89 ms for the new version; Backdrop remains about
6.6–6.7 microseconds. Fractional Scene medians are 20.18–21.28 versus
20.09–21.11 ms. These small differences show similar warm cost. A 1600 resting
baseline drift of 20.82→24.54 ms makes its apparent aggregate gain inconclusive.
This excludes geometry disposal, GPU submission and presentation. Probe-window
screenshots are blank and are not used as artwork evidence.

## Verification

The production package passes 307 Rust tests, including the two differential
stroke tests. All 22 repository checks and 28 matrix cases pass on their first
attempt. Of the 27 visual outputs, 26 are pixel-exact and the existing
Neo-kitsch bar difference remains one pixel by one channel level; source
sampling is the twenty-eighth case. G1i and packaged G2i Store gates pass.
Fresh actual-package captures at all three sizes match the accepted native
prototype byte-for-byte. The tested Nix source is checked against every frozen
tracked crate file; final documentation is the only subsequent change.

Integration initially stopped at the whitespace check after staging all seven
reviewed files. Eight whitespace-only SVG lines left by removing unused filters
were trimmed, with an identical parsed SVG tree; the full check then ran.
All earlier staged work is preserved, and nothing is committed or deployed.

The broader primary-bottom and fine-printing TODOs remain open. Scratch
evidence is under `/tmp/cp-eras-next/ie-echo-masked-contours/`,
`if-echo-native/`, `ig-echo-cache-cost/`, `ih-bounded-stroke/v2/`,
`ii-echo-states/`, `ij-echo-warm-draw/`, `ht-echo-review/` and
`ik-echo-integration/`.
