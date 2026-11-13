# cp-eras agent guidance

## Who works on this

Since 2026-11-13 the crate is run by one Claude session, which
orchestrates, verifies and commits. The GPT allocation agreed on
2026-09-27 (Astra orchestrating and verifying, Sol and Luna as workers) no
longer applies; rounds 1-85 in `docs/` were done under it.

What the project is for is in [GOAL.md](GOAL.md): recreate the four eras,
Matt evaluates them, animations, a framework per era, and mobile in the
long term. Read it first. Where a visual judgment is Matt's to make, show
him the comparison rather than deciding fidelity for him.

When work is delegated to subagents they run on Opus 5.5 (Matt,
2026-11-13; with the Agent tool, `model: "opus"`), and the rules below
still hold: the
orchestrating session reviews the actual changes, keeps ambiguous visual
judgments and anything touching shared rendering for itself, and workers
report uncertainty rather than closing a task on it.

Live desktop runs are allowed (Matt, 2026-11-13): these apps may be
launched on terra's logged-in session, sent test input and screenshotted.
Never a real password, never a workspace switch, and say so before each
round. See `docs/live-desktop-2026-11-13.md` for the method.

## Keep delegation focused

- Select the worker model and reasoning level explicitly when spawning.
- Give each worker a concise brief with the task, owned files, relevant
  references, acceptance checks and applicable repository instructions.
  Use fresh task-specific context where supported; avoid copying the full
  conversation by default.
- Assign file-disjoint scopes. Coordinate changes to shared renderer code
  and serialize shared build or golden-update operations.
- Request short reports: changed files, checks run and their results,
  evidence paths, and unresolved issues. Put detailed findings in a scoped
  document when needed.
- Workers do not commit, push or deploy. The orchestrator reconciles
  `TODO.md` and `todo/` after verification.

## Verify before closing work

The orchestrator reviews the actual changes and evidence, not just the worker's
completion claim. For fidelity corrections, compare the original image,
the rendered SVG and the Iced result in the affected region and relevant
states. Broad shape gates and matching goldens alone do not establish
source fidelity. Run the relevant checks required by the repository.

For new viewport-size source studies, use `scripts/sample_source.py` and
retain its metadata. See [source sampling](docs/PIPELINE.md#sampling-originals-at-a-viewport-size)
for uniform scale, top-left alignment, unknown padding and filter-edge
limits. Verify a historical native baseline against its accepted capture
or golden; a scratch filename alone does not establish provenance.

Model choice aims to reduce total work and cost. Keep handoffs compact
and avoid repeated investigation; a smaller model does not guarantee
fewer tokens or fewer repair rounds. Revisit the allocation using observed
results on cp-eras tasks.
