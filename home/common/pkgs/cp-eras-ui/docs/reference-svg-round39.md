# Reference SVG audit — BC–BD

The Kitsch compliance studies identify a repeated-text placement mismatch,
but neither the `ONLY` contour nor the two-line placement trial passes
local source controls. No SVG, runtime, component or golden change is
accepted. All 29 broader TODO boxes remain open.

The contour revision improves twelve whole-word RGB comparisons, while
worsening ordinary-card O counters and N/L gaps. Source-to-source text
correspondence then supports small right/down offsets on cards 3 and 4.
An independent edge-correlation audit confirms the card-3 offset across
six regions on both lines; card 4 is less precise because of its fade.

One fixed SVG translation of the two complete text lines improves all
twelve whole-line RGB comparisons. It nevertheless increases first-line
source-gap ink at 4K and worsens card-3 words and endpoints at smaller
sizes. All changed pixels remain inside the intended text blocks, with
zero alpha changes. Locality and average error do not overcome those
failures. Astra rejects the trial before native integration; the worker's
recommendation for further review is not acceptance. The full measurements,
control corrections and source-sampling limits are in the
[typography record](kitsch/typography-fit.md#bcbd-separate-glyph-shape-from-two-line-placement).

The round starts from all 312 hashes of the completed BA checkpoint, with
no unstaged production changes. A Sol implementation worker owns the
scratch placement candidate; Astra independently checks registration,
SVG structure, full RGBA locality, words, counters and gaps. The agent
service rejects a second lane with `agent thread limit reached`, so Astra
performs that audit directly.

This round changes documentation only. BA's 292 Rust tests, 22 repository
checks and 27 visual cases remain the latest implementation validation;
they were not rerun or claimed as new trial verification. Prior staged
work is preserved. Nothing is committed, pushed or deployed.
