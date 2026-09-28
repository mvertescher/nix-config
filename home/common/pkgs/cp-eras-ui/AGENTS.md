# cp-eras agent guidance

## Model allocation

Agreed 2026-09-27. Keep orchestration and verification on GPT-6 Astra,
and use smaller models for scoped implementation work.

| Role | Model | Reasoning | Responsibilities |
| --- | --- | --- | --- |
| Orchestrator | `gpt-6-astra` | Retain the session setting | Select tasks, define acceptance checks, make architectural decisions and integrate changes. |
| Verifier | `gpt-6-astra` | Retain the session setting | Review diffs, source fidelity and test evidence before closing tasks. |
| Default implementation worker | `gpt-6-sol` | `medium` | Scoped Rust fixes, SVG corrections, tests and performance work. |
| Simple-task worker | `gpt-6-luna` | `medium` | Inventories, documentation cleanup and scripted measurements with explicit instructions. |

The orchestrator may also act as the verifier. Keep ambiguous visual
judgments and changes spanning shared rendering behavior with Astra;
workers should report uncertainty and unresolved failures to it.

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

Astra reviews the actual changes and evidence, not just the worker's
completion claim. For fidelity corrections, compare the original image,
the rendered SVG and the Iced result in the affected region and relevant
states. Broad shape gates and matching goldens alone do not establish
source fidelity. Run the relevant checks required by the repository.

Model choice aims to reduce total work and cost. Keep handoffs compact
and avoid repeated investigation; a smaller model does not guarantee
fewer tokens or fewer repair rounds. Revisit the allocation using observed
results on cp-eras tasks.
