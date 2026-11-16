# Fidelity re-review, 2026-11-16

The second review for stage 2 of [GOAL.md](../../GOAL.md) ("Evaluate"),
three days and about seventy commits after [the first](../review-2026-11-13/README.md).
Same method: each era's four accepted renders
(`tests/golden/<screen>-<era>-1600x900.png`) beside the reference art,
read by one reviewer per era, who scored from what they saw and only
then compared with the first review. One JSON per era, in the first
review's shape plus `previous_score`, `regressions`, and per screen
`since_2026_11_13` (each of the first review's gaps marked fixed,
improved, unchanged or worse).

Scores are a reviewer's opinion from 1600-pixel images, not
measurements, and a different reviewer wrote each round.

| Era | Overall | Login | Mailbox | Store | Dashboard |
| --- | --- | --- | --- | --- | --- |
| Entropism | 8 → 8 | 8 → 8.5 | 8 → 8 | 8 → 8 | 7 → 7.5 |
| Kitsch | 6 → 7 | 7 → 7.5 | 6 → 7 | 6 → 7 | 7 → 7.5 |
| Neo-kitsch | 7 → 7.5 | 7 → 7.5 | 8 → 7.5 | 7 → 7.5 | 6.5 → 7 |
| Neomil | 8 → 8.5 | 8.5 → 8.5 | 7.5 → 8.5 | 7.5 → 8.5 | 8 → 8 |

## What to take from it

- **The work since the first review shows.** Kitsch gained most: of its
  20 first-round gaps, 15 are fixed and 4 improved. Neomil's mailbox and
  store each gained a point, from dark ink on red and the print copies.
- **The print overlay helps and has side effects.** Every reviewer said
  the screens no longer read as flat vector art. Every reviewer also
  listed regressions from it: Entropism's grain is coarser and stronger
  than the reference's (std 5.0 against 3.7 on sage at 1600 px) and its
  smallest captions are less legible; Neomil's dark emblem inside the
  light active login card is smeared (checked by the orchestrator: the
  reference is crisp black, ours has a diagonal band through it) and
  its header badge shows a second "T1"; Neo-kitsch gets hard bright
  slivers at the ends of its bars where the reference has a soft halo;
  Kitsch's copies are neutral grey where the reference's halo is mint,
  and its BRAINDANCE text is smeared (already known).
- **Neo-kitsch's mailbox went down** (8 → 7.5), the only drop. The
  second reviewer scored the selected row harder: its ink is still pale
  grey (darkest pixel 90 against the reference's 57, unchanged), its
  wood lacks the reference's lens figure and bevel, and the tab under
  it draws as a black notch where the reference has a gold tab (checked
  by the orchestrator; all three hold).
- **Two high gaps the first review missed in Kitsch**: the dashboard's
  mint connector lines between the fan blades were never drawn
  (checked: absent on 11-13 too, and the first review listed them as
  matching), and the mailbox's tab labels are a fifth too small, thin
  and grey.
- **Light text colour in Neo-kitsch**: ours is pale cream (#E7C686), the
  reference a saturated amber (#FFCD79) with a halo. New, and on every
  screen of that era.
- **No bloom anywhere.** Three of four reviewers named the reference's
  soft glow around bright elements as missing. The print pass draws
  copies and grain, not glow.

## Caveats

- Reference pairing as before: `images/entropism-store.png` is the
  module hub and `images/entropism-dashboard.png` is the store; Neomil's
  `img-06-private` is the login and `img-08-main` the mailbox.
- Three claims were re-checked by the orchestrator on the reviewers'
  crops (Neomil login emblem, Neo-kitsch selected mail row, Kitsch
  connectors) and held. The rest are reviewer claims.
- The Neomil dashboard's inverted LOCATIONS tile is the owner's
  addition, not in the art; its reviewer did not score it as an error.

## Questions the reviewers put to Matt

1. Entropism: search again for a face closer to the reference's rounder
   sans, or accept Rajdhani?
2. Neomil mailbox: the reference's selected cartridge is dark inside a
   red outline; ours fills it mid red. Invert it?
3. Kitsch mailbox: are the small tab labels and message title a
   deliberate fit to the trace, or should they be measured against the
   art?
4. Neo-kitsch: move light text from cream to the reference's amber?
