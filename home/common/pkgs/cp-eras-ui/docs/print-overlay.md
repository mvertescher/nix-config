# Print overlay, 2026-11-13 (copies remeasured, and on for all eras, 2026-11-14)

Owner decision 6 (`docs/review-2026-11-13/DECISIONS.md`): one shared
print-texture pass per era instead of element-by-element echo and grain
fitting. This records what the reference surface measures as, what the
prototype draws, and its limits. Code: `src/print.rs` (era-side data and
CPU mirror), `src/eras/{neomil,entropism}.rs` `PRINT`,
`vendor/iced_wgpu/src/print.rs` + `shader/print.wgsl` (the pass;
`vendor/iced_wgpu/PATCHES.md`). On by default for every era since
2026-11-14 (Matt: "On for all eras"); see "On by default" below.

## What the references show

All eight references are 3840x2160 lossless exports, not photographs:
the dashboard's flat red tiles are exactly `#ef3333` with zero variance.
The "printed" look is designed, and it is not uniform across the screen.
Measured on the native files (design px = native / 2.4):

| Effect | Neomil | Entropism |
|---|---|---|
| Faint copies of bright ink | everywhere; displaced **outward from the screen centre** in proportion to distance: a whole-frame scale of 0.50-0.55 % (dashboard, store), 0.8 % (login), second peak near 2x (copies at k and 2k; matches the earlier per-element dashboard fits, e.g. JHN codes -3.6/+3.6 dp at x 45/1548) | same geometry, correlation peak 0.6 % on all four screens; visible ghosts on isolated stems sit farther (~0.8-1 %) |
| Rows of the copies | modulated at 4.764 native = 1.985 dp, depth ~55-60 % (amplitude/mean of the echo residual) | banded at ~8 native = 3.3 dp, depth ~20-30 %; plus a dot/moire structure not modelled |
| Fill scanlines | **not global**: clear ground 0.01 levels, dashboard tiles 0, mail selected row 0; present only on some fills (login active card +-3.5 on 246, store VIDEO +-6.5 on 197, GO HOME panel +-3.5 on 32), already drawn per element for the panel and badges | none on fills or ground |
| Grain | ~1.1-1.7 levels per channel on dark ground, ~0 on bright fills | sage fills RGB std 4.4/6.0/9.0 on 167/212/171 (~2.3 % of Y, chromatic, correlation ~1-2 native px); dark ground 0.7 |
| Softness / bloom | copies soft (~1-2 native px) | copies soft; clear bloom around light text and fills |

Vertical copy offsets are ambiguous in every fit (the copies' scanlines
comb the correlation at the 4.76 px pitch), so the scale is isotropic.

## Registration copies, measured 2026-11-14

This section supersedes the copy geometry, blend and row figures above and
in "The pass" below; the grain and fill-scanline figures stand.

The owner's reading of the echo was "2 or 3 separate ghostings, and then
they overlap, and that increases the intensity". Measured on the native
references: **two** separate copies, not three; and where they overlap
the result is brighter than either copy alone but less than the sum --
ordinary alpha compositing, `A = 1 - (1 - a1)(1 - a2)`. Neither "lighten"
(max, no gain on overlap) nor "add" fits as well.

### Method

A forward model per patch, fitted by least squares on the non-ink pixels:
the crisp ink mask `M` (the ink channel above a threshold, with its
anti-aliased edge), shifted by `d_i` and Gaussian-blurred, at gain `g_i`,
combined by over / add / max; a free per-row gain absorbs the row texture.
For the glowing eras a centred blur term (the bloom) is added. Patches:
13 hand-picked dashboard sites, plus an automatic tiler (220x110 native
tiles with 1-22 % ink on dark ground) over all four Neomil screens. The
geometry is then a line fit of `d` against position per axis. Tools are in
the session scratch (`gfit.py`, `auto.py`, `bfit.py`, `rule_reg.py`), not
the repository. The same tiler run on our own 4K mailbox render recovers
our parameters (copy 1 0.595 % / 0.99 %, copy 2 1.22 % across, gains 0.30
/ 0.14 against the reference's 0.30 / 0.15, over better than max on 36 of
44 sites), which checks the method end to end.

