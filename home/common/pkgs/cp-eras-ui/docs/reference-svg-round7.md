# Reference corrections, seventh batch — 2026-09-27

Sol implementation lanes and Astra review continued from the staged sixth
batch. This checkpoint preserves that work and adds measured source art,
printing and broad materials. Native source/SVG/Iced crops are the fidelity
evidence; matching goldens alone do not establish correctness.

## Primary login artwork and printing

Neomil now draws the source active avatar and the repeated face/clothing
portrait, rather than generic silhouettes. Six measured vector tone masks
retain 105 portrait regions; no bitmap is embedded. The active-avatar dark
mask overlap improves from .169 to .872 in native Iced (.878 in SVG).
The mask pitch, USER 01 weight/placement, captions and separate moving tail
are fitted independently. An 80-mask fractional input changes no pixels
outside the field; dark blink changes only the caret. The full secret and
existing submission behavior are preserved.

Era-owned header art replaces hardcoded protocol bars and tape. LEVEL uses
open, non-retraced line paths; tier glyphs, tape-code contours, margin
marks and rotated labels follow source measurements. A screen-local bright
foreground applies only to the full reference palette. Explicit plate
fills retain priority, and customized palettes keep semantic colors.
Native review accepts the primary art and printing; card/badge material,
secondary echoes, posterized portrait tones and small glyph-edge residuals
remain separate. See [login measurements](neomil/login-primary.md).

Neo-kitsch login now has the measured stencil ARASAKA contour and fitted
cell annotations. Mailbox subject/sender/body weights and widths are closer
to source without changing words or line breaks. The original bar field
and inferred hover preblend remain intact. See [branding](neokitsch/login-branding.md)
and [mailbox typography](neokitsch/mailbox-typography.md).

## Store art and broad grounds

Entropism's plain rifle uses two nested native-source contour masks,
retaining rail holes, fasteners, receiver grid, grip and stock openings.
Native plain-mask overlap is .931, with best alignment at zero offset.
The selected weapon is shifted by its measured 14 native pixels; its
remaining light-rail-channel loss requires a distinct source contour, so
E4 remains open. The 4ST contours and stats/socket/compliance lettering are
fitted locally. The login footer now keeps the source fill for standalone
and published reference palettes. See [store art](entropism/store-art.md).

All four Kitsch and Neo-kitsch broad grounds use measured source patches
and held-out comparisons. Kitsch dashboard/mail/store share a field while
login retains its independently measured field. Neo-kitsch's four clear
source regions establish one shared violet/blue field. Native Iced realizes
the accepted SVG models; exact photographic texture is not claimed. The
Entropism ground fit and derived-sheet/bar provenance correction are also
reviewed. See [Kitsch](kitsch/ground-fit.md),
[Neo-kitsch](neokitsch/ground-fit.md) and
[Entropism](entropism/ground-fit.md).

## Upload cost

The image cache omits all-zero RGBA bands while preserving every nonempty
band's coordinates, pixels, filtering and separate paint order. At 4K,
payload falls from 253.1 to 82.4 MiB and the measured software-renderer
callback interval falls from 1.817 to .511 s. Process peak RSS falls from
3723 to 988 MiB. First CPU preparation has a small scanning overhead.
Rest/opening captures are pixel-identical; a fractional interaction preview
has one repeatable one-level red difference. These observations do not
establish live hardware presentation or continuous-resize behavior. See
[upload measurements](neomil/image-upload.md).

## Validation checkpoint

The final native-art build passes 263 Rust tests (218 library, 45 bar-window).
Astra reviewed original/SVG/Iced native crops, fractional input/feedback and
opening evidence. Thirteen reviewed screen goldens are intentionally
refreshed. The source gate's palette/support corrections have synthetic
missing/moved-widget controls, with unchanged match and area thresholds.
The full repository check passes all 22 checks, with all 27 visual cases
matching 100.000% on their first attempt. All 16 source/SVG gates pass;
all 13 affected SVG/Iced gates pass. Nineteen extractor controls retain
missing/moved-widget and genuine-overlap failures. The contour-support
corrections resolve false component splits without lowering matching
thresholds. See [extraction evidence](shape-support.md). The next material/inverse-art changes are not part of
this H snapshot.

Evidence is under `/tmp/cp-eras-completion/`. Changes remain staged and
uncommitted; live desktop and authentication checks remain separate.
