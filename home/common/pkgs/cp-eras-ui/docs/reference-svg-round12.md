# Twelfth reference checkpoint — AA, 2026-09-29

AA restores Neomil store directional socket printing and moves eligible
post-quiet dashboard resize preparation off the UI thread. All 285 Rust
tests and the release build pass. The 24 SVGs parse with unique IDs;
both affected store gates pass (72% source area, 89% implementation area).
All 22 repository checks pass, including 27 visual cases at 100.000% on
their first attempt. All 197 frozen source/script/documentation/test hashes
match the 15 MB Nix source snapshot. Only the reviewed Neomil store golden
changes in AA. Changes remain staged/uncommitted.

## Store socket printing

The source's ordinary card-1 tails run left, cards 3/4 run right and the
selected card's tails run down. Shared source-fitted gradients restore
those directions and local scan bands inside the existing shelf material
image. All 25 primary cells, their 3×3 size and 3.6667-unit pitch remain
unchanged. Continuous SVG gradients avoid half-pixel strip seams; the
fourth ordinary card retains an independently measured vertical phase.

On a fixed SVG-derived footprint, native source RGB error falls
19.76→11.92, 26.46→11.95, 20.09→12.51 and 23.34→14.47 across the four
cards. Independent cell/row checks improve. Every bright primary cell
is unchanged. All 19,165 changed 4K pixels and all 3,555 changed 1600×900
pixels are confined to the four socket clusters. Some ordinary dark-gap
errors increase by 1–2.4 RGB levels; regular gradients do not reconstruct
the source's exact irregular grain.

Sixteen native captures cover production, fractional rest, ordinary and
selected hover/held states, all selected slots, the fourth-card cutoff,
custom palettes and opening at .15/.35 seconds. Comparisons with the
previous matched states change only socket clusters; the custom rest
capture is identical. The early opening preserves its shared material/
foreground clip. Feedback redraws semantic echo ink after its opaque coat
and before the unchanged cells. Source-only state selection shares the
same palette and category/card eligibility as the source backdrop.

An alternating local CPU geometry probe compares the same foreground
with and without the extra feedback primitives, excluding its first pair.
At 1537×947, ordinary held median is 79.32→84.59 ms, selected held is
25.40→25.52 ms; the unchanged custom control is 27.964→27.972 ms. There
are eight samples per path. Ordinary feedback adds 2,400 small rectangle
fills; selected feedback adds 400. These are geometry construction times,
not presentation latency or FPS. Idle printing adds no Soft image layer.

## Asynchronous resize preparation

One worker renders a job-local cache; one latest pending target replaces
older requests. Owner, generation, palette, display-list context, exact
scale and actual canvas bounds prevent stale publication. Publication of
a complete distinct-key set is atomic. Exact snapshots clone all ordered
handles under one lock and paint at source==target, avoiding a subsequent
synchronous lookup race. A dropped/changed route invalidates only its own
job. Worker failure takes a defined exact fallback and stops polling.

The existing Neomil rest/unheld/nonfrozen opt-in, minimum 2560×1440 size,
10% preview bound and 200 ms quiet timer remain. Missing/invalid previews
retain the exact fallback. The shared cache still retains at most 384 MiB;
one temporary job-local cache can additionally retain up to 384 MiB.
Those bounds exclude compositor scratch, renderer copies and process RSS.

A headless compositor sequence without an application heartbeat exercises
bounds arriving before resize events, expiry during preparation and stale
completion. Its obsolete 3600×2025 job takes 527.443 ms and publishes no
entries; the desired 3733×2100 job takes 617.480 ms and publishes a complete
80,632,800-byte set. Shared payload is 167,078,880 bytes. The exact redraw
occurs before the delayed screenshot signal, proving that the canvas's
completion poll is sufficient. Returning to cached 3840×2160 needs no new
final job. A final instrumentation pass times the entire backdrop callback, including
key construction, cache locking and snapshot acquisition. Its 21 preview
draws take .015–.033 ms; no eligible resize draw invokes synchronous
preparation. That pass publishes intermediate/final jobs in 663.308 and
715.813 ms and retains 245,389,280 bytes. Its final image differs from
the earlier same-history exact control at three red-channel edge pixels
by one level; the two footer-history pixels agree. It samples
1,351,052 kB RSS and 2,546,816 kB peak RSS. The earlier .012–.017 ms
figures measured image submission only and are not full callback timings.
These repeated tiny edge differences remain an explicit sampling limit.

The CPU compositor itself is not faster in these runs: the change
keeps eligible image preparation off the UI thread.

The first sequence did not receive its last compositor configure and was
rejected by the harness. Reasserting the final size completed the run.
The 4K return matches its exact control at every pixel. At 3733×2100,
a same-resize-history exact control also matches byte-for-byte. A fresh
exact-size launch differs at five pixels (three by one red level and two
footer pixels by at most ten); the same-history control reproduces all
five, separating them from the new preparation path.

The fractional run samples 1,207,636 kB process RSS and 2,092,280 kB peak
RSS; the 4K-return run samples 905,344 / 1,153,000 kB. These include software
rendering, allocation and screenshot work. They do not establish hardware
presentation, input latency, smooth dragging, or a process-memory bound.
The previously reviewed bounded preview still uses separate original image
layers; exact final pixels do not use the scaled preview as a golden.

## Research results and next correction

Uniform mailbox rib opacity and a shared softness filter fail independent
ridge/gap controls and are rejected. A Neo-kitsch fifth-frame opacity
change improves one ridge but worsens neighboring gaps and upper shoulders;
it is rejected too. No material was changed for those trials.

The small Neomil MASURAO margin word is a distinct primary defect: its
font word is misplaced over accepted CJK and a long rule replaces a short
slash. A simple position/scale trial and an initial skeleton-vector trial
do not reproduce the source contours. A separate source-contour correction
is under review; it is not included in AA. Exact fine printing, unknown
assets, design-dependent interactions and live desktop checks remain open.
