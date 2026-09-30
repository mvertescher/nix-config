#!/usr/bin/env python3
"""extract_spec.py — measure a design image into a machine-readable spec.

The problem this exists to solve: `compare_ref.py` scores two images against
each other on coarse grid statistics. That is the right instrument for "is
this trace roughly in the right place", but it is far too blunt to notice
that a trace shows *the wrong things*. `docs/neomil/dashboard-trace.svg`
scored 0.560 layout against `img-07-dashboard.png` — comfortably above the
~0.15 "unrelated scenes" baseline — while drawing three chart cards for a
source that actually holds a six-diamond menu and a chamfered info panel.
A blue band on top and red mass in the middle is enough to pass a grid
correlation. It is not enough to be a trace.

This script measures the *inventory* instead: what shapes are present, of
what class, at what coordinates, in what colours. Two specs can then be
diffed shape-by-shape (`spec_diff.py`), which turns "the trace is a
fabrication" from something you need eyes to notice into a failing gate.

It is deterministic: no RNG, no sampling. Same bytes in, same JSON out.

    extract_spec.py IMAGE.png [-o spec.json] [--canvas 1600x900]
                              [--palette-from SOURCE.json]
                              [--crops DIR] [--debug DIR]

Pipeline:

  1. resize to a common canvas (LANCZOS), so source photos and SVG renders
     are measured in the same coordinate space.
  2. quantize to a small palette by k-means with deterministic seeding
     (the k most-populated coarse bins, no RNG).
  3. split palette into "ground" and "ink": a cluster is ground if it holds
     a meaningful share of the image border. Gradients and backdrop bands
     reach the border; foreground widgets do not. A cluster that never
     reaches the border but sits within a few RGB units of one that does
     is the inner band of the same gradient, and is ground too.
  4. close small gaps and recover fine striped material only inside an
     independently measured similar-colour body; label connected ink
     components; split touching/overlapping convex
     blobs by nearest-peak (Voronoi) assignment on the distance transform.
  5. fit each component against shape templates (rect, diamond, chamfered
     rect, rule) and keep the best by an occlusion-aware IoU.
  6. group leftover small components into text runs, so a paragraph reads
     as one entry instead of two hundred.

Needs numpy + scipy + Pillow; `fidelity_check.sh` finds or nix-builds one.
"""

import argparse
import base64
import json
import math
import os
import sys
import zlib

import numpy as np
from PIL import Image
from scipy import ndimage