### Neomil

- **Count.** Two. On the dashboard's VEHICLES label one copy leaves rms
  0.030, two 0.013; a third takes no weight (gain -0.01..0.003). The header
  rule on the mailbox and login shows them cleanly apart: copies at -6.6 and
  -12.7 native rows (mailbox), -7 and -13 (login), nothing beyond.
- **Geometry.** Each copy is the whole frame scaled about a point near the
  screen centre; residuals 0.1-0.2 native px across the dashboard. The
  registration differs per reference:

  | screen | copy 1 kx / ky | copy 2 kx / ky | centre (native) | red gain 1 / 2 | sites |
  |---|---|---|---|---|---|
  | dashboard | 0.50 % / 0.505 % | 0.92 % / 0.93 % | 1935, 1160 | 0.46 / 0.30 | 13 |
  | mailbox | 0.60 % / 1.00 % | 1.22 % / 1.9 % (rule; tiler 1.73 %) | 1920, 1117 | 0.30 / 0.15 | 37 |
  | store | 0.54 % / 1.07 % | 1.45 % / 2.25 % | 1920, 1091 | 0.41 / 0.23 | 32 |
  | login | 0.59 % / 1.05 % | 1.3 % / 1.95 % | (mailbox's) | 0.34 / 0.23 | 8 + rule |

  So only the dashboard is isotropic and only there is copy 2 near 2x copy
  1 (1.85x); elsewhere the copies are stretched about twice as much
  vertically, and copy 2 sits at 1.8-2.7x. The login fit is the weakest
  (its ink mostly sits on the red cards, which hide the copies).
- **Blend.** Normal alpha. Dashboard marker boxes, where the two copies of
  a 30 px box overlap over most of their width: rms 0.0074 over, 0.0141
  add, 0.0181 max (box 1); 0.0071 / 0.0116 / 0.0128 (box 2). Over beats max
  on 34/37 mailbox and 31/32 store tiles. Over a coloured ground the copies
  lower the channels the ink lacks: per-row regressions across the
  dashboard rule's length (r^2 = 1.00) give one slope for all three channels,
  `out = (1 - a) bg + a C`, with green and blue pulled *below* the blue
  ground -- not lighten, not screen, not add.
- **Copy and original.** The copies lie under the art: ink pixels are
  exactly the ink (251, 53, 53) wherever a copy falls on them, and the flat
  tiles (#ef3333) have zero variance. Exception seen and not modelled: dark
  text on the mailbox's selected red row carries red row stripes, i.e.
  copies of the fill do show over that dark ink (the fill gate suppresses
  this).
- **Blur.** Gaussian sigma 1.05-1.14 native px (0.45 dp).
- **Texture.** Rows of 4.764 native px (1.985 dp): the period is 4.7640 and
  the phase the same (red crest on native row 4.68 + n 4.764) on all four
  screens and in both halves of each. It colours the copies rather than
  modulating their opacity: over black the copies' red swings 0.72-1.27 of
  its mean while green and blue swing 0.2-2.2 in antiphase; the rule
  regressions give the colour itself, about (240, 40, 40) to (215, 15, 15)
  near the red crest and (140-150, 50-56, 50-56) at the trough. Modelled as
  a factor on the copied colour, crest (1.0, 0.30, 0.30), trough (0.58,
  0.98, 0.98); the per-row colours also carry a second harmonic in green
  that the model does not have.

### The other eras (bloom-aware fit, fewer sites)

| era | sites | copy 1 kx / ky | copy 2 kx / ky | gain 1 / 2 | blur sigma |
|---|---|---|---|---|---|
| Entropism (store file `entropism-dashboard.png`) | 4 + 19 | 0.61 % / 0.68 % | 1.52 % / 1.60 % | 0.36 / 0.14 | 1.5-2 px |
| Kitsch | 7 | 0.60 % / 0.65 % | 1.58 % / ~1.8 % (0.6-2.9) | 0.28 / 0.08 | ~1.6 px |
| Neo-kitsch | 9 | 0.60 % / 0.60 % | 1.57 % / ~1.5 % | 0.21 / 0.12 | ~1.7 px |

Two copies in each, the second at about 2.5x the first, about isotropic,
over better than max on nearly every site. A third copy near 3.5x took
0.1-0.2 on some sites but lowered the residual only 2-4 %: possible, not
established, not drawn. Gains scatter widely between sites (e.g. Entropism
0.19-0.60): the copies are broken up by a diagonal circuit-like pattern
that also lies on the ground, and the text's bloom (a centred blur, sigma
about 5-8 native px, gain 0.3-1.9) overlaps them. Neither is drawn.

### The pass now

Up to three copies, each `Echo { kx, ky, gain }`, scaled about
`echo_centre`; blurred (4-tap box, `echo_soft` = sigma in dp); coloured by
`Texture { pitch, angle, crest, trough, phase }`; composited farthest
first by `echo_blend` (`Over`, or the old `Lighten`); gated by
`echo_lift` (a copy shows only where it is lighter than the pixel under
it -- what puts it under the art) and by the fill gate. `print::of(era)` is
the era's treatment for a new screen (Neomil: the mailbox fit);
`print::of_screen(era, Screen)` returns the per-screen fit, which the four
reference binaries wear. `print::DEFAULT_STRENGTH` is 1: the fit is worn
by default (below).

Overrides: `CP_ERAS_UI_PRINT_SET="echo1=kx,ky,gain;echo2=...;echo_centre=x,y;echo_blend=over|lighten;echo_soft=s;echo_lift=lo,hi;tex_pitch=p;tex_angle=a;tex_crest=r,g,b;tex_trough=r,g,b;tex_phase=f;..."`.

### On by default, 2026-11-14

Who wears it:

- `cp-eras-ui-login`, `-mailbox`, `-store`, `-dashboard`: their screen's
  fit (`print::wear_screen`). The dashboard binary's hub re-wears on each
  route, so the mailbox and the store reached from it wear their own fits
  (`screens::hub`, `Hub::wear`), as their own binaries do.
- `cp-eras-ui-mail` (the working mail client): the era's treatment for a
  new screen, `print::of` (Neomil: the mailbox fit).
- Not the status bar (`cp-eras-ui-bar`, `-bar-window`): a live desktop
  surface designed separately, with its own rule echo and the
  `PanelEcho` menu motifs. Its render is unchanged byte for byte.
- `CP_ERAS_UI_PRINT=0` turns the pass off in any of them.

### Per-element copies removed (they doubled the pass)

With the pass on, these era-drawn copies were drawn as well as the pass's
own (Neomil dashboard marker boxes showed four copies instead of two).
Each was checked at 4K against the reference with the pass on; all are
radial registration copies the pass reproduces, so all were deleted:

- Neomil dashboard: `neomil_dashboard_{margin,header,tape_chip,label,brand}_echoes.rs`,
  `neomil_dashboard_echoes.rs` (`PANEL_ECHOES`),
  `neomil_dashboard_panel_printing.rs` (`PRINTING_ECHOES`, the whole file),
  `HEADING_ECHOES` in `..._panel_primary.rs`, and the footer
  `DASHBOARD_FOOTER_{FRAME,TEXT}_ECHO_{1,2}`. The dashboard now has four
  software layers instead of eight.
- Neomil login: `neomil_login_echoes.rs` (`HEADER_ECHOES`) and the notice
  copy (`NoteEcho`, `Slot::reference_note_echo`, `src/screens/login/echo.rs`).
  Its offset (-2.5, +2.5) dp was exactly copy 1 of `PRINT_LOGIN` at the
  notes' position.
- Neomil store: `neomil/store_echo.rs` (scatter-code trails: "directional"
  only in that they point away from the centre), `store_title_echo.rs`,
  `store_frame_echo.rs`, and the nav rows' `NAV_ECHO` (measured -8.8/-3.2
  and -22.7/-7.5 native px; `PRINT_STORE` puts copies at about -8.4/-5.2
  and -22.5/-11 there: the same across, a few px more up). With them gone
  the store's feedback states equal `STORE_STATES` again.

