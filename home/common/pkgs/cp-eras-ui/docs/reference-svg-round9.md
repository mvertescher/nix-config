# Ninth reference checkpoint — 2026-09-29

This checkpoint resumes the L/M work committed as WIP in `e04d72d` and
integrates the N corrections. The recovered baseline had six failed visual
cases. Reviewed changes have explicit source/SVG/Iced evidence; a stale
golden is not itself evidence that a changed picture is correct.

The immutable N source passes 269 Rust tests. All 24 reference, component and bar SVGs
parse with unique IDs. Ten screenshot baselines are
refreshed from reviewed captures, including the unchanged Neomil fallback
route. All 22 repository checks pass. All 27 visual cases match 100.000% on
the first attempt, including frozen/live store CJK printing. No comparison
thresholds have changed, and the new work remains staged/uncommitted.

Accepted local corrections include:

- Entropism dashboard body baselines, store stat height and socket placement,
  and individually fitted mailbox action widths. All four action endpoints
  agree with the native source within one pixel. Original font contours and
  material modulation remain separate.
- Kitsch mailbox selected sender weight follows the selected message,
  including first, middle and last rows. The notice's native ink areas are
  2334/2444 pixels against source 2345/2450; its former heavy/tall rendering
  is corrected. Store certification has the source's full-height inset
  three-bar mark, with tiny lettering and microprint still approximate.
- Neo-kitsch store sockets/BASKET use the measured 25-cell scatter in both
  SVG and Iced. The three inner upper frame turns follow source trajectories
  within about 1–2 native pixels. Held fourth-card clipping, full stems and
  lower joins retain continuity. Exact veneer flow, frame ink and historical
  lower-corner differences are not closed by this local correction.
- Neomil mailbox material and outline share the corrected upper contour.
  Native, fractional and 0.35-second opening crops show a connected chamfer
  and aligned fill/edge. Tighter cartridge ribs improve primary detail;
  normal-terminal materials and photographic echoes remain open.
- Neomil store primary certification/KIROSHI marks are source-supported
  vectors, and margin `益荒男` no longer depends on fallback fonts. Navigation
  and boxed footer type have fitted widths and weights. Native baseline
  differences of 1–2 pixels are assigned to the next batch. The reference
  footer interior now exposes the ground: three native patch medians match
  the SVG exactly and differ from source by only a few RGB levels, replacing
  the old opaque red field. A custom-palette capture retains its semantic
  fill. Unboxed footer type and repeated faint printing remain open.
- Dashboard maker microtext has trailing copies and softened edges. Native
  source RGB RMS falls 21.19→12.18 and 23.02→12.82, with all changed pixels
  inside the two runs; the early opening frame is unchanged. Fine scan
  striations are not reconstructed by this fit.

L's 14 affected fidelity gates and M's two source/three implementation gates
passed. N's 12 paired gates pass and all seven fractional
rest/held/selected/custom captures are reviewed. Shape inventory gates cannot establish tiny glyph or
photographic material fidelity; the local comparisons above control the
scoped acceptance. Live desktop/session verification remains separate.
