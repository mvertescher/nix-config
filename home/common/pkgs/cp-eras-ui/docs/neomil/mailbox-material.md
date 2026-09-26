# Mailbox material correction, 2026-09-21

Source: `images/img-08-main.png`, Behance #61, 3840×2160.
Coordinates below use the 1600×900 design unless marked native. The source
was inspected whole and in native crops before fitting. These are visible
composite colors; they do not identify the original shader or alpha stack.
The SVG and `src/eras/neomil_mailbox_material.rs` carry the same geometry,
rounded RGB controls, scan pitch and masks. No source bitmap is embedded.

## Ground

The previous mailbox reused an older, short blue glow followed by a warm
wash. The source instead has cyan down to the message panel. The accepted
dashboard background reconstruction already describes this same source
asset: native RGB bytes agree exactly in three unobstructed mailbox and
dashboard patches, (500,20)–(600,65), (30,700)–(100,755), and
(1475,360)–(1530,430), totaling 70,272 pixels.

Mailbox `GROUND` therefore reuses the unchanged
`dashboard_ground::BACKGROUND` slice: twelve horizontal RGB ramps under
eleven full-frame vertical blend masks. This does not modify any dashboard,
login or store data. The mailbox SVG carries equivalent local definitions.

An independent mailbox evaluation uses 3,829,324 native clear-background
pixels. It excludes header/labels, list/icons/scroll rail, body panel,
buttons, margin printing and footer, using these design rectangles:

```text
(90,68)–(430,180)       (1110,68)–(1390,180)
(0,180)–(1600,193)     (230,192)–(460,222)
(715,192)–(946,222)    (30,210)–(90,244)
(1520,210)–(1570,244)  (110,298)–(590,900)
(720,305)–(1472,770)   (725,260)–(1010,298)
(760,852)–(930,895)    (1295,850)–(1340,885)
(32,370)–(72,606)
```

Both candidates are actual native SVG renders, using the earlier ground
alone or the sampled ground alone on this fixed mask:

| Clear-background metric | Previous | Revised |
|---|---:|---:|
| RGB RMS, 0–255 scale | 12.289 | 1.787 |
| Channel MAE, R/G/B | 3.39 / 7.53 / 12.10 | 1.02 / 1.48 / 1.50 |
| Channel 95th-percentile error | 8 / 26 / 39 | 2 / 4 / 4 |
| Pixels differing by more than 8 in any channel | 49.663% | 0.082% |

The broad source comparison validates reuse rather than fitting new colors
to this mask. Hidden ground remains interpolated; exact common-asset bytes
in three patches do not prove identity underneath opaque UI elements.

## Message-panel field

The old opaque `#1c0608` panel suppressed the upper cyan entirely. The
replacement is an opaque sampled composite clipped to the same chamfered
contour, with x controls at 729,850,1000,1150,1300,1450 and y controls at
312,340,370,400,440,480,560,640,699. The upper blue/purple region transitions
to the retained warm lower field. Full-bounds ramp layers avoid translucent
seams between neighboring strips when the scene scales fractionally.

A fixed source mask contains 769,458 native pixels inside
(735,318)–(1433,693). It removes source pixels with R>62 and R>1.8G,
dilated by sixteen native pixels to exclude printing and its faint copies,
and removes x>1404 above y405 to exclude the vertical brands. Every third
remaining pixel enters the sparse bilinear fit. Horizontal second
differences weakly regularize controls obscured by text. No text silhouette
or source raster enters the application; missing field samples are inferred.

The red residual independently measures a 1.985045px scan pitch. The
implementation retains the previously established 1.984934px pitch, fixed
before the mailbox holdouts, and fits only phase and RGB modulation. A
seventeen-stop cosine-like luminance cycle blends two color fields. Its
endpoints are black, so cycle joins cannot create bright seam lines. The
phase is local to this panel; it is not copied from the dashboard panel.

Training/evaluation alternate whole 37×23-design-pixel blocks. Separate
field and modulation coefficients are fitted to each training half and
rendered as an actual SVG before scoring the other half:

| Held-out native pixels | Earlier flat panel RMS | Revised SVG RMS |
|---:|---:|---:|
| 386,227 | 9.010 | 1.420 |
| 383,231 | 9.012 | 1.411 |

These holdouts are spatial partitions of one image, not independent
screens. The final all-mask SVG has RGB RMS 1.410 and channel MAE
1.00/1.11/1.11, versus 9.011 and 3.55/4.35/5.11 before. Its channel
95th-percentile error is 3/3/3; no pixel in the fixed mask exceeds eight
levels in any channel, versus 19.47% before. These local measurements do
not certify the text, brands, frame or silhouettes excluded from the mask.

At native coordinates corresponding to design (800,265), the source is
RGB 16/40/73 and the revised ground is 16/40/72. Inside the panel at
(900,325), source 35/37/57 becomes 35/39/58; the earlier panel was 28/6/8.
The lower panel at (1400,650) remains close: source 25/8/9, revised 25/7/8.
These are individual native pixels, so they differ slightly from the
downscaled audit's samples.

## Badge fields

The mailbox badge contours match the measured dashboard primary contours,
but their fields differ. Clear lower mailbox patches have roughly six
fewer red levels in CUSTOMER, seven fewer in unselected security badges,
and fifteen fewer in selected T2. Copying the dashboard fields would
therefore introduce a measurable color error.

Each mailbox badge has its own four-corner bilinear RGB field. Unselected
fields carry a separately fitted phase/modulation at the established
1.984934px pitch. Selected T2 has no measurable coherent modulation at
that pitch (fitted amplitudes below 0.04 levels), so it retains a smooth
field. Frames and primary lettering are drawn separately after the fields.

The fixed native masks inset each contour by at least three design pixels,
exclude the chamfer, exclude the central text area above y145, and remove
R>170 printing dilated seven native pixels. They contain 4,558–4,574
pixels apiece. Comparing actual native SVG material renders on those masks:

| Badge | Previous RGB RMS | Revised RGB RMS |
|---|---:|---:|
| CUSTOMER | 22.894 | 1.831 |
| T1 | 34.269 | 2.093 |
| Selected T2 | 35.519 | 1.063 |
| T3 | 34.940 | 2.156 |
| T4 | 34.578 | 2.117 |

These are descriptive fits on their clear fields, not a text-fidelity
score. Fine scan contrast, weak printing residues and grain remain visible
in the unselected fields. No new `photo` exclusions or gate thresholds
hide those residuals.

## Integration and limits

`GROUND` and `BADGES` belong in the mailbox's initial software surface.
`PANEL` is separately composited with panel printing, inside the existing
message-opening motion; the foreground panel frame must have no opaque
fill. This retains the startup clip and lets the static surfaces use the
existing compositor cache. A regression checks that the panel stays
opaque across every internal field/scan join at five scales, including
fractional scales and 2.4× source scale. Runtime rendering and the complete
repository checks are performed by the integrating change.

The source's exact background texture and some fine scan/printing detail
remain unrecovered. The common source patches demonstrate fixed shared
texture, not independent noise. Arbitrary procedural grain would add
unsupported material. Coarse sampled colors, local periodic modulation
and rounded byte controls reconstruct the visible low-frequency field;
their serialization precision is not measurement accuracy.
