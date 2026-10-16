# Reference SVG round seventy-one — fourth Store top coverage

The ordinary fourth Neomil Store card now uses the same thinner top as
the first and third cards. Its permanent cut stays at x1557, and its held
outline stays bright. Other thin edges and faint printing remain open.

## Construction and source comparison

A filled U-shaped ribbon replaces the open stroked border. Its two left
corners are exact intersections of adjacent half-width offset lines; its
right endpoints remain butt caps at the permanent cut. The top width is
0.6 and the other edges remain 1.2. The centerline, Rect face, content,
material, three primitive slots and viewport are unchanged. No runtime
geometry calculation, effect layer, cache or additional path is introduced.
Fonts, text caches and the shared renderer are unchanged.

At the original width of 1.2, native rendering reproduces the current
package byte-for-byte at 3840×2160, 1600×900 and 1537×947. SVG rasterization
changes 1467 / 590 / 691 edge pixels by at most 2 / 1 / 1 channel levels;
every fixed source region improves or remains exact. This establishes the
representation before testing the thinner top. The 0.6 trial is the same
bounded half-width hypothesis supported by repeated ordinary-top profiles,
not a unique recovery of physical stroke width or blur.

All fourteen fixed source regions improve or remain exact in both RGB L1
and RMS at all three sizes for the thinner SVG and native candidate. Whole
top-region RMS is:

| Viewport | SVG, before → after | Native, before → after |
| --- | ---: | ---: |
| 3840×2160 | 18.772 → 13.273 | 23.849 → 14.315 |
| 1600×900 | 17.751 → 11.774 | 18.143 → 12.769 |
| 1537×947 | 18.189 → 11.943 | 23.113 → 13.045 |

Native changes 947 / 132 / 254 pixels, confined to the fourth top and its
corners. Other edges, contents, selected artwork, the middle cut, exterior
and alpha are exact. Astra reviews source, SVG and native crops at every
size. The component sheet deliberately omits the cropped fourth card, so
no component artwork needs changing.

The existing border test now checks actual painted coverage in rest,
hover and held states instead of requiring an open-path representation.
It requires top/left/bottom ink and an empty middle and exterior at the
right cut. Both variants pass the fifteen Store interaction tests.

## State and repository verification

Four paired reference controls cover fourth hover, fourth held, fourth
selected/held and early opening. The original-width ribbon is byte-exact
to the accepted baseline in every case. The thinner top changes only its
ordinary top/corners; the selected fourth card stays exact. Two custom
palette cases cover rest and fourth held, with disabled-probe parity for
all three binaries. The original-width custom images remain exact, and
the thinner images preserve all pixels outside the expected top. These
are synthetic Pointer snapshots through production drawing and selection
logic, not live desktop input tests.

Both Store fidelity gates pass, with the implementation gate using the
newly built package. All 22 repository checks and 28 matrix cases pass
on their first attempt; packaged tests pass 260 library and 45 other tests
(305 total). Direct pixel checks find 26 of 27 visual cases exact, with
the existing one-pixel, one-level Neo-kitsch bar difference unchanged.
The remaining case checks source sampling. All 353 frozen tracked crate
files match the tested Nix source. Fresh packaged captures at all three
sizes are byte/RGBA-exact to the reviewed candidate. The six final
documentation records are the only post-verification changes. All work
remains staged and uncommitted.

## Next source trial

A separate frozen 0.6 left-and-bottom SVG trial improves all 4K regions,
but every bottom region regresses at both smaller sizes. The combined
trial is rejected, retaining all eleven local losses. A left-only ablation
keeps the original bottom width and improves or preserves all ninety
fixed region comparisons across the three sizes. Its changes are confined
to left edges and left corners. Baseline renders reproduce the fourth-top
candidate exactly; analytic geometry and XML comparisons verify its scope.
This supports a native trial only. No left/bottom native change is included.

Evidence: `/tmp/cp-eras-next/gj-neomil-fourth-top/`, GK/GL proposals,
GM original-width native replay, GN Astra review, GQ thinner native
controls and GP integration. GO retains the rejected combined edge trial;
GR retains the left-only source proposal. The broader primary-coverage
task remains open, with 30 open scopes overall.
