# Dashboard printing follow-up, 2026-09-21

This continues the [dashboard fidelity audit](dashboard-fidelity.md).
Source: `images/img-07-dashboard.png`, 3840×2160, Behance asset
`3fc4ef118663901.60e5fa6a7f2f7.png`. Coordinates use the 1600×900 design
unless marked native. The proposals started on September 14 and were
recovered after the preceding dashboard and reader fixes were committed.

## Margin and vertical-brand copies

The two JHN codes, KIROSHI wordmark and two GO HOME vertical brands have
separately measured outward copies. Primary paths, fonts and colors stay
unchanged. These local fits do not identify the original effect's cause.

| Family | First copy dx,dy | Second copy |
|---|---:|---|
| left JHN | −3.615342,+0.165646 | twice the first translation |
| right JHN | +3.603943,+0.597206 | twice the first translation |
| KIROSHI | +3.608415,+1.257728 | twice the first translation |
| BETTERLIFE TEC | +3.059864,−0.553728 | twice the first translation |
| PETROCHEM/frame | +2.766196,−0.522796 | twice the first translation |

Finite profiles approximate softness. Margin profiles add 2.4, 1.2, 0.4
or 0.0 to the existing glyph stroke width. Brand profiles expand the
filled outlines by 1.4, 0.7, 0.25 or 0.0; the small PETROCHEM frame adds
the same widths to its existing 0.4167 stroke. Each complete profile
receives alpha once, after combining touching paths. The fainter second
copy is painted first. Explicit transparent-ended ramps over an opaque
gray floor retain the established 1.984934 pitch, with independently
fitted phase and contrast per family. No whole-screen transform or noise
field is added.

The SVGs reuse the accepted primary margin paths; Rust reuses
`dashboard_type::{LEFT_CODE, RIGHT_CODE, KIROSHI}`. Brand copies use twelve
outlines from bundled Rajdhani SemiBold at the primary's size 6.25 and
horizontal stretch 1.3. This retains the existing font approximation;
it does not identify the source font. Isolated native text/outline checks
have alpha MAE 1.84/1.76 levels and hard-mask IoU 0.967/0.995 for
BETTERLIFE/PETROCHEM. Primary brand lettering remains text.

The margin copies join the first software surface; brand copies stay
under the GO HOME opening clip and before the primary panel frame. This
order matters where a faint brand copy overlaps that frame. The component
sheet adds a native left-code excerpt and expands its right-margin crop
to retain the farthest copy edges. Other-screen excerpts are untouched.

## Margin validation and limits

The native fixed ROIs omit the union of baseline R>130 and source R>170
primary printing, dilated one pixel; right-code fitting also excludes the
preceding arrow. Training and held-out pixels alternate in 19-pixel bands
along the vertical runs. Brand masks use baseline R>150/source R>170 and
17-pixel bands. These holdouts are within the same source image, not unseen
screens or independent photographs. Actual full-SVG renders were scored:

| Family | Held-out pixels | RGB RMS before → after, 0–255 scale |
|---|---:|---:|
| left JHN | 7,659 | 10.753 → 3.981 |
| right JHN | 9,108 | 10.999 → 3.938 |
| KIROSHI | 4,307 | 12.316 → 5.043 |
| BETTERLIFE TEC | 3,090 | 10.984 → 6.073 |
| PETROCHEM/frame | 2,490 | 11.304 → 5.251 |

All margin channel MAEs improve. Brand improvements are chiefly red;
green/blue MAE remains about 3.0–3.7 and can worsen by up to 0.51. Each
independent brand model improves held-out RMS (10.984→8.945 and
11.304→8.144); the combined SVG improves further because the neighboring
copies contribute within overlapping ROIs. Source/design-size crop review
confirms the intended faint printing; it also retains visible residuals
in primary coverage, frame width, local ground and scan contrast.

No source bitmap, photo exclusion, gate mask or threshold change is
introduced. The rounded KIROSHI O arc and all original margin center lines
are preserved. Serialization precision is not source measurement accuracy.

## GO HOME heading and maker primary printing

