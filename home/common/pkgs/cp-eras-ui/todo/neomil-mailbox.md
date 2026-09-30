# Neomil mailbox correctness

Audit and implementation, 2026-09-21. The reference is
`images/img-08-main.png` (3840×2160, source #61), traced by
`docs/neomil/mailbox-trace.svg` at 1600×900. The dashboard hub opens
`screens::mail::MailBox`, also exposed by `cp-eras-ui-mailbox`.
`cp-eras-ui-mail` is a separate synthetic-data client. Fixed lorem body
content and decorative action/scroll artwork are intentional in the
trace screen and are not backend defects.

## Audited fixes

- [x] **Advance the standalone opening animation.** The launcher omitted
  `MailBox::subscription()` and stayed blank with an unfrozen clock.
  It now batches navigation and animation subscriptions. A permanent
  `mailboxLive.neomil` golden case exercises actual startup; the hub
  already forwarded this subscription. A five-second unfrozen capture
  matches the completed frozen frame exactly.
- [x] **Separate initial fixture content from selected-message content.**
  Explicit fixture state preserves the source's initial “Urgent
  Information (!)” heading. Selecting any row shows its own subject,
  including row 2 (“I'm worried man”) and subsequent returns to it.
  Re-entering the screen preserves the selected message. Tests cover
  all four eras, repeated selections and keyboard bounds.
- [x] **Correct source typography before porting it.** Body text is
  Regular 17.5 at (750,347.5), replacing the short 15px text at x740.
  All ten SVG line widths match source within one design pixel. Header,
  protocol, tape, badge and margin lettering use measured vector paths;
  heading, row and button type sizes/inks are corrected. See
  [the fidelity record](../docs/neomil/mailbox-fidelity.md).
- [x] **Restore the blue/cyan background and panel material.** Clear source
  patches establish the same immutable background as the dashboard.
  Mailbox-specific panel and badge fields preserve their own colors and
  scan modulation. Independent clear-ground RGB RMS falls 12.289→1.787;
  panel spatial holdouts fall about 9.01→1.42. No other screen's material
  changes. A fractional-scale opacity regression covers internal joins;
  panel material and maker printing follow the existing opening clip.
  See [material measurements](../docs/neomil/mailbox-material.md).
- [x] **Correct cartridge silhouettes and preserve their inner detail.**
  SVG and Rust share complete stepped casings, contact strips, inset
  lower tiles, C-shaped discs and both inner rings. Source-measured
  row placements replace regular icon pitch; the selected variant shifts
  left 15.8333px and stays inside the expanded list-opening clip.
- [x] **Restore omitted printing and lower-row shading.** Rotated
  BETTERLIFE TEC/PETROCHEM, the maker frame and both margin codes are
  vector paths. Normal rows darken down to `#1d0708`; feedback still
  overrides resting fills/printing. Generic data defaults preserve the
  other eras. The old rotated-text limitation no longer applies.

## Acceptance

- [x] Review original → corrected SVG → fresh Iced captures, including
  native-source crops, rest, opening motion, selection and row feedback.
  Keep component excerpts and source documentation consistent.
- [x] Run Rust tests and the complete repository check; refresh only
  visually reviewed intentional golden changes. Preserve the previously
  staged dashboard/performance work and the other eras' goldens.
- [x] Confirm unfrozen standalone startup. Frozen captures cannot
  establish this; the headless startup case remains separate from
  live desktop verification.

## Remaining fidelity and desktop work

- [x] Correct the reader panel's bottom-right corner. Native source
  crop (1400,680)..(1460,705) has a square primary corner; the trace path
  ends `V691 L1442,699`, introducing an 8px chamfer. Remeasure and update
  SVG, component excerpt and Iced together, preserving the actual
  upper-right chamfer and side bar. This primary geometry defect was
  found by the [all-reference recheck](../docs/reference-svg-audit.md#neomil)
  after the earlier six fixes. The correction includes the inward side
  step and measured primary stroke; [measurements](../docs/reference-svg-fixes.md#neomil)
  distinguish the thin lower contour from its thicker upper stem.
- [x] Refine the remaining upper panel contour against native source.
  The material and bright outline now share the measured warped top and
  connected chamfer. Native source-edge IoU is .767 across the upper
  contour and .848 in the clean corner. Fractional and 0.35-second opening
  crops have no detached edge or fill seam. Fine echoes remain separate.
  See the [ninth checkpoint](../docs/reference-svg-round9.md).
- [ ] Reconstruct source-supported fine text/cartridge echoes and edge
  softness. The primary shapes and low-frequency fields are corrected;
  they do not establish pixel-perfect printing. The tiny cartridge
  normal terminal strips now use 20 slanted ribs at the measured cadence;
  the selected strip retains its separate 22-rib art and corrected ink.
  X fixes the sloped-top coordinate conversion, improving all four native
  row comparisons. Fractional selection/press behavior is preserved.
  Ordinary-terminal contrast, edge softness and faint copies remain.
  Inspect native source crops before
  adding secondary art; do not invent glow/noise or weaken gate thresholds.
- [ ] Recover the exact common background texture/authoring recipe if
  source material becomes available. Identical clear source patches show
  fixed shared texture, not independent procedural noise. The original
  effect and echo cause remain unknown.
- [ ] Live desktop verification of input, hub navigation and route-return
  behavior. Headless pointer and startup tests do not close this; an
  authorized GUI launch is required when the user is ready.

## Earlier mailbox batch validation record

Baseline: G1i PASS, 14/31 shapes / 86% source area; G2i PASS, 23/36 shapes /
89% trace area. The old rest capture matched its golden exactly despite
all six defects. These scores establish broad layout/regression stability,
not detail correctness. Triptych/crop review and runtime checks remain
required.

Tooling corrections: `scripts/render.sh --live` explicitly unsets the
frozen clock and rejects a simultaneous `--at`. The golden matrix includes
one unfrozen mailbox case. Triptych captions now count pixels whose maximum
RGB difference exceeds 8 directly; the old fuzzed AE metric underreported
0.8% versus the actual 9.725% on the baseline. A threshold-boundary fixture
confirms that 8 is excluded and 9 included. Output strips inherited PNG text
metadata so Pillow can read source-containing triptychs. No fidelity or
golden thresholds were weakened.

The final local run passes all 238 Rust tests (193 library + 45 binary).
Reviewed rest/selection and actual pointer hover/held previews preserve
row feedback and unread-stamp contrast. Source/SVG/Iced crops cover
header, body and rows; native 4K and fractional 1537×947 renders cover
panel material and rotated printing. Zero-time and mid-opening captures
keep panel material/printing/edge art inside the reveal. The component
sheet now takes mailbox excerpts directly from the corrected trace.

The row follow-up removes an invented solid spine: native scans identify
a dim offset copy instead. Primary rows use measured geometry, a 15px
chamfer, thin dark borders and corrected selected ink/fill. The NEW stamp
uses a source path and distinct selected offset; its ink follows feedback.
Neomil's Rajdhani baseline conversion is 0.84, matching the scene renderer;
the prior 0.95 raised text by about 2px. Other eras retain their calibrated
positions. The corrected source gate passes 11/31 shapes / 84% source area;
the implementation gate passes 13/20 shapes / 91% trace area. The changed
counts reflect extracted shapes; they do not measure fine printing.

The first full matrix exposed a harness issue in its new unfrozen case:
headless pixman with default FIFO presentation retained the opening frame
at 15s, failing twice with 96.983% similarity. A controlled Nix rerun of the
same packaged binary and golden with `ICED_PRESENT_MODE=mailbox` passes
100.000%, with no pixels beyond the per-channel tolerance. Live-clock
cases now use that mode, matching `scripts/render.sh`; frozen cases retain
their existing mode. Clock and screenshot thresholds are unchanged.

Final review also restores the source/SVG lower-list hairline inside the
list reveal. It is ordinary list artwork, avoiding an extra full-size
software surface/cache entry for one line.

Final verification passes all 238 Rust tests, both fidelity gates and all
19 repository checks, including all 26 golden cases. Both Neomil mailbox
cases match their golden at 100.000%, including the unfrozen Nix startup case.
The final lower rule changes only 540 pixels at x241..510/y875..876 and
remains hidden at animation time zero. SVG/Iced maximum-channel >8-pixel
coverage is 4.448%, versus 9.725% in the baseline comparison.

Only the Neomil mailbox golden was refreshed for this work. Previously
staged dashboard/performance code and other goldens were preserved; the
Neomil implementation outside its mailbox block is byte-identical to the
pre-task index. All changes remain staged, with no commit or activation.

## Lower-contour follow-up, 2026-09-21

The reference recheck's NM2 correction now includes the inward side step,
square lower corner and source-measured stroke. Source/SVG/Iced native
crops and opening captures are reviewed; G1i/G2i pass. Remaining upper-edge
and echo work stays open.
The [correction record](../docs/reference-svg-fixes.md) contains current
measurements and validation: 250 Rust tests and all 19 repository checks,
including 27 visual cases. Changes remain staged and uncommitted.
