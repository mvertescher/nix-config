# Twentieth reference checkpoint — AI, 2026-09-30

Two Sol medium lanes handle Neomil login notice ink and narrow-window
containment. Astra owns the shared reference-ink design, native review,
final block fitting and integration. Prior staged work is preserved;
changes remain uncommitted.

## Locked-card ink

The locked notice retains semantic `Ink::Dim`, with the existing source
notice red (#e63132) applied only to the reference palette. The SVG uses
the same fill; the component sheet's card-two specimen already does.
Changing even only the custom panel or Dim color disables the override.
Interaction coats also clear reference label ink so feedback wins.

All 30 fixed source comparisons improve for both SVG and native rendering
across six first/middle/last spans and five thresholds. At 4K, only 4,529
native locked-notice pixels change; the SVG changes 5,114. Co-located
source/old-SVG core samples, chosen independently of the new ink, also
show lower native red-channel error in all six spans. Exact glyph
contours and photographic softness remain open; brighter trial inks
brighten unsupported contours and are rejected. See
[the source measurements](neomil/login-primary.md).

## Notice containment

Login anchors scale separately on each axis while text size uses their
mean. Narrower aspect ratios therefore made notice lines exceed their
cards, including the active card at more extreme sizes. The fit measures
each complete run against the card interior and shrinks the notice block
only when needed. Relative font sizes and tracking are preserved across
its lines, as are the text, anchors, baselines, stretch and ink. Slots
without card bodies and rotated margin legends keep their geometry.

The initial horizontal-compression trial passes numerical bounds checks
but moves tiny letters onto Iced's outline path, visibly breaking strokes.
It is rejected after native review. Uniform font sizing retains hinted
glyphs. A second review keeps both lines at a common scale rather than
shrinking each independently. The canonical 1600×900 and 4K layouts need
no fitting and retain their original notice geometry.

## Integration

All 288 Rust tests and both affected fidelity gates pass at unchanged
thresholds. Source/SVG and SVG/native shape-area matches remain 94% and
99%. Thirteen final native captures cover four production sizes, canonical
preview parity and Dim-only custom colors, plus seven fractional states.
The custom Dim image is byte-identical to the old renderer; fitting alone
changes no pixels at either canonical size.

At 1537×947, all six reference states change the same 5,489 notice pixels;
custom colors change 5,466. All changes stay within the three notices.
Pointer and input differences stay inside the controls. Independent
outside-card probes find zero remaining notice pixels on all three cards
at 1537×947, 1200×900 and 900×1200. The probe excludes the neighboring
card's own printing rather than counting it as overflow.

Only the Neomil login golden is refreshed. All 22 repository checks pass;
all 27 visual cases match 100.000% on their first attempt. All 220 frozen
source/script/documentation/test/TODO hashes agree with the tested Nix
snapshot; the local original login image is unchanged. The two bounded
login tasks close. Source glyph/echo/material limits remain open. Final prose records these
results without changing tested runtime, SVG, scripts, tests or goldens.
Evidence is under `/tmp/cp-eras-next/ai/`; rejected compression artifacts
are preserved in `mesh-trial/`. Live desktop input/IME, hardware
presentation and actual greetd validation remain separate.

## Next-round scratch review

A separate AJ investigation rejects global Medium weight and brighter
Entropism mailbox body fills. Astra re-renders the baseline and independently
reproduces 9/8/7/6 local span losses for Medium at green thresholds
110/130/150/165, despite improved whole-row overlap. The worst span loses
.1165 IoU; mean RGB error rises 39.751→40.694. The two brighter fills also
increase error to 40.575 and 41.880. The milder fill retains one very small
loss at threshold 165 that rounded worker results had counted as a tie.
All trial changes remain within body row bands; no repository artwork
changes. Evidence: `/tmp/cp-eras-next/aj-entropism-mail/root-results.json`.
Widths/layout are already fitted; exact printed contours remain open.

A continuous fifth-frame gradient in the Neo-kitsch store also stays in
scratch. It removes the previous hard join and improves relative contrast,
but Astra's fixed bottom controls show worse RGB RMS: 9.66→14.89,
9.98→15.47 and 10.02→15.44 on ordinary cards, and 15.94→22.24 selected.
Ordinary peaks and already-bright gaps become brighter than the source;
the selected bottom's positive red area rises to 116 against source 83.6.
Relative contrast alone hides those losses. The frame task remains open;
this trial does not establish a blocker or a recovered source material.
Evidence: `/tmp/cp-eras-next/aj-neokitsch-frame/root-results.json` and
the worker's fixed upper/side/bottom/join controls in that directory.