The heading remains bundled Rajdhani Bold. Fitting its origin to
(1135.7820,333.3901), size 20.8708 and horizontal stretch 0.987681 corrects
the bright envelope, using sampled ink #f83333. Medium/SemiBold/Bold
bright-core IoUs were 0.297/0.753/0.955. Two locally fitted copies use
multiples of (2.010417,−0.692179), opacities 0.324452/0.236801 and the
existing scan pitch; they do not reuse the body-line transform.

The maker M retains its broad anchors and gains rounded shoulders,
notches and lower corners. Its detached dot has a thin bright rim and
darker interior. Independent periodic RGB fields stay within the two
primary silhouettes: source interior means are approximately
(238.44,48.58,48.58) for M and (185.58,37.17,37.60) for the dot. Opaque
endpoint-colored underlays prevent seams between rows. The regenerated
M/dot union supplies two copies at multiples of (2.933105,1.289086),
opacities 0.358614/0.202403, with its own scan phase and contrast.

The two small lines use measured geometric monospace strokes, including
barred I, instead of a lighter Rajdhani weight. PRECISION LIQUID starts
at (1220.2237,732.6243), with cell 2.94545×4.31275, advance 4.08625 and
stroke 0.4167. POLYMER MUSCLE starts at (1224.2702,741.1263), with cell
2.85552×4.17979, advance 4.09775 and stroke 0.417597. Both use #f93333.
Reusable letter paths reconstruct visible geometry without identifying
the original font. Their faint trailing copies and edge softness remain
open; the maker's narrow edge halo is also not recovered exactly.

Actual native SVG comparisons against fixed source crops:

| Whole crop, design coordinates | RGB RMS before → after | Bright-core IoU before → after |
|---|---:|---:|
| heading (1132,316)–(1224,338) | 55.452 → 7.738 | 0.161 → 0.954 |
| maker (1214,677)–(1298,732) | 12.937 → 7.225 | 0.965 → 0.987 |
| PRECISION (1217,730)–(1288,740) | 48.364 → 21.990 | 0.180 → 0.613 |
| POLYMER (1221,739)–(1285,748) | 48.983 → 24.501 | 0.205 → 0.629 |

These geometry comparisons are fitted descriptive results. Independent
echo holdouts fit GO and evaluate HOME, then reverse the words. A fixed
mask excludes primary alpha>0.25/source R>190, dilated one native pixel,
and retains the region within 22 pixels of primary alpha>0.2. Held-out
RGB RMS falls 17.897→6.180 and 17.559→6.190 from the corrected primary
without copies. Compared with the old drawing it falls 39.342→6.180
and 36.395→6.190.

Maker echo holdouts use alternating 17×19 native-pixel blocks, excluding
primary alpha>0.25/source R>170 and retaining a 24-pixel neighborhood.
Both improve the old echo drawing: 8.201→6.247 and 7.194→6.158. For the
interior ink, actual-render left/right and alternating-cycle holdouts
improve over each training partition's mean: M RMS about 1.36–1.37 falls
to 0.73–0.76; dot 4.59–4.85 falls to 1.64–2.30. Final all-mask interior
RMS is 0.740 for M and 1.695 for the dot, versus 32.997 for the old dot.
The M mask erodes four native pixels and omits the dot region; the dot
uses fixed design bounds (1274.25,716)–(1282.75,725.5).

Direct 1600×900 SVG renders also improve all four crops. Both SVG sheets
and Rust carry the same primary geometry and material; all panel copies
and maker material remain under the existing GO HOME opening clip.

## Six matrix/code ink groups

The original matrix modules, code paths, circular symbols, rotation and
placement remain exact. Unbounded dark copies were rejected because they
introduced haze between source modules. The accepted model varies ink
only inside the existing silhouette M. Two displaced copies M1/M2 set
coverage to `1 - (1-a0)*(1-a1*M1)*(1-a2*M2)` inside M and zero outside.
A seventeen-stop RGB cycle uses the existing 1.984934 design-pixel pitch.
The SVG reuses white copies of the original art; Rust invokes the same
geometry macros with white for its masks. Group opacity is applied once
to each union, avoiding compounded alpha where pieces touch.

The source mask is fixed: original glyph alpha>0.05 dilated by ten native
pixels within a 76×76 design-pixel crop about each tile center. Independently
fitted complementary 17×19 native-pixel checker partitions give these
actual full-SVG held-out RGB RMS values:

