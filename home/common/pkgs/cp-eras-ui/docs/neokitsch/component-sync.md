# Neo-kitsch component source sync (NK-12)

`dashboard-trace.svg` is the source schematic for the hub specimens in
`components.svg`. The photographed master is `images/neokitsch-dashboard.png`
(run screen #69). The sheet translates these excerpts without scaling:

| Specimen | Trace geometry and content | Sheet placement |
| --- | --- | --- |
| EMAIL selection | `#ncardsel` at (246,384), 42 clipped grain paths at 2.1px pitch, plate at (244.75,442.3) | `translate(-130 67)` |
| Cascade caption | `#ncaption`: five Rajdhani 7.5 lines on a 6.77px pitch, with the source's CC35/DHSF wording | (250.7,731.25) and (351.2,630.8) inside the same card group |
| Detail panel | `#npanel` at (1170.8,259.7), four inner rings; body (1170.8,326) 230.4×309, base `#fcbe6d`, 85 clipped grain paths at 2.7px pitch | `translate(-259.8 70.3)` |
| Panel printing | Source #69's literal six-plus-two lines, Rajdhani 16.3; two CC35/DHSF tape lines, Rajdhani 7.5 at baselines 645.8/652.1; EMAIL label | same panel group |

The six-plus-two body copy was already current before NK-12. It was retained
exactly, including `maece-` and the blank line slot. The replaced examples
were the flat EMAIL and panel fills, five caption bars and two tape bars.
The new grain and caption groups match their current trace XML exactly.

The bar is an **original design target** with no photographed bar source.
Its clipped ground copies the *dashboard* haze parameters (825,−120),
radius 1030, vertical scale 0.515 and stops 0/.258/.572/.873/1, plus the
blue annulus. Store, mailbox and login use a separate shared ground fit.
Their disagreement with dashboard remains NK-11; no bar fidelity score is
implied. The ENTER / LOGIN source CTA uses 4.1px tracking at 14px. The
bar's `SYNC` tracking is an original 2px design choice.

The sheet's rest specimens are traced; band 11's hover and field focus
remain inferred interaction conventions. Current screen rendering uses
screen-specific drawing tables, including the dedicated `Mailbox` layout. `Chrome::DeviceFrame`, `Corner::ClipTopRight`
and `Ground::Bloom` remain generic or bar-example fallbacks, not evidence
of a full frame, universal corner or single bloom on the sourced screens.
The selected mailbox title, sender and envelope now use the SVG's
distinct reference inks in the runtime. The native color comparison and
custom-palette behavior are recorded in [mailbox-inks.md](mailbox-inks.md).
Source typography refinement (NK-05), photographed veneer material (NK-07)
and ground fit (NK-11) remain open.

Verification on 2026-09-27: rendered the 1920×1560 sheet before and after
with `rsvg-convert` and the audit font config; inspected the card and panel
crops. The rendered band 11 is pixel-identical before and after. XML checks
found 53 unique IDs, no unresolved local references, and exact agreement
with the dashboard trace for `#ncaption`, its two use origins,
`#panel-copy`, the two tape lines and both clipped grain groups (42/85
paths). `bar.svg` changes only comments.
