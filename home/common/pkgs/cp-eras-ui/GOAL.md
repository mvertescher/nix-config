# What this project is for

This file is the owner's intent for cp-eras-ui, kept in the repository so
it is versioned with the work. It changes only when Matt changes the goal.
Everything else in the crate -- `TODO.md`, `todo/`, `docs/` -- is in
service of what is written here; when they disagree with it, this file
wins and they get corrected.

## The goal

Recreate four fictional interface styles ("eras") from the Cyberpunk 2077
UI reference art as working software, then build on them:

| Era | Theme directory |
| --- | --- |
| Entropism | `home/themes/entropism` |
| Kitsch | `home/themes/kitsch` |
| Neo-kitsch | `home/themes/neokitsch` |
| Neomil (Neo-militarism) | `home/themes/neomil` |

The stages, in order. A stage is not started in earnest until the one
before it is accepted.

1. **Recreate.** Faithful reproductions of each era's reference screens
   (login, dashboard, mailbox, store) and a status bar in the same
   language, running on a real desktop.
2. **Evaluate.** Matt reviews the reproductions against the references
   and says how they are doing. Fidelity is his call: where a visual
   judgment is his to make, he is shown the comparison rather than told
   the result.
3. **Animate.** Add motion to the accepted reproductions.
4. **Framework.** Grow each era into a larger UI framework: the reusable
   widgets and rules that let new screens be built in that era's style,
   not only the four reference screens.

This is what the reproductions are FOR (Matt, 2026-11-14): eventually every
component -- surfaces, outlines, text treatments, effects -- is to be
applied to new designs. So work on a reproduction is not finished when
the screen matches: the thing that makes it match should be a component
or a named treatment that a new screen can use, with its measured
parameters (line weights, colours, effect strengths) recorded, not a
one-off fitted to that screen. Getting those details exactly right now,
pixel for pixel against the reference where it shows them, is worth the
depth, because every later screen inherits them.

## Long term

- **Mobile.** Plan for these themes on iOS and Android, or a general
  mobile theme drawn from them (Matt, 2026-11-13). This is a direction,
  not current work. What it asks of today's decisions: keep each era's
  description (palette, type, shapes, spacing, motion) separable from the
  desktop toolkit that draws it, so it can be carried to another platform
  without re-deriving it from the reference art.

## How the work is run

- One Claude session orchestrates, verifies and commits (`AGENTS.md`).
  Subagents it dispatches run on Opus 5.5.
- Matt is asked questions along the way and shown artifacts to review
  whenever something is uncertain.
- The task list is `TODO.md` and `todo/`; this file holds no tasks.

## History

- 2026-11-14: the reuse paragraph under the stages, from Matt.
- 2026-11-13: written, from Matt's statement of the goal that day, when
  the project passed from the GPT agents that ran rounds 1-85 to Claude.
  The mobile direction was added the same day.
