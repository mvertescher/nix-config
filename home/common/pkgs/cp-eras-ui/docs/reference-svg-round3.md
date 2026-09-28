# Reference corrections, third batch — 2026-09-27

Three scoped Sol workers implemented these corrections; Astra integrated
the shared support and reviewed the source, SVG, Iced output and runtime
states. This follows the [second batch](reference-svg-round2.md), whose
previously staged changes landed in `79c1a02`.

## Neo-kitsch mailbox envelopes — NK-13

The original TODO incorrectly attributed the envelope displacement to row
pitch. Native source measurements support the existing 60.2px pitch for
text and rules, and the existing selected-row bounds. The envelope glyphs
alone need extra vertical offsets of 0/0/1.5/1.5/2.3/3.2/4.4 design pixels.
Both traced outlines and runtime glyph placement now use those offsets.

Direct native SVG registration reduces row7's vertical shift from 4.58px
to zero at the source's 0.417px sampling interval. Other residuals are at
most one source pixel. The native Iced before/after comparison changes
4,189 pixels, all within the envelope column; text, rules and material
remain pixel-identical. Selected and unselected hover/held previews retain
the appropriate open/closed outline and printing ink. Opening still applies
the list's reveal and opacity. See [measurements](neokitsch/mailbox-row-spacing.md).

## Kitsch store fourth-card cutoff — K7

The primary drawing remains opaque through x1523, followed by five narrow
residue bands with decreasing opacity to x1550. SVG and Iced apply this to
the foreground only. A persistent viewport also prevents hits beyond the
visible right edge. The reusable component specimen stays complete; its
documentation identifies the screen's crop.

Native review caught an initial regression: a clip starting at the card's
left boundary removed its projecting flag and half of the outline stroke.
The final bounds preserve both, including the lifted hover ghost. The final
native Iced comparison changes **zero pixels left of x1523**. At 1537×947,
selecting card4 and applying hover/held feedback changes **zero pixels past
the fade endpoint**. The selected lower detail, footer and opening clip are
retained. States beyond the source's selected card2 remain inferred.

The pointer probes also confirmed a pre-existing limitation: selected
Kitsch cards retain their normal 320px hit height even though their drawing
extends lower. The visible point `(1485,650)` does not hit selected card4.
This is now an explicit [runtime follow-up](../todo/toolkit.md#current-runtime-follow-ups),
separate from the corrected horizontal viewport.

This is a measured approximation of foreground attenuation. The source's
chromatic echo and broader right-side ground vignette remain separate
material work; neither is reproduced by covering the background. See
[edge samples and limits](kitsch/store-fade.md).

## Neomil login SVG-to-Iced drift

The active card now uses the trace's #f63333 under the complete reference
palette, retaining its foreground role under custom palettes. On a clear
native source patch, RGB RMS falls from 14.99 to 1.93/255. Other foreground
printing keeps its existing palette role.

The inactive chamfers are 51px; each notch follows y390/405/483/496; the
first avatar tab ends at x498; card3's rail keeps its darker edge ink; and
USER01 retains 1.5px tracking. The renderer draws explicit era-owned plates
for notches, rails and tabs. The SVG and component sheet already held these
targets. [The drift record](neomil/login-rendering-drift.md) gives coordinates
and separates remaining portraits, artwork, typography and material work.

A new regression protects complete custom palettes, including changes that
retain the reference foreground, and verifies that transient coats override
reference fills. Existing input/submission tests remain intact. At 1537×947,
240 synthetic Unicode scalars change no pixels outside the password field;
the separate static hollow button slot remains untouched.

## Validation

All 253 Rust tests pass (208 library and 45 bar-window). All 24 SVGs parse
with unique IDs and resolved local references. Three source inventory gates
and three implementation gates pass without threshold changes:

| Screen | Source → SVG | SVG → Iced |
| --- | --- | --- |
| Neo-kitsch mailbox | ink placement 0.75 | matched shape area 91% |
| Kitsch store | ink placement 0.75 | matched shape area 97% |
| Neomil login | matched shape area 78% | matched shape area 98% |

These inventory figures are broad checks, not detail-completeness scores.
Native comparisons, fractional input/feedback captures and opening frames
were reviewed separately. Three intentional goldens are refreshed: Neomil
login, Neo-kitsch mailbox and Kitsch store. The full repository check passes
all 22 checks, including all 27 visual cases on their first attempt at
100.000% similarity. The packaged check's source and goldens match the final
reviewed files byte-for-byte.

Scratch evidence, source baselines, measurements, captures and logs are in
`/tmp/cp-eras-round3/`. Changes remain staged and uncommitted. Live desktop
verification remains separate; no desktop application was launched.
