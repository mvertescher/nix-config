# Round 85: working-mail DELETE contrast

Entropism's working mail client used the dark selection ink on its less
bright alert fill, leaving DELETE weak at rest. It now uses the existing
background ink for that label in the exact compiled and published nexus
reference palettes. A crate-private `Style::mail_delete_ink` keeps the
palette decision in the style layer; the panel resolves it once before
constructing its three interaction faces. The alert fill and existing
hover/held coats are preserved. Other eras and nonreference palettes keep
their previous ink. This affects `cp-eras-ui-mail`; the photographed
mailbox reference uses a separate implementation.

The existing 14px Rajdhani Regular face, shaping, placement, clipping and
effect work are preserved. Nominal role contrast rises from 4.200 to
5.471. In the actual 1600 render, the darkest glyph pixel changes from
RGB 41/54/40 to 32/37/29, raising its contrast against the unchanged plate
from 3.574 to 4.395. These are descriptive color measurements; antialiased
edge pixels have different contrast. The change darkens 136 pixels, all
inside the six-letter glyph at [755,789,794,798), with exact alpha.

Seventeen frozen native pairs cover compiled/published reference rest,
hover and held faces, rest at 1537×947 and 1024×768, the other three eras,
and burn-in/light custom palettes in all three states. The four reference
rest cases darken 136/136/137/155 glyph pixels respectively; the other
thirteen pairs are RGBA-exact. Astra checks locality at every size and
verifies the actual palette and state fill pixels, so state coverage is
not inferred only from filenames. All other-era resting baselines also
match their committed working-mail goldens.

The unmodified packaged app, scratch baseline and disabled capture probe
reproduce the original golden exactly. The unmodified candidate and its
disabled probe agree exactly too. The probe forces only the existing
DELETE face and preserves the interactive wrapper. All 267 library tests
pass, including existing interaction tests. The source diff changes only
`src/style.rs` and `src/panels/mail.rs`; the sole updated golden is the
working Entropism mail screen.

All 312 packaged Rust tests (267 library and 45 integration), 22 repository
checks and 28 first-attempt matrix cases pass. The packaged working-mail
frame is PNG-byte identical to the reviewed uninstrumented candidate.
Every other matrix image matches the preceding checkpoint; twenty-six of
27 visual images match their goldens exactly, and the existing Neo-kitsch
bar residual remains one pixel at one channel level. The remaining case
checks source sampling. All 370 frozen tracked crate files match the
tested Nix source. Final documentation changes leave the tested package
unchanged. Changes stay staged and uncommitted.

Plans, source/dependency hashes, the exact two-file diff and all captures
are in `/tmp/cp-eras-next/mg-mail-delete-candidate/`; Astra's independent
verification is in `/tmp/cp-eras-next/mh-mail-delete-review/`. The prebuild
harness path mismatch was corrected before the execution harness froze;
no failed capture or visual retuning preceded this result. Repository
integration evidence is in `/tmp/cp-eras-next/mi-mail-delete-integration/`.
