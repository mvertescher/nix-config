#!/usr/bin/env python3
"""Compare two renders and fail if they have drifted.

Companion to visual_diff.py, not a replacement. That script brute-forces
a +/-60px alignment search -- 14641 full-image comparisons -- which is
the right thing when you are eyeballing how close the app is to a design
reference that was drawn by hand at an unknown offset.

This one is for automation: the headless capture is deterministic and
fullscreen, so the offset is always zero, and a build check needs a
comparison that finishes in milliseconds and exits non-zero when the
render changes.

    check_similarity.py <golden.png> <render.png> <min_percent> [diff.png] [max_stray_fraction]

Two independent checks must both pass:

  * mean similarity >= <min_percent> -- the original whole-image score;
  * per-pixel strictness: no more than `max_stray_fraction` of pixels
    may differ from the golden by more than PIXEL_TOLERANCE on any
    channel (defaults 0.05 and 24/255; see the constants below).

The mean alone is not strict: a change confined to under 0.1% of the
frame passes a 99.9% mean check no matter how wrong those pixels are.
The pixel count catches that class. The defaults tolerate essentially
nothing because a legitimately unchanged render is byte-identical to
its golden (the harness's two independent sandbox runs were verified to
match pixel-for-pixel), while a genuine regression moves thousands of
pixels either way.
"""

import sys

from PIL import Image, ImageChops, ImageStat

# A pixel "differs materially" when any of its three channels is more
# than this many 8-bit steps away from the golden. Above this is a
# colour change an eye would notice; below it lives only the sort of
# sub-step jitter that 8-bit PNG round-tripping would introduce.
PIXEL_TOLERANCE = 24

# The largest fraction of pixels that may differ materially while the
# render is still accepted. 5% of a 1600x900 frame is 72 000 pixels --
# far above the ~0 a byte-identical re-render produces, far below what
# a real regression moves. Overridable on the command line.
MAX_STRAY_FRACTION = 0.05


def similarity(golden: Image.Image, render: Image.Image):
    """Mean per-channel difference, expressed as a percentage match."""
    diff = ImageChops.difference(golden, render)
    stat = ImageStat.Stat(diff)
    mean = sum(stat.mean) / len(stat.mean)
    return (1.0 - mean / 255.0) * 100.0, diff


def stray_fraction(diff: Image.Image, tolerance: int = PIXEL_TOLERANCE) -> float:
    """Fraction of pixels whose worst channel is more than `tolerance`
    steps away, as a fraction of the frame.

    C-speed: each channel is thresholded with point() to 0/255, the
    three planes are OR-combined by saturating ImageChops.add (so a
    pixel counts once no matter how many channels moved), and the
    violating pixels are summed from the combined histogram.
    """
    or_all = None
    for ch in range(3):
        band = diff.getchannel(ch).point(lambda p: 255 if p > tolerance else 0)
        or_all = band if or_all is None else ImageChops.add(or_all, band)
    hist = or_all.histogram()
    material = sum(hist[1:])
    return material / (diff.width * diff.height)


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__)
        return 2

    golden_path, render_path, threshold = sys.argv[1], sys.argv[2], float(sys.argv[3])
    diff_path = sys.argv[4] if len(sys.argv) > 4 else None
    max_stray = float(sys.argv[5]) if len(sys.argv) > 5 else MAX_STRAY_FRACTION

    golden = Image.open(golden_path).convert("RGB")
    render = Image.open(render_path).convert("RGB")

    # A size change is a failure in its own right, not something to paper
    # over by resizing: it means the window or output geometry moved.
    if golden.size != render.size:
        print(f"FAIL: size changed: golden {golden.size} vs render {render.size}")
        return 1

    score, diff = similarity(golden, render)
    stray = stray_fraction(diff)

    if diff_path:
        # Amplified, because a real regression is often a few percent
        # that is invisible at 1x.
        diff.point(lambda p: min(p * 8, 255)).save(diff_path)

    print(f"similarity: {score:.3f}% (threshold {threshold:.3f}%)")
    print(
        f"pixels beyond ±{PIXEL_TOLERANCE}/255 per channel: "
        f"{stray * 100:.4f}% (allow ≤ {max_stray * 100:.3f}%)"
    )

    if score < threshold:
        print("FAIL: render drifted from the golden image")
        return 1

    if stray > max_stray:
        print(
            f"FAIL: {stray * 100:.4f}% of pixels differ materially "
            f"(no more than {max_stray * 100:.3f}% allowed)"
        )
        return 1

    print("OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
