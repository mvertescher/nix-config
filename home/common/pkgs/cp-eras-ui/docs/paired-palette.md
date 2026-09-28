# Paired palettes for ink placement

G1i compares a source image with its SVG trace. For Kitsch and Neokitsch it
uses the `inks` gate: each measured colour family has an 80 × 45 placement
grid, and `spec_diff.py` pairs families by RGB distance before comparing
those grids. A candidate's independent eight-colour k-means fit can use most
of its bins on a stronger ground gradient. In Kitsch login, that merged two
teal source inks into one candidate family and made unchanged foreground
fail at 0.22 weighted placement IoU.

The ink gate now extracts the source normally, then gives that source spec to
the candidate with `extract_spec.py --palette-from source.json`. Pixels are
assigned to the source's fixed RGB centers, and each center keeps its source
ground/ink role. A candidate entry reports the **mean RGB of its own assigned
pixels**, not the reference center; an empty bin has no reported colour.
Pixels at RGB distance 110 or more from every source center stay unassigned.
That distance is the existing family-pairing boundary in `spec_diff.py`.
The candidate spec records their canvas coverage, and the diff prints it as
review evidence. It does not add a failure threshold for candidate-only
colours; the existing ink gate permits extra annotations. Source families
that disappear or move still fail through the existing missing-family and
placement rules. Supported candidate pixels with a changed hue retain their
measured RGB, so the colour pairing still sees the change.

Only G1i's Kitsch and Neokitsch comparisons use this mode. The standalone
extractor, the axis-aligned G1i shape gates, and G2i keep independent
quantization. In a local 2026-09-27 audit of all 16 source/SVG pairs, the
eight ink pairs passed with the paired palette (Kitsch: 0.74, 0.73, 0.95,
0.76; Neokitsch: 0.76, 0.70, 0.72, 0.65, in login/dashboard/mailbox/store
order). Kitsch login changed from a false FAIL to PASS. Pairing all 16
would have made three previously passing Entropism shape cases fail because
fixed source bins fragment thin rules. A separate check of 16 available
SVG/Iced image pairs found that pairing would make every existing G2i shape
comparison fail, often through a lost shape class despite high matched area.
That is why the G2i path does not use it.

The synthetic extractor test runs the full ink gate on a foreground over a
changed ground gradient. It passes unchanged ink and fails when that ink is
removed, moved, or replaced with a contrasting colour. It also checks that
a novel colour is reported as outside the source palette without changing
the historical candidate-only-colour policy.