What looks different from the hand-drawn copies, by eye at 4K:

- Store scatter-code cells: the reference's trails are a fairly continuous
  smear; the pass gives two discrete, fainter steps of the same length.
  The hand-drawn trail matched the intensity better.
- Login notices: the pass's copies are fainter than the reference's (the
  hand-drawn single copy at 0.30 was closer in strength). The login fit is
  the weakest (8 sites).
- GO HOME vertical brands: the pass draws copies to their right that the
  reference barely shows (the hand-drawn ones were similar).
- Gained: copies the hand drawing never had and the reference does (store
  nav labels AUDIO / GAMEPLAY, login PROTOCOL glyph and tape, HAND GUN).

Kept, as designed motifs rather than registration copies: the bar's
`PanelEcho` (`EdgeBar`, `Rings`, `Wave`; the bar does not wear the pass),
and the mail `MailRowEcho` hover / pressed ghosts (Kitsch: one copy lifted
20 dp up-right; Neo-kitsch: seven nested rings), which are feedback-state
drawings at offsets the pass does not produce.

Evidence (native 4K crops, 4-10x zooms, profiles, model diagram) was
written outside the repository; no reference pixels are committed.

## The pass (2026-11-13 prototype; superseded where the section above says)

Per pixel, in encoded sRGB: two copies of the frame scaled about the
centre by `1+2k` then `1+k`, each a 4-tap box (softness), lighten-composited
at alpha x row cosine x a fill gate (encoded luminance on a 6 dp ring: a
copy is suppressed inside light fills so dark ink stays firm); then an
optional darkening scanline multiply; then bilinear value-noise grain,
multiplicative plus additive. Pitches are in design px converted per axis,
snapped to whole device pixels below 3 px (2 px at 1600x900: no moire).
Zero strengths skip the pass and the frame is byte-identical.

