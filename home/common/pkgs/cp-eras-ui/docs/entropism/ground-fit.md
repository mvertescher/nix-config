# Entropism ground fit — 2026-09-27

The old three-stop radial fields were too olive at the sides and missed
the bright upper area. This fit changes only the broad photographed
ground. Frames, printing, layouts, and the login's solid sage footer
band retain their previous inks and geometry. Fine grain, printing
echoes, and irregular local light remain outside this model.

## Sources and relationship

The four native files are 3840×2160; positions below use the 1600×900
trace frame (native coordinates divided by 2.4). The filenames are
historically swapped: `images/entropism-store.png` is the module **hub**,
and `images/entropism-dashboard.png` is the 4ST **store**. The other two
files are `entropism-mail.png` and `entropism-login.png`.

The hub, mail, and store have the same broad field. At the clear centre
of the upper edge (800,40), their native patch medians are 34/35/25,
34/35/25, and 34/35/25. The narrow left edge (20,300) is 15/9/1 on
all three. A conservative patch filter finds 920 matching locations in
their upper 340 design pixels; their RGB medians agree within two
levels. Login is consistently dimmer: its (800,40) patch is 29/30/22
and its lower clear field is near 13/9/4. Login therefore has its own
stop colors and ellipse.

## Model

Each screen paints a page color and one elliptical `Prim::Lobe` inside
`Prim::Soft`. The matching SVG uses a user-space radial gradient with a
vertical scale. Offsets interpolate in encoded RGB in both renderers.

| Screens | Page | Centre | Radii | Stops at 0 / .25 / .5 / .75 / 1 |
| --- | --- | --- | --- | --- |
| Hub, mail, store | `#0f0903` | (935,5) | (943,420) | `#1f1e15` / `#1f1f15` / `#1f1e14` / `#18140a` / `#0f0903` |
| Login | `#0d0804` | (950,-139) | (1172,767) | `#1a150d` / `#1a1a14` / `#1c1a12` / `#0f0b04` / `#0d0804` |

The common ellipse spans the top and fades near the middle. Its left
and right margins remain near black. The login ellipse is wider and
weaker. The login footer fill at x36..1565, y765..880 is drawn after
the field, with its previous `#8aac8c` source-derived fill.

## Fit and holdout

I sampled 20×20 native-pixel patches every 16 design pixels and used
per-channel medians. A patch entered the common fit only when all three
screens agreed within two RGB levels, its robust local variation was at
most two levels, and no channel exceeded 65. Login used the same
variation and brightness limits, and excluded its controls and footer.
The upper y≤340 patches were split by alternating grid parity before
fitting the ellipse and stops. The held-out half, plus clear patches
below y340, was used for evaluation. SVG evaluation also rejected
patches whose rendered foreground crossed the sampled area.

The table gives channel RMS from source medians on held-out clear
patches. “Before” evaluates the previous radial stop table at the
same positions; “after” samples the actual rendered SVG. Upper is
y<300, middle y300..600, lower y≥600, and sides x<250 or x>1350.
The regions overlap, and each score pools all three RGB channels.

| Screen | Upper before → after | Middle before → after | Lower before → after | Sides before → after |
| --- | ---: | ---: | ---: | ---: |
| Hub | 8.89 → 1.61 | 9.10 → 0.93 | 7.14 → 0.95 | 6.35 → 1.24 |
| Mail | 7.66 → 1.61 | 9.70 → 0.88 | 7.54 → 0.95 | 6.67 → 1.26 |
| Store | 7.33 → 1.58 | 10.34 → 1.08 | 7.87 → 0.92 | 6.73 → 1.21 |
| Login | 5.71 → 1.36 | 8.36 → 0.77 | 7.70 → 0.83 | 6.52 → 1.21 |

At (800,40), the common SVG renders 31/31/21 against 34/35/25 in
the source; login renders 26/26/20 against 29/30/22. At (20,300),
the common SVG renders 15/9/3 against 15/9/1. The first difference
is a small, broad upper underfit; the second is a warm one- or two-level
base discrepancy. Both are smaller than the prior field error. This
single smooth ellipse does not reproduce localized exposure, grain,
or faint green residues among the foreground. It should be judged as
a broad field correction, not as a full material reconstruction.

The four SVGs rendered successfully with the pinned librsvg. Astra reviewed
all four original/SVG/native Iced comparisons. On a separate grid of clear
20×20-native-pixel patches, actual Iced and SVG medians differ by at most
one RGB level. Upper-field source error in Iced is 2.04 for the hub, 1.34
for login, 1.89 for mail and 2.15 for store; middle/lower errors are
0.70–1.71. These sample masks differ from the fitting holdout above,
but independently confirm the renderer reproduces the fitted ground.
Full crate checks remain part of integrated verification.

The ground review revealed a separate pre-existing login footer mismatch:
the SVG retained its source-supported `#8aac8c` fill while Iced used the
brighter selection role. The subsequent reference-only plate override now
reproduces RGB138/172/140 in native Iced for both standalone and published
source palettes. Regression coverage retains semantic Select for custom
palettes. The ground geometry is independent of that surface correction.
