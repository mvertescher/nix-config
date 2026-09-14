## Current remaining work (2026-09-14)

The linked workstream records preserve the implementation history. Earlier
"still open" lists are superseded by later completion entries, including
the five-task feedback/native-control/extractor batch committed in `de56b25`.

- **Neomil dashboard source fidelity:** actionable corrections were found
  by the 2026-09-14 visual audit. Follow the
  [measured plan](todo/neomil-dashboard.md); the earlier 94% shape-area PASS did not establish detail completeness.
- **Kitsch dashboard hover:** needs a hub-specific design. Press is done;
  the existing blade trails do not establish an additional hover state.
- **Animated interaction transitions:** need reviewed timing/easing and
  transition rules. Component sheets specify destination drawings only;
  existing boot-in/caret animation timings are not interaction timings.
- **Desktop verification:** native input/IME, hub navigation, feedback,
  and first-frame haze/route latency still need a live check. Headless
  tests and previews do not close these items.
- **Future widgets:** slider ticks, shared icons, spec/log rows, meters,
  tooltips/modals and tab widgets wait for actual callers or reference
  material, as detailed in the [toolkit roadmap](todo/toolkit.md).
  Do not build unused widgets to close its umbrella checkboxes.
- **Extraction diagnostic:** the Kitsch mailbox yellow extraction remains
  explicitly "do not fix" in the [pipeline record](todo/design-pipeline.md).

The three reader follow-ups are resolved: Entropism cursor wording,
Neomil's title-card index and the separate hollow Login-button slot.
See [the source audit](docs/source-followups.md); the password caret
was already correct, and the slot's state semantics remain unknown.

Follow-up audit: three native-event regression tests now exercise button
cancellation/disable, touch release/loss and selected-range Unicode commits
after input re-enabling. All five control tests pass using the headless
software renderer. No production behavior changed; OS IME/preedit and live
desktop verification remain open.

## Workstream records

- [Neomil dashboard fidelity](todo/neomil-dashboard.md): the current source
  correction plan, accepted measurements, validation and remaining detail work.
- [Design pipeline and component sheets](todo/design-pipeline.md): source
  audits, trace corrections, gates and extraction diagnostics.
- [Bar styling](todo/bar.md): measured era chrome and implementation history.
- [SVG-to-iced conversion](todo/svg-to-iced.md): renderer coverage, scene
  conversion and the widget-layer decisions.
- [Toolkit, interactions and themes](todo/toolkit.md): reusable controls,
  theme plumbing, motion, caller-dependent widgets and desktop follow-ups.
- [Verification infrastructure](todo/verification.md): headless rendering,
  desktop renderer troubleshooting and golden-history notes.

The latest Neomil batch integrates dashboard-local primary-ink mapping,
tape/chip exterior echoes, and GO HOME body/maker-shape echoes. All 209 Rust
tests and both fidelity gates pass; reviewed state/opening captures and two
dashboard goldens are updated. The full repository check passes all 19 checks, including all 25 golden cases.
The [dashboard record](todo/neomil-dashboard.md) names the remaining local
source fits and the work that needs original material or live verification.
