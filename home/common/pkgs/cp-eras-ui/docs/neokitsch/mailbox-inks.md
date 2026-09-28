# Selected mailbox printing — NK-05

The selected bar in `images/neokitsch-mail.png` is a 3840×2160 photograph
of the 1600×900 design (2.4 source pixels per design pixel). It prints the
row-2 subject, FROM line and closed envelope in three similar, distinct
browns over the wood veneer. The previous Iced mailbox used the general
`on_select` role (`#3a2410`) for all three. The traced SVG instead uses
subject `#7b5438`, sender `#895f3b`, envelope `#865c39`; those are now the
reference mailbox's three runtime inks.

The photograph supports the distinction, though it does not support exact
flat swatches. In corresponding glyph boxes at native resolution, pixels
below R185/G145 have these RGB quantiles. The SVG was rendered at native
3840×2160 with the audit font config and librsvg. The source has soft glow,
dark grain and photographic grading, so its lower quantiles are darker
than a flat SVG core; the SVG's mid-brown values remain a reasonable,
source-supported transcription rather than measured exact pixels.
The pre-change Iced capture's P25 cores were (58,36,16) for the subject,
(60,38,17) for the sender, and (58,36,16) for the envelope, showing the
shared `on_select` role on all three marks.

| Mark | Native photo P25 | Native SVG flat core | Reference ink |
| --- | --- | --- | --- |
| Subject | (106, 70, 48) | (123, 84, 56) | `#7b5438` |
| FROM line | (114, 75, 50) | (137, 95, 59) | `#895f3b` |
| Closed envelope | (117, 81, 51) | (134, 92, 57) | `#865c39` |

The sampled boxes are subject (193,318)–(360,341), sender
(193,344)–(290,361), and envelope (428,332)–(450,352) in design
coordinates, all multiplied by 2.4 for the native analysis. The native
photograph remains darker and textured; exact photographic color and
veneer grain belong to material refinement, not this ink-role fix.

The final 3840×2160 Iced capture has P25 dark cores (123,84,56),
(137,95,59), and (134,92,57) in those same boxes: exactly the three
configured flat inks and a clear change from the previous shared
`on_select` core. Comparing full before/after captures finds 7,067
changed native pixels, confined to design bounds
(193.75,322.5)–(445.83,356.67), the selected row's printing. This
checks runtime reach and edit scope; it does not imply a photographic
match. The native photo's darker cores and soft edges remain visible.

Runtime selection uses the three values on the row currently selected,
including another row while held, because that transient state borrows
the same veneer. Selected hover keeps the resting material. Explicit
transient printing and sender overrides take precedence. A sender below a
shorter selected fill still uses `Ink::Select`, as kitsch requires.
Unselected rows and the NeoKitsch selected row's words, glyph forms,
read/unread choices, placements, tab and veneer are unchanged.

The fixed colors apply to the standalone and published `reference`
palettes. The published theme derives panel `#16161f` where the crate's
standalone palette has `#34344c`; both are accepted by exact palette
comparison. Other variants and direct palette edits fall back to the
semantic `on_select` role, preserving custom colors.
