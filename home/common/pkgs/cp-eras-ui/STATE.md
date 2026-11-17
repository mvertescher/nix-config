# State of the project, 2026-11-16

A snapshot written when the work paused, ahead of a possible move to
its own repository (`THEMES-SPLIT-PLAN.md` in the private wrapper).
What the project is for is in [GOAL.md](GOAL.md); the task lists are
[TODO.md](TODO.md) and `todo/`, and `todo/review-2026-11-13.md` is the
one that is current. This file says where things stand and what to pick
up; when it goes stale, delete it rather than trust it.

## Where it stands

Stage 2 of the goal, "Evaluate". All four eras' four screens and the
status bar are recreated and running on terra; Matt has not yet given
an overall verdict on the real display.

Scores from the two fidelity reviews (opinions from 1600-pixel images,
a different reviewer each round; `docs/review-2026-11-13/`,
`docs/review-2026-11-16/`):

| Era | 13 Nov | 16 Nov |
| --- | --- | --- |
| Entropism | 8 | 8 |
| Kitsch | 6 | 7 |
| Neo-kitsch | 7 | 7.5 |
| Neomil | 8 | 8.5 |

The 16 Nov scores were taken BEFORE that day's fixes (the print pass's
regressions, the review's gaps in all four eras, the bloom, the
above-print regions, the amber text, three veneer passes). No re-score
has been run since.

## What is on main

- **Line weights and plates** measured for Neomil
  (`docs/neomil/line-weights.md`; `style::Weights`, `DarkPlate`).
- **The print pass**, on for every era (`docs/print-overlay.md`): two
  registration copies under the art, per-era and per-screen; gates for
  fills (an enclosure test), ground, chroma; grain that holds across
  output sizes; a softer far copy; regions a screen marks as above the
  print (`print::AbovePrint`); bloom per era. The status bar does not
  wear it.
- **The Neo-kitsch veneer** as generated textures
  (`scripts/make-veneer.py`, `docs/neokitsch/veneer-texture.md`), six
  surfaces, six refinement passes.
- **The review gaps**: all 28 from the first review and the second
  review's lists for all four eras, each measured against the reference
  and fixed as named era parameters.
- **The bar**: Neomil's reference header, scaled by `ui_scale`;
  workspaces that answer clicks and show their state; the output
  re-creation leak fixed (two vendored crates).
- **The Neomil desktop** (`home/themes/neomil/design/` is the approved
  mock-up, `desktop.nix` the implementation), in desktop and TV
  fittings; terra runs the TV one.

All 25 goldens are current and reproduce from main.

## Open decisions (Matt's)

1. **Entropism's typeface.** The reference is set in Exo 2: letters land
   within about half a pixel along whole lines with the font's own
   kerning, and every other face fails. By the strict bar it is "not
   proven", only because the art scaled each text element by 2-3%.
   Choose: switch, re-set one screen first, or keep Rajdhani. Switching
   re-fits about 90-100 constants and needs static instances cut from
   the variable font at build time. Evidence:
   the Entropism Typeface review page; the proof's numbers are in the
   session scratch and not in the repository.
2. **Thin light lines.** A renderer correction is built on branch
   `thin-lines` and NOT merged: it has a defect at 1600x900 and thins
   lettering drawn as vector paths. The finding that matters is that in
   Kitsch, Neo-kitsch and Entropism the excess is line weight, not
   blending. Recommended: measure line weights per era, as Neomil's
   were, and drop the renderer change.
3. **On the real display, unseen by anyone:** the veneer, the bloom,
   the clickable workspaces (ten badges squeeze the title plate at the
   TV fitting), the lock screen (never seen at all), and whether the
   era screens feel slow with the print pass on a real GPU.
4. **The Neomil desktop's app interiors** (terminal, browser, editor
   colours): not designed.

## What to pick up next, in order

1. A third fidelity review, so the scores reflect what is on main.
2. The veneer's remaining list (section 4 of
   `todo/review-2026-11-13.md`): the store card's boxy patch first.
3. The medium gaps still listed in each era's
   `docs/review-2026-11-16/*.json`; Entropism's ink on sage.
4. The TV fitting: no pass has rendered the era screens at 1.5x since
   the grain and bloom changed; the print pass's cost was last timed
   before the enclosure gate.
5. Per-element print treatments still open: the halo's and the bloom's
   hatch is straight lines where the reference's is a maze; Neo-kitsch's
   near copy at bar ends ~1.5x the reference.

## How work was run

- One orchestrating session; subagents on Opus 5.5 in separate git
  worktrees, file-disjoint, never committing. The orchestrator checked
  each report's evidence by eye, merged, ran `cargo test`, re-rendered
  all 25 golden cases and compared before refreshing any.
- **One render at a time.** terra reaches 90 C under modest load; an
  agent that ran seven renders at once took the load average to 176.
- Measure the reference first. Several listed "defects" were measuring
  errors or misreadings (a window that included text; print copies read
  as marks), and three passes that mapped the reference before changing
  anything found it was not what earlier notes assumed.
- Public commits: Matt as author and committer, no trailers.

## Loose ends in the repository

- Branch `thin-lines` (pushed) is the only unmerged work.
- About twenty merged local branches and three worktrees under
  `~/workspace/cp-eras-*` on terra can be deleted.
- `docs/neokitsch/components.svg` and `bar.svg` still show the old
  cream; `src/widgets/stem.rs` docs cite superseded Neo-kitsch inks.
- Review pages (claude.ai artifacts, private to Matt) are not in the
  repository: Era Fidelity, Veneer, Print Texture, Hub Selection,
  Fidelity Gaps, Entropism Typeface, Neomil Desktop Design.
