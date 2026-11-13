# Decisions from the 2026-11-13 review

Matt's answers to the questions the review raised
([README](README.md)), given the same day. These are owner decisions:
they settle the design question and change only when he changes them.

1. **Neomil hub, keyboard focus.** A selected tile is drawn inverted: dark
   fill, red outline, red glyphs. It borrows the era's dark-on-red
   language. Accepted cost: the resting screen no longer matches the art
   exactly, since one tile is always selected.
2. **Neomil hub, dead tiles.** Wire the mailbox to one tile so every built
   screen is reachable. The other four stay inert until screens exist.
   Which tile was left to us; LOCATIONS was the suggestion.
3. **Neo-kitsch wood veneer.** Use a pre-rendered texture clipped to the
   shapes, not drawn vector grain. Accepted cost: an image asset, and
   less clean scaling at odd sizes.
4. **Ink on filled surfaces.** Match the reference's firm dark ink, in
   every era that shows the pale-text symptom. First find out whether one
   shared cause explains all three eras.
5. **Kitsch label typeface.** Switch labels and small print to an
   open-licence wide face close to Eurostile Extended (Michroma is the
   candidate), rather than wait for the original to be identified.
6. **Print texture.** Stop fitting echoes and grain element by element.
   One shared overlay pass per era (scanlines, grain, slight
   misregistration) applied to the whole screen and tuned by eye against
   the reference. It should be able to animate later.
7. **Order of work.** Ink, then Kitsch type, then the Neomil hub (focus
   look and mailbox route), then textures and the veneer.

Decision 6 changes the standing of the open element-by-element texture
items in `todo/` (scan modulation, printing echoes, edge softness,
background material): they are superseded by the shared pass, not
abandoned as unimportant. They are not edited yet; the work list is
[todo/review-2026-11-13.md](../../todo/review-2026-11-13.md).
