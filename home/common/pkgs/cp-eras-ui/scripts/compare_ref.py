#!/usr/bin/env python3
"""compare_ref.py — structurally compare a source design image against a render.

The two existing tools in this directory cover a different need:

  check_similarity.py  diffs two captures of the *same* geometry and fails
                      outright if the size differs. It exists to gate golden
                      regression: two headless renders must be byte-identical,
                      so a strict per-pixel threshold is exactly right.

  visual_diff.py       brute-forces a +/-60px translation between an app
                      render and a hand-drawn SVG, then applies the same
                      per-pixel diff. Still raw pixels.

Neither is the right instrument when the "source" is a full-resolution
photograph of a game UI (Behance, 3840x2160, gradients and noise) and the
"render" is a clean schematic (rsvg-convert of a 1600x900 SVG). A faithful
drawing will score ~0% on per-pixel diff because the text rasterization,
antialiasing and texture never line up — which tells you nothing about
whether the *shapes* are right.

This script is that missing instrument. It resizes both inputs to a common
canvas and compares their *structure* rather than their pixels:

  layout  : downscale each to a coarse grid (default 48x27) and compare the
            per-cell mean colour, so "where are the big blocks of colour" is
            what is scored, not the text inside them.
  edges   : run edge detection on each, then compare the per-cell edge
            density, which captures panel boundaries, chamfer cuts and the
            layout skeleton while ignoring filled text.
  palette : quantize each to a coarse palette and report how much of the top
            dominant colours overlap.

It always writes visual overlays so the numbers can be eyeballed:

  <out>/side-by-side.png  source on top, render below, same canvas
  <out>/checker.png       checkerboard blend (source / render in alternate
                          32px tiles) — the fastest way to see shapes drift
  <out>/edges.png         source edges in red over render edges in cyan
  <out>/heatmap.png       per-cell layout difference, warm = divergent

The scores are directional, not a gate. A high layout score with a high edge
score and a mid palette score is what a faithful schematic looks like; a low
edge score with a high layout score usually means the panels are placed right
but the corner/chrome details are missing. Read the overlays, not just the
numbers.

Usage:

  compare_ref.py SOURCE.png RENDER.png [--out DIR] [--grid WxH] [--regions]

  --regions prints a per-region breakdown using the dashboard's known layout
            (top bar, sidebar, modules, detail, footer) as normalised boxes.

Only Pillow is required (already the harness dependency). Exit status is 0
unless the inputs cannot be read.
"""

import argparse
import os
import sys

from PIL import Image, ImageChops, ImageDraw, ImageFilter, ImageStat


# Dashboard screen regions as (label, x0, y0, x1, y1) in 0..1 coordinates,
# matching src/screens/dashboard.rs layout at the 1600x900 golden geometry.
DASHBOARD_REGIONS = [
    ("top bar", 0.0, 0.02, 1.0, 0.08),
    ("sidebar", 0.0, 0.08, 0.16, 0.93),
    ("modules", 0.21, 0.08, 0.64, 0.93),
    ("detail", 0.64, 0.08, 0.98, 0.93),
    ("footer", 0.0, 0.93, 1.0, 0.985),
]


def pearson(a, b):
    """Pearson correlation of two equal-length float lists."""
    n = len(a)
    ma, mb = sum(a) / n, sum(b) / n
    sa = sum((x - ma) ** 2 for x in a)
    sb = sum((x - mb) ** 2 for x in b)
    if sa == 0 or sb == 0:
        return 1.0 if sa == sb else 0.0
    cov = sum((x - ma) * (y - mb) for x, y in zip(a, b))
    return cov / (sa * sb) ** 0.5


def load_pair(source_path, render_path):
    src = Image.open(source_path).convert("RGB")
    ren = Image.open(render_path).convert("RGB")
    # Common canvas: scale the source down to the render's geometry. If the
    # render is bigger, scale up instead so the comparison is always 1:1.
    target = ren.size
    if src.size != target:
        src = src.resize(target, Image.Resampling.LANCZOS)
    return src, ren


def grid_features(im, cols, rows):
    """Per-cell mean colour and per-cell edge density as two float lists."""
    small = im.resize((cols, rows), Image.Resampling.BOX)
    cells = list(small.getdata())
    colour = [sum(p) / (3 * 255) for p in cells]

    edges = im.convert("L").filter(ImageFilter.FIND_EDGES)
    edge_small = edges.resize((cols, rows), Image.Resampling.BOX)
    density = [v / 255.0 for v in edge_small.getdata()]
    return colour, density


def layout_score(src, ren, cols, rows):
    sc, se = grid_features(src, cols, rows)
    rc, re = grid_features(ren, cols, rows)
    return pearson(sc, rc), pearson(se, re)


def palette_overlap(src, ren, bits=4, top=10):
    """Weighted overlap of the top-N quantized colours in each image."""
    q = lambda im: [tuple(c >> (8 - bits) for c in p) for p in im.getdata()]
    count = lambda qs: dict(
        sorted(_tally(qs).items(), key=lambda kv: -kv[1])[:top]
    )

    def _tally(qs):
        d = {}
        for p in qs:
            d[p] = d.get(p, 0) + 1
        return d

    a, b = count(q(src)), count(q(ren))
    total = sum(max(a.get(k, 0), b.get(k, 0)) for k in set(a) | set(b))
    shared = sum(min(a.get(k, 0), b.get(k, 0)) for k in set(a) & set(b))
    return shared / total if total else 0.0


