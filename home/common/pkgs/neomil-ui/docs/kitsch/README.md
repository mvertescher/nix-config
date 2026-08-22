# Kitsch design targets

Proposals, not sampled. The neomil targets next door were derived from
the Behance reference images; these are drawn from the era's description
— maximalist, gilded, glossy, ornament for its own sake — and should be
checked against reference art before anything is built from them.

- `target-components.svg` — the widget sheet, and the two test cases the
  crate-boundary decision turns on.
- `target-app.svg` — "MAISON", a salon booking screen composed from
  those widgets. Deliberately carries the *same information* as a neomil
  screen (list, detail panel, actions, status strip) so the two can be
  compared like for like: everything that differs is decoration.

## What the component sheet is arguing

It exists to answer one question before code is written: **is
era-difference expressible as parameters, or is it a codebase?**

**Test 1 — silhouette.** The same chip drawn three ways: entropism
square, neomil chamfered, kitsch scalloped. This is one number in a path
builder — `chip.rs` already has `cut` as a local. A `Silhouette`
parameter covers all three, and no crate split is needed for this axis.

**Test 2 — ornament.** A gilded panel with four filigree corners, three
nested strokes and a damask fill. This is *not* an outline; it is extra
geometry laid over the base, in counts and positions the base knows
nothing about. A trait shaped as "give me the outline path" cannot carry
it.

Neither test says "fourth crate". Together they say: one toolkit, a
silhouette parameter plus a decoration layer, and per-era widget modules
where the interaction model genuinely differs (a radial dial versus
neomil's `diamond_menu`, which is 511 lines of mostly hit-testing).

**The open risk**, stated on the sheet: if the decoration layer starts
needing to know widget internals — where the label sits, how tall a row
is — the abstraction is wrong and the conclusion should be revisited.

## Roles

Kitsch needs more than the shared seven. The sheet proposes `gild` (a
gradient, not a colour), `sheen` and a second accent, which is what
`roles.nix`'s `toBase16 { accents = ...; }` argument exists for.

```
bg #17071a  panel #2a0f2e  border #d4a13c  dim #a2739e
fg #ffd2ee  alert #ff2f6d  tape #ffd75e
gild (gradient)  sheen #fff3fb  accent2 #45e0d2
```

## Rendering

```sh
nix shell nixpkgs#librsvg --command \
  rsvg-convert -w 1600 target-components.svg -o /tmp/kitsch-components.png
```

Rajdhani is expected on fontconfig; without it the sheet still renders,
in a fallback face.