Fitted values are the `PRINT` constants. Neomil: k 0.0055, alpha
0.24/0.14, copy rows depth 0.6 at 1.985 dp, soft 1.5 dp, no fill
scanlines, grain 1 level. Entropism: k 0.008, alpha 0.32/0.18, depth 0.3 at
3.33 dp, soft 2 dp, grain 2.8 % + 0.75 levels at 0.8 dp cells.

Tuning knobs (read once at start): `CP_ERAS_UI_PRINT=<strength>`,
`CP_ERAS_UI_PRINT_SET="key=v;key=a,b"` (`Print::with_overrides`).

## Limits

- No bloom (needs a separable blur, i.e. more passes). Entropism text
  regions stay darker than the reference (mail body mean G 38 vs 46).
- Non-Neomil eras: the copies are crisper and a little more solid than
  the reference's, which are broken by a diagonal pattern and wrapped in
  bloom the pass does not draw; light text over a mid fill (Kitsch
  dashboard tiles) shows a copy the reference hides. Gains left at the fit.
- White ink under Neomil's red texture (the mail client's title tab) gets
  pale cyan-striped copies; the texture was fitted on red ink only.
- Entropism mean colour rises 1-3.6 levels (G) against a reference ~1-2
  above the goldens: its frame strokes were already brighter than the art.
- Lavapipe cost of the pass alone (timed 2026-11-14 with a throwaway
  wgpu test on llvmpipe, a 32-thread machine under heavy unrelated load,
  best of 8): Neomil 68 ms / Entropism 98 ms at 1600x900, 347 / 422 ms at
  3840x2160, against 4 / 8 ms for a bare clear pass. The 2026-11-13 figure
  was +~45 / +~350 ms. Real-GPU cost unmeasured.
- `Renderer::screenshot` and the tiny-skia backend do not run the pass.
