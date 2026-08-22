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

    check_similarity.py <golden.png> <render.png> <min_percent> [diff.png]
"""

import sys

from PIL import Image, ImageChops, ImageStat


def similarity(golden: Image.Image, render: Image.Image):
    """Mean per-channel difference, expressed as a percentage match."""
    diff = ImageChops.difference(golden, render)
    stat = ImageStat.Stat(diff)
    mean = sum(stat.mean) / len(stat.mean)
    return (1.0 - mean / 255.0) * 100.0, diff


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__)
        return 2

    golden_path, render_path, threshold = sys.argv[1], sys.argv[2], float(sys.argv[3])
    diff_path = sys.argv[4] if len(sys.argv) > 4 else None

    golden = Image.open(golden_path).convert("RGB")
    render = Image.open(render_path).convert("RGB")

    # A size change is a failure in its own right, not something to paper
    # over by resizing: it means the window or output geometry moved.
    if golden.size != render.size:
        print(f"FAIL: size changed: golden {golden.size} vs render {render.size}")
        return 1

    score, diff = similarity(golden, render)

    if diff_path:
        # Amplified, because a real regression is often a few percent
        # that is invisible at 1x.
        diff.point(lambda p: min(p * 8, 255)).save(diff_path)

    print(f"similarity: {score:.3f}% (threshold {threshold:.3f}%)")

    if score < threshold:
        print("FAIL: render drifted from the golden image")
        return 1

    print("OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