def region_report(src, ren, regions):
    rows = []
    for label, x0, y0, x1, y1 in regions:
        box = (int(x0 * ren.width), int(y0 * ren.height),
               int(x1 * ren.width), int(y1 * ren.height))
        cs, _ = grid_features(src.crop(box), 16, 9)
        cr, _ = grid_features(ren.crop(box), 16, 9)
        rows.append((label, pearson(cs, cr)))
    return rows


def write_overlays(src, ren, out_dir):
    w, h = ren.size
    tile = 32

    # side-by-side
    canvas = Image.new("RGB", (w, h * 2 + 8), (40, 40, 40))
    canvas.paste(src, (0, 0))
    canvas.paste(ren, (0, h + 8))
    draw = ImageDraw.Draw(canvas)
    draw.line([(0, h + 4), (w, h + 4)], fill=(120, 200, 120), width=2)
    canvas.save(os.path.join(out_dir, "side-by-side.png"))

    # checkerboard
    checker = Image.new("RGB", (w, h))
    for y in range(0, h, tile):
        for x in range(0, w, tile):
            box = (x, y, min(x + tile, w), min(y + tile, h))
            checker.paste(src.crop(box) if (x // tile + y // tile) % 2 == 0
                          else ren.crop(box), box)
    checker.save(os.path.join(out_dir, "checker.png"))

    # edges overlay: source edges red, render edges cyan
    def edges(im):
        return im.convert("L").filter(ImageFilter.FIND_EDGES).point(
            lambda p: 255 if p > 60 else 0)
    es, er = edges(src), edges(ren)
    red = Image.new("RGB", (w, h), (255, 0, 0))
    cyan = Image.new("RGB", (w, h), (0, 255, 255))
    overlay = Image.new("RGB", (w, h), (12, 12, 14))
    overlay.paste(red, (0, 0), es)
    overlay.paste(cyan, (0, 0), er)
    overlay.save(os.path.join(out_dir, "edges.png"))

    # per-cell layout difference heatmap
    cols, rows = 48, 27
    sc, _ = grid_features(src, cols, rows)
    rc, _ = grid_features(ren, cols, rows)
    heat = Image.new("RGB", (w, h), (0, 0, 0))
    d = ImageDraw.Draw(heat)
    cw, ch = w / cols, h / rows
    for i in range(cols * rows):
        diff = abs(sc[i] - rc[i])  # 0..1
        # cool (blue) = agree, hot (red) = diverge
        colour = (int(diff * 255), int((1 - diff) * 120), int((1 - diff) * 255))
        x0, y0 = (i % cols) * cw, (i // cols) * ch
        d.rectangle([x0, y0, x0 + cw, y0 + ch], fill=colour)
    heat.save(os.path.join(out_dir, "heatmap.png"))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("source", help="source image (e.g. the Behance reference)")
    ap.add_argument("render", help="render to compare (e.g. rsvg-convert of an SVG)")
    ap.add_argument("--out", default=".", help="directory for overlay images")
    ap.add_argument("--grid", default="48x27", help="layout grid as WxH")
    ap.add_argument("--regions", action="store_true",
                    help="print per-region breakdown for the dashboard layout")
    args = ap.parse_args()

    cols, rows = (int(v) for v in args.grid.split("x"))
    if not os.path.exists(args.source) or not os.path.exists(args.render):
        print("error: source and render must both exist", file=sys.stderr)
        return 1
    os.makedirs(args.out, exist_ok=True)

    try:
        src, ren = load_pair(args.source, args.render)
    except Exception as e:  # noqa: BLE001
        print(f"error: could not read images: {e}", file=sys.stderr)
        return 1

    lay, edge = layout_score(src, ren, cols, rows)
    pal = palette_overlap(src, ren)

    print(f"source {args.source} ({src.size[0]}x{src.size[1]})")
    print(f"render {args.render} ({ren.size[0]}x{ren.size[1]})")
    print(f"grid   {cols}x{rows}")
    print(f"layout colour correlation : {lay:5.3f}  (1.0 = blocks of colour land in the same cells)")
    print(f"edge density correlation  : {edge:5.3f}  (1.0 = panel boundaries/chamfers align)")
    print(f"palette overlap (top-{8}) : {pal:5.3f}  (1.0 = same dominant colours)")

    if args.regions:
        print("\nper-region layout (dashboard geometry):")
        for label, score in region_report(src, ren, DASHBOARD_REGIONS):
            print(f"  {label:9s} {score:5.3f}")

    write_overlays(src, ren, args.out)
    print(f"\noverlays written to {args.out}/ (side-by-side.png, checker.png, "
          f"edges.png, heatmap.png)")

    # Directional guidance, not a gate.
    if lay > 0.8 and edge > 0.6:
        print("reading: panels placed well; chrome/ornament fidelity is the "
              "remaining gap (check edges.png).")
    elif lay > 0.6:
        print("reading: overall layout recognisable; panel shapes or placement "
              "drift (check checker.png + heatmap.png).")
    else:
        print("reading: layout differs substantially (check side-by-side.png).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