| Glyph | Mask pixels | First fold before → after | Complement before → after |
|---|---:|---:|---:|
| VEHICLES | 12,244 | 21.03 → 10.74 | 24.66 → 12.35 |
| LOCATIONS | 11,964 | 17.81 → 10.51 | 20.61 → 10.69 |
| FACTIONS | 12,013 | 10.51 → 8.33 | 11.82 → 8.31 |
| WEAPONS | 13,386 | 21.69 → 12.84 | 22.36 → 13.49 |
| PRODUCTS | 13,364 | 16.80 → 11.84 | 15.19 → 10.97 |
| CORPORATIONS | 13,700 | 13.79 → 11.48 | 13.65 → 10.21 |

Alternating whole-cycle holdouts also improve in both directions for all
six groups over the coverage-only model. Most of the improvement comes
from spatial coverage; the periodic term is smaller. Separate matrix,
code and circle submasks improve in both spatial folds, so large modules
do not conceal regressions in smaller printing. The weakest improvement
is FACTIONS code, 8.12→7.97. Submasks use coordinates rotated +45° about
each center: code x<6,y>9 (upper row) or y>13 (lower); symbol x≥6,y>5
or y>9; the remaining mask is matrix. No subgroup parameters are fitted.

Exact XML comparison preserves all six original art groups after only
normalizing IDs and colors. In actual native SVG crops, zero pixels
outside the original nonzero glyph alpha change. Flat tile caps and gaps
remain #ef3333. The parameterization is an explanatory reconstruction:
coverage and base ink are partly interchangeable, and some fitted channels
reach zero. It does not recover unique original paint colors or opacities.
Fine antialiasing, pixel variation and thin-code residuals remain visible.

The app paints resting faces and glyph material in the first software
surface; crisp insets, tabs and seams remain in the foreground plates.
Faces retain `Ink::Fg`, and plate bounds, group and index are unchanged.
Hover/held drawings remain opaque and cover only their own resting tile
with the existing inferred inks. This moves resting outer-edge
antialiasing to the software rasterizer; state captures must check both
upper/lower targets and adjacent resting cells at fractional scale.

## Integrated app validation

All 209 Rust tests pass (164 library, 45 binary), including the existing
state-geometry and software-surface invariants. Static review found matching
SVG/Rust masks, transforms, ink stops and clipping. Native release captures
and fractional 2400×480 state previews were inspected with close-ups; the
GO HOME 0.15-second frame retains its opening clip. Against an otherwise
identical all-rest preview, hover/held changes stay within the target tile
for both VEHICLES and WEAPONS; no neighboring glyphs or panel pixels change.

Close-up review caught a renderer difference hidden by broad gate scores:
the new 0.42px microtext strokes became jagged through canvas tessellation.
Moving those same paths into the existing panel software layer reduced
SVG→app RMS from 17.79 to 6.46 in the fixed design crop
(1217,730)–(1289,749). Changes are confined to (1220,732)–(1285,746).
The corrected native and fractional captures were reviewed again.

| Gate | Matched shapes | Matched area | Median center error | Ink occupancy IoU |
|---|---:|---:|---:|---:|
| dashboard source→SVG | 18/30 | 83% | 0.5px | 0.87 |
| dashboard SVG→app | 19/30 | 95% | 0.0px | 0.71 |
| Login SVG→app | 26/31 | 96% | 0.0px | 0.78 |

All three gates pass unchanged. Their palette segmentation and broad shape
matching are not detail-completeness measures; the local source masks and
reviewed captures above establish the scope of this correction.

Reference and fallback dashboard captures are identical at the normal
four-second release settle. All six cap samples remain (239,51,51),
the source #ef3333. Only the two dashboard goldens and the Neomil Login
golden are refreshed. Login's previously committed hollow-slot correction
had not reached its golden: its entire difference is (497,635)–(501,655).
Caret-on/off captures confirm the slot stays static while the password
tail blinks; see [the source follow-up](../source-followups.md).

The full repository check passes all 19 checks, including all 25 golden
render cases. Other era/screen goldens remain unchanged. The completed
batch is staged for review; it has not been committed or deployed.

Live desktop input, navigation, first-frame behavior and latency remain
unverified by these headless checks. Fine background material still needs
the original asset or authoring recipe. Maker microtext trails/edge
softness and chip-2/tiny tape-mark ink are the next local source fits.
