# Round twenty-three: source audits and rejected trials

AL uses three scoped Sol lanes with Astra source/native review. No
production geometry, renderer, reference SVG, test or golden changes are
accepted. The earlier staged work remains intact. The audits identify
concrete missing title printing and a cartridge-depth defect, and narrow
the evidence for unresolved veneer joins. These findings remain open;
an improved average image score alone does not close them.

## Neomil store titles

The source's ordinary `MAGNUM 650` titles carry repeated glyph printing
that is absent from SVG and Iced. A full shifted copy improves average
RGB error but adds ink in source-dark letter counters. A three-row upper
band preserves the primary letters and improves all three ordinary-card
crop averages, yet worsens 95/27/49 dark-gap pixels on cards 1/3/4 and
leaves isolated schematic bars. Both trials are rejected. The next fit
must recover interrupted repeated contours while checking counters and
true gaps. Cards 3/4 were inspected during exploration, so are not blind
holdouts. See [the title-printing audit](neomil/store-printing.md#al-missing-ordinary-title-printing).

## Neomil mailbox terminals

Fresh native captures confirm that a common background adjustment is
unsupported: clear ground above four ordinary terminals already matches
closely. An inner-fill/opacity trial improves full-strip averages but
worsens a rib core and makes a lower-row gap too bright. It is rejected.

The subsequent geometry audit finds a deeper source terminal boundary
across four rows. Extending ribs while retaining the old inner edge makes
a doubled ladder. Relocating that edge removes the central divider but
the tested right join crosses the notch and leaves gaps too dark. Neither
geometry trial is accepted. The deeper boundary and joins are a concrete
open task, requiring coupled geometry/material review and native state
checks. See [the terminal audit](neomil/mailbox-fidelity.md#al-terminal-depth-and-inner-boundary).

## Neo-kitsch veneer

The upper EMAIL ridge can plausibly continue toward native x694 rather
than AE's proposed x685 join. One scratch path improves entry, middle and
merge RGB comparisons. Its endpoint remains ambiguous, crossing losses
remain, and ridge counts do not improve. Adjacent and lower controls are
unchanged. The trial is retained as evidence, not ported. A broader source
map distinguishes visible segments from joins obscured by overlapping
ridges, text or clipping. See [the topology audit](neokitsch/veneer-fit.md#al-visible-continuations-and-unresolved-joins).

## Verification and retained state

Astra reruns the worker measurements, checks the actual source/SVG/native
crops and verifies changed-pixel containment. Fresh 4K mailbox and
Neo-kitsch dashboard baselines come from the verified AK package; the
store uses its matched AK production capture. Source hashes and binary/
capture provenance are frozen under `/tmp/cp-eras-next/al/`. The original
three reference images are unchanged. No candidate native build is made
after the SVG/source controls reject these trials.

Only Markdown findings and TODOs change in AL. Runtime, SVGs, scripts,
tests and goldens retain the verified AK content. No new full build or
golden run is claimed: AK remains the last implementation checkpoint,
with 288 Rust tests, six affected gates, 22 repository checks and 27
first-attempt matrix passes. Its recorded one-level bar pixel remains
separate from rounded 100.000% reports. See [round twenty-two](reference-svg-round22.md).

Scratch evidence is in `/tmp/cp-eras-next/al-store-printing/`,
`al-mail-terminals/` and `al-veneer-topology/`. The prepared native mailbox
harness is uncompiled scratch material, not a validation result. Rust
port preparation stopped before writing a candidate. Live desktop/session checks
still require their recorded prerequisites. Nothing is committed, pushed
or deployed.