# A component smaller than this is a glyph, not a widget: it goes to the
# text-run grouper rather than being fitted as a shape.
MIN_SHAPE_AREA = 900
# Below this IoU a component is recorded as class "blob": present and
# measured, but not claimed to be any particular shape.
MIN_SHAPE_IOU = 0.62
# A cluster owning more than this share of the border is backdrop. Widgets
# in a framed design do not reach the canvas edge at all, so the ink clusters
# sit at ~0 and anything that touches the frame is a band or a gradient.
GROUND_BORDER_SHARE = 0.015
# A cluster this close (RGB Euclidean) to a border-touching ground cluster
# is a band of the same gradient rather than an ink. Added 2026-09-04 for
# entropism's login: its radial lift quantised as #151209 at the edge and
# #18160d in the middle, and the middle band -- 33% of the frame, never
# touching the border -- was fitted as a 1044x586 "chamfer" that no
# implementation could draw, failing the screen at 28% while every real
# widget matched. 10 is well under the darkest ink-on-ground distance in
# the four eras (neokitsch's #302418 strands on #0a0a0a are ~48 away).
GROUND_NEIGHBOUR_DIST = 10.0
# Ink clusters below this coverage are resampling noise, not a design colour.
MIN_INK_COVERAGE = 0.002
# The existing small-gap cleanup spans two pixels on either side. Changes
# within that square's diagonal are raster-scale evidence, not a new lobe.
INK_CLOSE_SIZE = 5
RASTER_RADIUS = math.sqrt(2) * (INK_CLOSE_SIZE // 2)
# Existing straight-side evidence used to contradict a missing corner.
STRAIGHT_SIDE_SUPPORT = 0.8
# spec_diff pairs ink families only when RGB distance is below 110. A
# candidate pixel farther away than that from every source center cannot
# represent any source family under the same comparison rule.
MAX_REFERENCE_RGB_DISTANCE = 110.0


# --------------------------------------------------------------------------
# palette
# --------------------------------------------------------------------------

def quantize(rgb, k, iters=12):
    """k-means with deterministic seeding. Returns (labels, centres)."""
    flat = rgb.reshape(-1, 3).astype(np.float32)
    # Seed from the most-populated 32-level bins, spread out so the seeds do
    # not all land inside one dominant colour.
    coarse = (flat // 32).astype(np.int32)
    keys = coarse[:, 0] * 64 + coarse[:, 1] * 8 + coarse[:, 2]
    uniq, counts = np.unique(keys, return_counts=True)
    seeds = []
    for key in uniq[np.argsort(-counts)]:
        c = np.array([key // 64, (key // 8) % 8, key % 8], np.float32) * 32 + 16
        if all(np.linalg.norm(c - s) > 40 for s in seeds):
            seeds.append(c)
        if len(seeds) == k:
            break
    while len(seeds) < k:  # degenerate (near-monochrome) input
        seeds.append(seeds[-1] + 1.0)
    centres = np.stack(seeds)

    # Cluster on a strided subsample for speed, then assign every pixel once.
    sample = flat[::7]
    for _ in range(iters):
        d = ((sample[:, None, :] - centres[None, :, :]) ** 2).sum(2)
        lab = d.argmin(1)
        for i in range(k):
            m = lab == i
            if m.any():
                centres[i] = sample[m].mean(0)
    d = ((flat[:, None, :] - centres[None, :, :]) ** 2).sum(2)
    labels = d.argmin(1).reshape(rgb.shape[:2])
    return labels, centres


def reference_palette(spec, k):
    """Read an extractor spec as fixed candidate colour centers and roles.

    A source/candidate comparison should measure both images in the same
    colour space. Re-running k-means on the candidate lets a large backdrop
    consume the clusters that represented small source inks. Reject malformed
    or incompatible specs instead of silently falling back to a new palette.
    """
    entries = spec.get("palette", [])
    if len(entries) != k or sorted(e.get("index") for e in entries) != list(range(k)):
        raise ValueError("reference palette must have --colors indexed entries")
    entries = sorted(entries, key=lambda e: e["index"])
    if any(e.get("role") not in ("ink", "ground") for e in entries):
        raise ValueError("reference palette has an invalid ground/ink role")
    centres = np.asarray([e.get("rgb") for e in entries], dtype=np.float32)
    if centres.shape != (k, 3) or not np.isfinite(centres).all() or not ((0 <= centres) & (centres <= 255)).all():
        raise ValueError("reference palette needs three RGB values per entry")
    return centres, [e["role"] for e in entries]


def assign_palette(rgb, centres):
    """Assign pixels to fixed centers, leaving unsupported colours unassigned."""
    flat = rgb.reshape(-1, 3)
    labels = np.empty(len(flat), dtype=np.int32)
    distances2 = np.empty(len(flat), dtype=np.float32)
    # Bounded temporary arrays matter for large native-resolution images.
    for start in range(0, len(flat), 100000):
        part = flat[start:start + 100000]
        d = ((part[:, None, :] - centres[None, :, :]) ** 2).sum(2)
        owner = d.argmin(1)
        labels[start:start + len(part)] = owner
        distances2[start:start + len(part)] = d[np.arange(len(part)), owner]
    residual = residual_summary(distances2, labels, len(centres))
    labels[distances2 >= MAX_REFERENCE_RGB_DISTANCE ** 2] = -1
    return labels.reshape(rgb.shape[:2]), residual


def residual_summary(distances2, labels, ncentres):
    """Coverage that lies far from its nearest palette center."""
    outside = distances2 >= MAX_REFERENCE_RGB_DISTANCE ** 2
    per_bin = np.bincount(labels[outside], minlength=ncentres)
    return {
        "rgb_distance_at_least_110": round(float(outside.mean()), 5),
        "outside_by_nearest_bin": [round(int(n) / len(labels), 5) for n in per_bin],
    }


def hexof(c):
    return "#%02x%02x%02x" % tuple(int(round(max(0, min(255, v)))) for v in c)


def palette_spec(labels, centres, h, w, roles=None, rgb=None):
    """Per-cluster coverage, border share, and the ground/ink verdict."""
    border = np.zeros((h, w), bool)
    border[0, :] = border[-1, :] = True
    border[:, 0] = border[:, -1] = True
    nborder = border.sum()

    out = []
    for i, c in enumerate(centres):
        m = labels == i
        share = (m & border).sum() / nborder
        # Fixed centers define bin membership; the entry's colour still
        # describes pixels actually drawn by this image. This keeps hue
        # mistakes visible to spec_diff's colour-family pairing.
        measured = rgb[m].mean(0) if rgb is not None and m.any() else (c if rgb is None else None)
        out.append({
            "index": int(i),
            "hex": hexof(measured) if measured is not None else None,
            "rgb": [int(round(v)) for v in measured] if measured is not None else None,
            "coverage": round(float(m.mean()), 5),
            "border_share": round(float(share), 4),
            "role": (roles[i] if roles is not None else
                     ("ground" if share > GROUND_BORDER_SHARE else "ink")),
        })
    # Second pass: inner bands of a ground gradient. Only border-touching
    # clusters seed this, so two near-identical inks never pull each other
    # into the ground.
    if roles is None:
        seeds = [np.array(e["rgb"], float) for e in out if e["role"] == "ground"]
        for e in out:
            if e["role"] == "ink" and any(
                np.linalg.norm(np.array(e["rgb"], float) - s) <= GROUND_NEIGHBOUR_DIST
                for s in seeds
            ):
                e["role"] = "ground"
    out.sort(key=lambda e: -e["coverage"])
    return out


# --------------------------------------------------------------------------
# shape templates
# --------------------------------------------------------------------------

def t_rect(shape, p):
    h, w = shape
    x0, y0, x1, y1 = p
    m = np.zeros((h, w), bool)
    m[max(0, y0):max(0, y1), max(0, x0):max(0, x1)] = True
    return m


def t_diamond(shape, p):
    h, w = shape
    cx, cy, d = p
    if d <= 0:
        return np.zeros((h, w), bool)
    ys, xs = np.ogrid[:h, :w]
    return (np.abs(xs - cx) + np.abs(ys - cy)) <= d


def t_chamfer(shape, p):
    """Axis-aligned rect with diagonal cuts. `corners` is a 4-bit mask over
    (top-left, top-right, bottom-right, bottom-left)."""
    h, w = shape
    x0, y0, x1, y1, cut, corners = p
    m = t_rect(shape, (x0, y0, x1, y1))
    if cut <= 0 or not corners:
        return m
    ys, xs = np.ogrid[:h, :w]
    if corners & 1:
        m &= ~(((xs - x0) + (ys - y0)) < cut)
    if corners & 2:
        m &= ~((((x1 - 1) - xs) + (ys - y0)) < cut)
    if corners & 4:
        m &= ~((((x1 - 1) - xs) + ((y1 - 1) - ys)) < cut)
    if corners & 8:
        m &= ~(((xs - x0) + ((y1 - 1) - ys)) < cut)
    return m


def score(template, ink, cell):
    """Occlusion-aware IoU.

    hit   template pixels that are ink (an overlapped neighbour still counts:
          the shape genuinely is there, it is just drawn under something)
    miss  template pixels that are not ink  -> template too large / wrong class
    extra ink in this component's own Voronoi cell the template misses
          -> template too small / wrong class
    """
    hit = int((template & ink).sum())
    miss = int((template & ~ink).sum())
    extra = int((cell & ~template).sum())
    denom = hit + miss + extra
    return (hit / denom) if denom else 0.0


def fit_shape(ink, cell, dist, canvas, raw=None, observed=None):
    """Best (class, params, iou, bbox) for one component."""
    ys, xs = np.where(cell)
    if len(xs) == 0:
        return None
    x0, x1 = int(xs.min()), int(xs.max()) + 1
    y0, y1 = int(ys.min()), int(ys.max()) + 1
    best = None
    observed = cell if observed is None else observed
    oy, ox = np.where(observed)
    observed_box = (ox.min(), oy.min(), ox.max(), oy.max())
    boundary_distance = None

    def supported_diamond(cx, cy, radius):
        """Do not invent wide tips around a rectangular material patch.

        A Voronoi cell may cover only part of an overlapping diamond, so
        the extent belongs to its whole observed component. Occluded tips
        may still be inferred when two diagonal sides retain independently
        measured boundary support; a large rectangular fill cannot supply it.
        """
        nonlocal boundary_distance
        left, top, right, bottom = observed_box
        if (cx - radius >= left - RASTER_RADIUS and
                cy - radius >= top - RASTER_RADIUS and
                cx + radius <= right + RASTER_RADIUS and
                cy + radius <= bottom + RASTER_RADIUS):
            return True
        if boundary_distance is None:
            boundary = observed & ~ndimage.binary_erosion(observed)
            boundary_distance = ndimage.distance_transform_edt(~boundary)
        tips = [(cx, cy - radius), (cx + radius, cy),
                (cx, cy + radius), (cx - radius, cy)]
        supported = 0
        for a, b in zip(tips, tips[1:] + tips[:1]):
            points = np.rint(np.linspace(a, b, max(2, int(radius * 2)))).astype(int)
            inside = ((points[:, 0] >= 0) & (points[:, 0] < canvas[1]) &
                      (points[:, 1] >= 0) & (points[:, 1] < canvas[0]))
            near = np.zeros(len(points), bool)
            valid = points[inside]
            near[inside] = boundary_distance[valid[:, 1], valid[:, 0]] <= RASTER_RADIUS
            supported += near.mean() >= MIN_SHAPE_IOU
        return supported >= 2

    def consider(cls, template, params, bbox):
        nonlocal best
        s = score(template, ink, cell)
        if best is None or s > best["iou"]:
            best = {"class": cls, "iou": round(float(s), 4),
                    "bbox": [int(v) for v in bbox], "params": params}

    # rect, with a small local search on each edge
    for dx0 in (0, -2, 2):
        for dy0 in (0, -2, 2):
            p = (x0 + dx0, y0 + dy0, x1 - dx0, y1 - dy0)
            consider("rect", t_rect(canvas, p), {},
                     (p[0], p[1], p[2] - p[0], p[3] - p[1]))

    # diamond, seeded from the distance-transform peak inside the cell
    dsub = np.where(cell, dist, 0)
    if dsub.max() > 3:
        py, px = np.unravel_index(dsub.argmax(), dsub.shape)
        d0 = dsub.max() * math.sqrt(2)
        for dd in (-6, -3, 0, 3, 6):
            for ox in (-4, 0, 4):
                for oy in (-4, 0, 4):
                    cx, cy, d = px + ox, py + oy, d0 + dd
                    if not supported_diamond(cx, cy, d):
                        continue
                    consider("diamond", t_diamond(canvas, (cx, cy, d)),
                             {"cx": int(cx), "cy": int(cy), "half_diagonal": round(float(d), 1)},
                             (cx - d, cy - d, 2 * d, 2 * d))

    # chamfered rect: which corners, and how deep
    short = min(x1 - x0, y1 - y0)
    for corners in range(1, 16):
        for frac in (0.18, 0.26, 0.34):
            cut = short * frac
            # A cut cannot remove a corner when the original image still
            # draws that corner's straight side. Hole filling can make an
            # outlined panel appear cut because rows of text break its top
            # interior into separate distance-transform cells. Test the raw
            # perimeter, rather than the hole-filled fitting mask.
            if raw is not None and cut >= 8:
                top = raw[y0 + 2:min(y1, y0 + int(cut) - 2)]
                bottom = raw[max(y0, y1 - int(cut) + 2):y1 - 2]
                sides = ((1, top, x0), (2, top, x1 - 1),
                         (4, bottom, x1 - 1), (8, bottom, x0))
                if any(corners & bit and side.size and
                       side[:, max(0, x - 1):min(canvas[1], x + 2)].any(axis=1).mean() > STRAIGHT_SIDE_SUPPORT
                       for bit, side, x in sides):
                    continue
            p = (x0, y0, x1, y1, cut, corners)
            names = [n for b, n in ((1, "tl"), (2, "tr"), (4, "br"), (8, "bl")) if corners & b]
            consider("chamfer", t_chamfer(canvas, p),
                     {"corners": names, "cut": round(float(cut), 1)},
                     (x0, y0, x1 - x0, y1 - y0))

    # a very thin rect is a rule, not a panel
    w, h = x1 - x0, y1 - y0
    if best and best["class"] == "rect" and (w > 20 * max(h, 1) or h > 20 * max(w, 1)):
        best["class"] = "rule"
    if best:
        best["template_class"] = best["class"]
        if best["iou"] < MIN_SHAPE_IOU:
            best["class"] = "blob"
    return best


# --------------------------------------------------------------------------
# component extraction
# --------------------------------------------------------------------------

def observed_mask(cell):
    """Lossless, cropped split-cell evidence for cross-render identity checks.

    Rows are packed independently with little-endian bit order so a reader
    can compare canvas-aligned pixels without relying on either fitted bbox.
    """
    ys, xs = np.where(cell)
    x0, y0 = int(xs.min()), int(ys.min())
    crop = cell[y0:int(ys.max()) + 1, x0:int(xs.max()) + 1]
    packed = np.packbits(crop, axis=1, bitorder="little").tobytes()
    return {"origin": [x0, y0], "size": [crop.shape[1], crop.shape[0]],
            "encoding": "zlib-packbits-little-row",
            "data": base64.b64encode(zlib.compress(packed, 9)).decode("ascii")}

def split_blob(mask, dist, peak_floor_frac=0.55, win_frac=2.0):
    """Nearest-peak (Voronoi) split of a blob of overlapping convex shapes.

    Two overlapping equal diamonds are separated along their perpendicular
    bisector, which is exactly the boundary a human would draw.
    """
    # The window is the shape's own width (twice its inradius) so that two
    # lobes must be a full shape apart to count as separate; the floor keeps
    # saddle points in an overlap region from registering as a third shape.
    md = dist.max()
    win = max(9, int(md * win_frac) | 1)
    peaks = (dist == ndimage.maximum_filter(dist, size=win)) & (dist > md * peak_floor_frac)
    plab, pn = ndimage.label(peaks, structure=np.ones((3, 3)))
    if pn <= 1:
        return [mask], pn
    cents = np.array(ndimage.center_of_mass(peaks, plab, range(1, pn + 1)))
    # A tiny nick in an otherwise continuous distance ridge makes several
    # local maxima. Keep only persistent maxima: a smaller summit belongs
    # to a taller one when the connecting saddle is within the cleanup's
    # raster radius. Genuine touching lobes have a deeper intervening neck.
    heights = np.asarray(ndimage.maximum(dist, plab, range(1, pn + 1)))
    ys, xs = np.where(mask)
    y0, y1, x0, x1 = ys.min(), ys.max() + 1, xs.min(), xs.max() + 1
    local_mask, local_dist = mask[y0:y1, x0:x1], dist[y0:y1, x0:x1]
    representatives = np.asarray(ndimage.maximum_position(dist, plab, range(1, pn + 1)))
    representatives -= (y0, x0)
    keep = np.ones(pn, bool)
    order = sorted(range(pn), key=lambda i: (-heights[i], i))
    for position, i in enumerate(order):
        if not keep[i]:
            continue
        level = max(np.nextafter(0.0, 1.0), heights[i] - RASTER_RADIUS)
        connected, _ = ndimage.label(local_mask & (local_dist >= level),
                                    structure=np.ones((3, 3)))
        # Use actual peak pixels, not a plateau's centroid (which can lie
        # in a hole or outside a curved ridge).
        labels = connected[representatives[:, 0], representatives[:, 1]]
        if any(keep[j] and labels[j] == labels[i] for j in order[:position]):
            keep[i] = False
    cents = cents[keep]
    pn = len(cents)
    if pn <= 1:
        return [mask], pn
    ys, xs = np.where(mask)
    pts = np.stack([ys, xs], 1)
    d = ((pts[:, None, :] - cents[None, :, :]) ** 2).sum(2)
    owner = d.argmin(1)
    cells = []
    for i in range(pn):
        c = np.zeros_like(mask)
        sel = owner == i
        c[ys[sel], xs[sel]] = True
        cells.append(c)
    return cells, pn


def supported_whole_outline(shape, observed):
    """Prefer one measured panel over internal texture's distance peaks.

    Each straight or diagonal polygon edge needs its own observed contour
    support. A good filled-area score alone cannot turn overlapping lobes
    into a single panel. Use the same side evidence, raster uncertainty and
    small local alignment search as the shape fitter.
    """
    if shape is None or shape["class"] not in ("rect", "chamfer"):
        return False
    x, y, w, h = shape["bbox"]
    right, bottom = x + w - 1, y + h - 1
    params = shape["params"]
    cut = params.get("cut", 0)
    corners = params.get("corners", [])
    tl, tr, br, bl = [cut if name in corners else 0
                      for name in ("tl", "tr", "br", "bl")]
    vertices = [(x + tl, y), (right - tr, y), (right, y + tr),
                (right, bottom - br), (right - br, bottom),
                (x + bl, bottom), (x, bottom - bl), (x, y + tl)]
    edges = []
    for a, b in zip(vertices, vertices[1:] + vertices[:1]):
        length = math.dist(a, b)
        if length >= 1:
            edges.append(np.rint(np.linspace(a, b, max(2, int(length * 2)))).astype(int))
    boundary = observed & ~ndimage.binary_erosion(observed)
    distance = ndimage.distance_transform_edt(~boundary)
    height, width = observed.shape
    for dx in (0, -2, 2):
        for dy in (0, -2, 2):
            supported = True
            for edge in edges:
                points = edge + (dx, dy)
                inside = ((points[:, 0] >= 0) & (points[:, 0] < width) &
                          (points[:, 1] >= 0) & (points[:, 1] < height))
                near = np.zeros(len(points), bool)
                valid = points[inside]
                near[inside] = distance[valid[:, 1], valid[:, 0]] <= RASTER_RADIUS
                if near.mean() <= STRAIGHT_SIDE_SUPPORT:
                    supported = False
                    break
            if supported:
                return True
    return False


def text_runs(small_mask, rgb):
    """Group glyph-sized components into lines by horizontal dilation."""
    joined = ndimage.binary_dilation(small_mask, np.ones((3, 25)))
    lab, n = ndimage.label(joined)
    runs = []
    for i, sl in enumerate(ndimage.find_objects(lab), start=1):
        m = (lab[sl] == i) & small_mask[sl]
        area = int(m.sum())
        if area < 40:
            continue
        ys, xs = sl
        px = rgb[sl][m]
        runs.append({
            "bbox": [int(xs.start), int(ys.start),
                     int(xs.stop - xs.start), int(ys.stop - ys.start)],
            "ink": hexof(px.mean(0)),
            "ink_area": area,
        })
    runs.sort(key=lambda r: (r["bbox"][1], r["bbox"][0]))
    return runs


def striped_body(mask, support):
    """Whether a supported body contains repeated fine horizontal/vertical grain.

    Require at least nine strands, mostly 2–7 px apart, across most of the
    body. Text, isolated rules and the two edges of a shadow are insufficient.
    The range includes resampling of the traced 2.1/2.7 px veneer pitches.
    """
    for pixels, body in ((mask, support), (mask.T, support.T)):
        eligible = body.sum(axis=1) >= 32
        if eligible.sum() < 32:
            continue
        repeated = 0
        for row in pixels[eligible]:
            starts = np.flatnonzero(np.diff(row.astype(np.int8), prepend=0) == 1)
            gaps = np.diff(starts)
            if len(gaps) >= 8 and np.mean((gaps >= 2) & (gaps <= 7)) >= 0.7:
                repeated += 1
        if repeated / eligible.sum() >= 0.7:
            return True
    return False


def repair_striped_regions(raw_masks, ink_masks, colours):
    """Recover grain families inside a separately observed solid body.

    K-means can assign antialiased grain to a sparse palette family. A 5 px
    close then leaves long slots, even though another similar ink independently
    measures the complete body. Fill only that measured support, and only when
    the raw family has repeated fine strands there. No global close, convex
    hull or merging across background gaps is involved. Use the original masks
    throughout so one repair cannot grow the support for another.
    """
    repaired = [mask.copy() for mask in ink_masks]
    for anchor, solid in enumerate(ink_masks):
        labels, _ = ndimage.label(solid)
        for label, slices in enumerate(ndimage.find_objects(labels), start=1):
            body = labels[slices] == label
            if body.sum() < MIN_SHAPE_AREA or body.mean() < 0.7:
                continue
            for target, raw in enumerate(raw_masks):
                if target == anchor or np.linalg.norm(colours[target] - colours[anchor]) > 50:
                    continue
                grain = raw[slices] & body
                coverage = grain.sum() / body.sum()
                if 0.05 <= coverage <= 0.8 and striped_body(grain, body):
                    repaired[target][slices] |= body
    return [ndimage.binary_fill_holes(mask) for mask in repaired]


def components(raw, ink, canvas, family):
    """Fit every large component of one ink family.

    `ink` is the hole-filled mask and drives shape fitting: a widget drawn as
    an outline plus an inset rule reads as a ring around a core otherwise, and
    both the distance transform and the component count come out wrong. `raw`
    is the unfilled mask and drives the text-run grouper, so outline glyphs
    are not solidified into blocks.
    """
    lab, n = ndimage.label(ink)
    sizes = ndimage.sum(ink, lab, range(1, n + 1))
    big_ids = [i + 1 for i, sz in enumerate(sizes) if sz >= MIN_SHAPE_AREA]
    small = raw & ~np.isin(lab, big_ids)

    out = []
    for bid in big_ids:
        blob = lab == bid
        dist = ndimage.distance_transform_edt(blob)
        cells, _ = split_blob(blob, dist)
        whole = None
        if len(cells) > 1:
            whole = fit_shape(ink, blob, dist, canvas, raw, blob)
            if supported_whole_outline(whole, blob):
                cells = [blob]
            else:
                whole = None
        for cell in cells:
            if cell.sum() < MIN_SHAPE_AREA // 2:
                continue
            fit = whole if whole is not None else fit_shape(ink, cell, dist, canvas, raw, blob)
            if fit:
                fit["ink"] = family
                fit["area"] = int(cell.sum())
                fit["observed_mask"] = observed_mask(cell)
                x, y, w, _ = fit["bbox"]
                # Keep this extraction-only evidence for distinguishing a
                # partial border from a real panel docked to its parent.
                if w > 10:
                    top = raw[max(0, y - 1):min(canvas[0], y + 2),
                              max(0, x + 5):min(canvas[1], x + w - 5)]
                    fit["_top_edge_support"] = float(top.any(axis=0).mean()) if top.size else 0.0
                out.append(fit)
    return out, small


def collapse_shared_outlines(shapes):
    """Keep one supported fit for coincident rectangular border fragments.

    Photographs often quantize adjacent pixels of one thin border into
    several colour families. If the left, right and bottom edges coincide,
    the shorter vertical detection is a fragment only when its alleged top
    edge has no measured ink. A docked panel with its own top edge survives.
    """
    ranked = sorted(shapes, key=lambda s: -s["bbox"][2] * s["bbox"][3])
    kept = []
    for shape in ranked:
        x, y, w, h = shape["bbox"]
        duplicate = False
        if shape["class"] in ("rect", "chamfer") and w * h >= 10000:
            for parent in kept:
                if parent["class"] not in ("rect", "chamfer"):
                    continue
                px, py, pw, ph = parent["bbox"]
                if (abs(x - px) <= 3 and abs(x + w - px - pw) <= 3 and
                        abs(y + h - py - ph) <= 3 and py <= y + 3 and
                        h >= ph * 0.5 and
                        shape.get("_top_edge_support", 1.0) < 0.2):
                    duplicate = True
                    break
        if not duplicate:
            kept.append(shape)
    return kept


def occupancy(mask, h, w, rows, cols):
    """Mean of `mask` over a rows x cols grid, for any canvas size.

    This used to be `mask.reshape(rows, h // rows, cols, w // cols).mean()`,
    which needs the canvas to divide evenly by the grid. 1600x900 does;
    the bar's 1600x220 does not, and G2i compares a bar render against a
    bar SVG at their own size rather than stretching both to 900 tall
    (a 4x vertical stretch turns every diamond and chamfer into
    something the shape templates no longer fit, on both sides at once,
    which loses the inventory the gate is there to compare). Binning by
    index instead is the same arithmetic whenever the old form applied —
    for 1600x900 the bins are exactly the 20x20 blocks it took — so no
    existing spec moves.
    """
    ri = (np.arange(h) * rows) // h
    ci = (np.arange(w) * cols) // w
    cell = (ri[:, None] * cols + ci[None, :]).ravel()
    n = rows * cols
    total = np.bincount(cell, weights=mask.ravel().astype(np.float64), minlength=n)
    count = np.bincount(cell, minlength=n)
    return (total / np.maximum(count, 1)).reshape(rows, cols)


def extract(path, canvas_wh, k=8, palette_from=None):
    w, h = canvas_wh
    im = Image.open(path).convert("RGB")
    if im.size != canvas_wh:
        im = im.resize(canvas_wh, Image.Resampling.LANCZOS)
    rgb = np.asarray(im).astype(np.float32)

    if palette_from is None:
        labels, centres = quantize(rgb, k)
        roles = None
    else:
        if palette_from.get("canvas") != [w, h]:
            raise ValueError("reference palette canvas must match candidate canvas")
        centres, roles = reference_palette(palette_from, k)
        labels, residual = assign_palette(rgb, centres)
    pal = palette_spec(labels, centres, h, w, roles, rgb if roles is not None else None)

    # Segment each ink colour separately rather than unioning them. A drop
    # shadow and the shape casting it are different elements: merged into one
    # mask they form a single lopsided blob whose centroid sits on neither.
    shapes, smalls = [], []
    inkmask = np.zeros((h, w), bool)
    entries = [entry for entry in pal
               if entry["role"] == "ink" and entry["coverage"] >= MIN_INK_COVERAGE]
    masks = [labels == entry["index"] for entry in entries]
    closed = [ndimage.binary_closing(mask, np.ones((INK_CLOSE_SIZE, INK_CLOSE_SIZE)))
              for mask in masks]
    filled = [ndimage.binary_fill_holes(mask) for mask in closed]
    repaired = repair_striped_regions(
        masks, filled, [centres[entry["index"]] for entry in entries])
    for entry, raw, ink in zip(entries, closed, repaired):
        inkmask |= ink
        # Family placement: where this colour sits on the canvas, as an 80x45
        # occupancy grid. Unlike the per-component shape fits, this survives
        # rotated and translucent geometry — two renders of the same design
        # fragment differently but ink the same cells.
        cw_, ch_ = 80, 45
        occ = occupancy(ink, h, w, ch_, cw_) >= 0.15
        ys, xs = np.where(ink)
        if len(xs):
            entry["ink_bbox"] = [int(xs.min()), int(ys.min()),
                                 int(xs.max() - xs.min() + 1), int(ys.max() - ys.min() + 1)]
            entry["ink_centroid"] = [round(float(xs.mean()), 1), round(float(ys.mean()), 1)]
        entry["ink_grid"] = ["".join("1" if c else "0" for c in row) for row in occ]
        found, small = components(raw, ink, (h, w), entry["hex"])
        shapes.extend(found)
        smalls.append(small)

    shapes = collapse_shared_outlines(shapes)
    for shape in shapes:
        shape.pop("_top_edge_support", None)
    shapes.sort(key=lambda s: (s["class"], s["bbox"][1], s["bbox"][0]))
    for i, s in enumerate(shapes, start=1):
        s["id"] = "%s-%02d" % (s["class"], i)

    small = smalls[0] if smalls else np.zeros((h, w), bool)
    for m in smalls[1:]:
        small |= m

    spec = {
        "source": os.path.relpath(path),
        "canvas": [w, h],
        "palette": pal,
        "shapes": shapes,
        "text_runs": text_runs(small, rgb),
    }
    if roles is not None:
        spec["palette_mode"] = "source_anchored"
        spec["palette_residual"] = residual
    return spec, inkmask, im


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("image")
    ap.add_argument("-o", "--out", help="write JSON here (default: stdout)")
    ap.add_argument("--canvas", default="1600x900")
    ap.add_argument("--colors", type=int, default=8, help="palette size (default 8)")
    ap.add_argument("--palette-from", metavar="SOURCE.json",
                    help="assign to source spec palette; keep its ground/ink roles")
    ap.add_argument("--crops", metavar="DIR",
                    help="write a zoom crop per shape, for visual inspection")
    ap.add_argument("--debug", metavar="DIR", help="write the ink mask")
    a = ap.parse_args()

    cw, ch = (int(v) for v in a.canvas.lower().split("x"))
    reference = None
    if a.palette_from:
        with open(a.palette_from) as fh:
            reference = json.load(fh)
    spec, ink, im = extract(a.image, (cw, ch), a.colors, reference)

    text = json.dumps(spec, indent=2)
    if a.out:
        with open(a.out, "w") as fh:
            fh.write(text + "\n")
        print("wrote %s: %d shapes, %d text runs, %d palette entries"
              % (a.out, len(spec["shapes"]), len(spec["text_runs"]), len(spec["palette"])),
              file=sys.stderr)
    else:
        print(text)

    if a.debug:
        os.makedirs(a.debug, exist_ok=True)
        Image.fromarray((ink * 255).astype(np.uint8)).save(
            os.path.join(a.debug, "ink-mask.png"))
    if a.crops:
        os.makedirs(a.crops, exist_ok=True)
        for s in spec["shapes"]:
            x, y, w, h = s["bbox"]
            pad = 24
            box = (max(0, x - pad), max(0, y - pad),
                   min(cw, x + w + pad), min(ch, y + h + pad))
            if box[2] <= box[0] or box[3] <= box[1]:
                continue
            im.crop(box).save(os.path.join(a.crops, s["id"] + ".png"))


if __name__ == "__main__":
    main()
