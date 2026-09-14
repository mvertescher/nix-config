# Source follow-ups, 2026-09-14

Three observations left by the component-sheet readers have been checked
against the local source images. Source identities are in [sources.md](sources.md).

## Neomil run index

`images/run-neomil/53-5707b711.png` visibly reads NEO MILITARISM /
SUBSTANCE OVER STYLE. It is the title card at canonical position 53,
followed by screens 54–62. The three-card Login is position 59,
`59-6cfb2011.png`, matching the full-resolution `img-06-private.png`.
The source table, Kitsch run table and Neomil module header now agree.

## Entropism list fill

In `images/entropism-mail.png`, the filled first row says YOU'LL REGRET
THAT / JACKIE. The second row says URGENT INFORMATION / MOM and has an
open envelope; the message panel also says URGENT INFORMATION / Mom.
The filled row therefore differs from the open message. This supports
the adopted list-cursor interpretation, but a still does not prove a
particular pointer gesture or transition.

The mailbox trace comments and component captions now distinguish this
cursor fill from the reverse-video message heading, buttons and tier
badge. The trace drawing, animation IDs and timings are unchanged.
Kitsch's open-envelope read/unread distinction remains an observation,
not a new interaction state.

## Neomil Login-button slot

The dark mark attached to the Login bar is an open-top hollow U, also
visible in run stills 55 and 57. It is separate from the password's `__`
caret at the text baseline. The earlier TODO conflated these two marks.
The old trace used a solid 2×15 rectangle; the app drew that rectangle
before the opaque action, so it was hidden.

The replacement is three filled bars in the existing dark `#420f10` ink.
At 1600×900 design scale the outer bounds are x 497.3342–500.3799,
y 635–654.6317. Side widths are 0.8031 and 0.7966; the bottom is 0.8871
high. The top stays fixed at the action's y 635 edge. This bounded fit
changes six rectangle edges, without changing the card or action.

Validation uses the fixed native source crop x 1185–1212, y 1529–1580
at 3840×2160, divided into complementary 2×3-pixel checker folds. Each
geometry fit was rendered as an actual full SVG before scoring its
unseen fold. RGB RMS, on the 0–255 channel scale:

| held-out fold | pixels | old solid rectangle | hollow slot |
|---|---:|---:|---:|
| first | 688 | 23.242 | 3.090 |
| complementary | 689 | 23.220 | 2.948 |
| final fit, whole crop (descriptive) | 1377 | 23.231 | 3.025 |

The trace, both component-sheet specimens and Rust use the same three
rectangles. `Slot::action_marks` draws these static details above the
action; typing and caret blinking remain independent. Headless Login
frames confirm the slot is identical with the password caret on and off.
Compared with the prior app golden, changes are confined to x 497–501,
y 635–655. The source-inventory gate still passes at 79% matched shape
area; that broad gate does not establish exact detail fidelity.

The mark's semantic meaning remains unknown. Default-action focus is a
possible interpretation, not an implemented state. Fine source edge
softness and scan variation are not recovered by this three-bar fit.
