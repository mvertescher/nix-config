# Reference correction batch twenty-five — AN

AN adds the complete AM ordinary Neomil store-title impression to the
trace, component excerpt and native display lists. The full displaced
titles retain their counters and use measured scan clips before the
registered primary text. Selected titles are unchanged; ordinary and
hover titles use semantic foreground ink, while held titles use existing
feedback ink. Six socket-material insertion offsets move with the new
title entry; selected and outer fourth-card indices stay in place.

Fixed 4K native title-crop RGB MAE improves 13.876→12.124,
12.074→10.076 and 12.932→10.135 on cards 1/3/4. All 11,353 changed pixels
are inside ordinary titles, with no changed opaque primary pixels. Local
fringe, counter and true-gap losses remain explicit; exact phase, softness
and lower repetitions are still open. The model followed source comparisons
and is not a blind validation. See the masks and losses in
[store printing](neomil/store-printing.md#an-native-title-impressions).

Kitsch's four small framed certification marks gain the deeper, chamfered
lower opening seen in the source. The retained `RG5` text moves upward and
uses wider, fainter printing; two lower strokes restore visible detail.
The photograph does not establish those letters' meaning. Native opacity
is calibrated for the canvas's dark-ground alpha rebasing, and the inner
opening retains the selected palette role. Whole-mark native RGB MAE
improves 31.77→26.08, 37.06→32.02, 31.27→25.39 and 35.26→29.57.
All 662 native and 1,074 SVG changed pixels stay inside the four marks.
The selected lower-rule crop retains a small loss, 10.18→11.27 native
and 5.55→6.38 SVG. Exact lettering, ink and photographic softness remain
open. See [certification controls](kitsch/store-art.md#an-certification-lower-frame-and-printing).

## Fractional clipping defect

The native title trial exposed an Iced 0.14.0 defect at smaller sizes.
Some scan clips become less than one physical pixel. The triangle renderer
uploads every mesh's indices, but when a clip snaps empty it advances only
the vertex and uniform offsets. Later meshes then read the wrong indices.
The unpatched 1600×900 candidate changes 33,578 pixels outside the titles,
corrupting otherwise unrelated artwork. The reviewed 4K frame alone did
not expose this failure.

A one-line fix advances the index offset in the skipped-mesh branch.
The pinned `iced_wgpu` crate is vendored with upstream provenance and its
MIT license; Cargo's path override makes development and Nix builds use
the same correction. Nix keeps its existing clean-source exclusions and
adds only the eleven required vendored shader files. No logical-coordinate
clip shortcut is used: physical transformation and pixel snapping remain
the backend's responsibility. No unrelated lockfile dependencies change.
The dependency-only Nix build also restores the real vendored crate after
Crane generates dummy application sources, since registry dependents need
its API. That hook references only the vendor directory, preserving the
dependency cache across application edits.

With the patch, the 1600 and fractional rest frames change 2,405 and
1,977 pixels respectively, all inside ordinary title crops. Selected
titles, opaque primary pixels and the right margin are identical to the
baseline. The 4K candidate is pixel-identical before and after the patch.
The small-size store golden exercises the formerly skipped-mesh failure.

State review caught another ordering failure: Iced places child clips below
the parent's directly drawn geometry. Opaque custom-card fills and feedback
coats therefore hid the new echoes. Only ordinary card backgrounds now use
an earlier separate draft. Its broad bounds inherit the existing shelf or
fourth-card clip, avoiding a different snap origin. Background geometry,
coats, selected artwork, array indices and the shared renderer stay intact.
Tests compare the wrapped background geometry and the title clips/opacity
through feedback. The corrected 4K rest frame is still pixel-identical.

## Performance

The 61 clipped full-title runs add CPU work. Twenty warmed 4K
`Scene.draw` samples of the final candidate measure median
20.734→26.280 ms, excluding backdrop, GPU and presentation. Benchmark
captures exactly match their normal frames. This is not an ongoing
frame-loop measurement. The initial candidate measured 20.669→25.315 ms.
A start-anchor width-shaping shortcut measures 20.631/25.357 ms and is
rejected because it does not materially reduce the cost. A separate
[bounded glyph-outline reuse task](../todo/performance.md#store-title-printing-cost--an)
requires exact pixels across sizes, palette roles, clips and feedback.

## Verification

All sixteen Neomil state pairs preserve pixels outside ordinary title
crops, selected titles, opaque primary ink and the fourth-card cutoff.
Every ordinary title also has a nonempty impression in each capture;
footprint checks alone had missed the hidden-echo failure. Cases include
1600 and fractional sizes, every selected slot, ordinary/selected hover
and held states, custom foreground/background roles and early opening.
These are synthetic drawing states, not live desktop event-routing tests.
Kitsch's eight state pairs change only the four certification marks:
225 pixels at 1600 and 212 in each fractional case. The backend correction
leaves all eight Kitsch outputs and the reviewed 4K frame identical.

All 288 Rust tests, 24 SVG parse/unique-ID checks and four unchanged-threshold
fidelity gates pass. Neomil matches 19/39 source shapes / 73% area and
23/34 implementation shapes / 89% area. Kitsch source ink placement is
.84, and the implementation matches 13/21 shapes / 91% area. These broad
gates do not prove fine printing fidelity. The final rest frames are
pixel-identical to the measured gate inputs. The two store goldens are
refreshed only from the reviewed corrected captures.

The full repository check passes all 22 checks and all 27 visual cases.
The first package build fails two login font-measurement tests, then its
identical retry passes; 26 matrix cases pass on their first attempt and
one after that build retry. This is not a clean first-attempt result.
One test registers a font while other tests measure text; the shared
[test-initialization fix](../todo/verification.md#concurrent-font-initialization--an)
is assigned to AO. Existing assertions remain intact.

Direct comparison finds 26 pixel-identical cases. The unchanged Neo-kitsch
bar retains its pre-existing single one-level pixel difference at (749,20).
Both refreshed store goldens are identical. All 291 frozen file hashes
match the tested Nix source; all four original reference images are
unchanged. Fresh packaged 4K Neomil and Kitsch captures exactly match
their reviewed previews. The subsequent TODO/checkpoint prose leaves
the tested runtime, SVG, script, test, vendor and golden content intact.

Evidence is under `/tmp/cp-eras-next/an/`: `check-verification.json`,
`package-parity.json`, `ordered-states-verification.json`,
`kitsch-patched-states-verification.json` and
`title-cpu-benchmark-final.json`. Prior staged work is preserved;
nothing is committed, pushed or deployed.
