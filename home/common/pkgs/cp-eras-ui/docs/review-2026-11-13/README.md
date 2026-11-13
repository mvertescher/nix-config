# Fidelity review, 2026-11-13

The first review for stage 2 of [GOAL.md](../../GOAL.md) ("Evaluate"): each
era's four accepted renders (`tests/golden/<screen>-<era>-1600x900.png`)
beside the reference art they recreate, read by one reviewer per era. Matt
was given the same comparison as a review page; this directory is the
record of what the reviewers wrote. One JSON file per era: an overall
read and score, then per screen a verdict, what matches, the gaps (most
noticeable first, each marked `known` if an open item already tracks it,
`new` if not, `unsure` if it could not be judged), and at most one
question for Matt.

Scores are a reviewer's opinion from 1600-pixel images, not measurements.

| Era | Overall | Login | Mailbox | Store | Dashboard |
| --- | --- | --- | --- | --- | --- |
| Entropism | 8 | 8 | 8 | 8 | 7 |
| Kitsch | 6 | 7 | 6 | 6 | 7 |
| Neo-kitsch | 7 | 7 | 8 | 7 | 6.5 |
| Neomil | 8 | 8.5 | 7.5 | 7.5 | 8 |

## What to take from it

- **Text on a filled surface is too pale and thin in three eras**:
  Neo-kitsch's gold panels and CTA bars, Neomil's selected red row and
  button, Entropism's sage fills. The reference prints firm dark ink in
  all three. Reported independently by three reviewers and visible in the
  live Neomil mailbox capture. Possibly one shared cause in how ink on a
  selected fill is coloured or weighted. **Not yet confirmed in code.**
- **Kitsch's label face is wrong**: the reference sets labels and
  microcopy thin and wide-extended (Eurostile-like); ours is bold
  condensed, and several weights are inverted.
- **Surface material** (print grain, scanlines, ghosted double-printing)
  is the top remaining gap on Neomil and Entropism, and is already what
  most open items in `todo/` are about.
- Many gaps are `new`: not tracked by any open item. They have not been
  added to `todo/` yet; that waits on Matt's answers.

## Caveats

- Reference pairing: `images/entropism-store.png` is the module hub and
  `images/entropism-dashboard.png` is the store (`docs/sources.md`). The
  JSON here uses the corrected pairing.
- Three pairs were re-checked by the orchestrator (Entropism store,
  Kitsch mailbox, Neo-kitsch dashboard) and the reports held. The rest are
  reviewer claims.
- The reference is concept art with glow and print texture; crisp vector
  drawing will not match it pixel for pixel.

## Open questions put to Matt

1. Neomil hub: what should keyboard focus on a tile look like? The
   reference shows no selected tile.
2. Neomil hub: five of six tiles lead nowhere and the mailbox is
   unreachable from it. Leave them inert, or wire the mailbox?
3. Neo-kitsch: accept a pre-rendered raster texture for the wood veneer
   instead of vector grain?
4. Entropism: match the firm dark ink inside sage fills, or keep the
   lighter look?
5. Kitsch: accept an open-licence Eurostile Extended look-alike for
   labels?
6. All eras: is print texture worth chasing before animation?
