# Nineteenth reference checkpoint — AH, 2026-09-30

Two Sol medium workers handle Entropism store title fitting and Neomil
inactive login notices. Astra reviews source/SVG/native evidence and owns
integration. A third worker could not start because the agent service
reached its thread limit. Prior staged work is preserved; this batch also
remains uncommitted.

## Entropism title width

The SVG fits MAGNUM and 650 independently, preserving the baseline and
inks. Card two's title width changes from 321 to 303 native pixels against
304 in the source. All 36 fixed first/middle/last comparisons and 108
individual glyph comparisons improve across selected/ordinary cards and
three thresholds. Subtitles and blank bands are unchanged; the fourth
card's fractional clip is preserved.

There is a local gap tradeoff: 73 of 96 individual gap comparisons improve,
eight are unchanged and 15 regress. The selected M–A gap gains 36 dark
pixels at its middle threshold. Exact contours/gaps remain open. The SVG
changes 22,026 pixels only in title rows. Fresh native 4K and 1600×900
captures are pixel-identical to the previous accepted native result;
this correction is restricted to the reference. See
[the title measurements](entropism/store-typography.md).

## Neomil inactive notices

Both inactive cards' notices were about ten native pixels too low. Their
new design baselines are 584.2 and 592.8; the active card is unchanged.
At red threshold 130, native occupied rows are 1389–1401 and 1409–1421,
against source 1389–1400 and 1410–1421. The one-row edge differences,
dim locked-card ink and font contours remain explicit.

Across five thresholds and first/middle/last crops, 24 of 30 native
comparisons improve and six are unchanged, with the same result for SVG.
The unchanged high-threshold locked-card comparisons have no rendered
bright core. Native changes affect only 16,262 notice pixels at 4K and
4,619 at 1600×900. Blank controls, the active card and all other artwork
are unchanged.

Seven fractional state pairs each change the same 4,910 notice pixels,
including custom colors, long synthetic input and a hidden caret. Pointer
and input differences stay within the existing control region. The rest
preview matches production exactly. Review finds an existing fractional
notice overhang of 5.3/5.7 pixels; it is recorded as separate follow-up.
See [the notice measurements](neomil/login-primary.md).

## Integration

All 286 Rust tests, the release build and all four affected fidelity gates
pass at unchanged thresholds. Source/native shape-area matches are
Entropism store 89%/91% and Neomil login 94%/99%. All 24 SVGs parse with
unique IDs. Only the Neomil login golden is refreshed.

The full repository check passes all 22 checks and all 27 visual cases;
every visual case matches 100.000% on its first attempt. All 219 frozen
source/script/documentation/test/TODO hashes match the tested Nix snapshot.
The 14 state/local region comparisons and source-specific controls above
are reviewed independently of those broad gates. Final prose records the
result; runtime, SVGs, scripts, tests and goldens remain unchanged from
the tested snapshot.

Evidence is under `/tmp/cp-eras-next/ah/`, including source/native
measurements, review montages and `state-review.json`. Live desktop/IME,
hardware presentation and actual greetd validation remain separate.
