#!/usr/bin/env python3
"""spec_diff.py — diff two `extract_spec.py` specs as shape inventories.

This is the gate `compare_ref.py` could not be. Grid correlation answers
"is the mass in roughly the right places", which a trace can satisfy while
drawing entirely the wrong widgets. This answers "does the candidate draw
the same things the source does", which is what a trace has to mean.

    spec_diff.py SOURCE.json CANDIDATE.json [--match-iou 0.3] [--strict]

Shapes are matched greedily by bounding-box IoU, best pair first, so the
result does not depend on input order. A matched pair whose class differs is
reported as a reclass, not a match: a diamond redrawn as a rectangle is a
trace error even though it occupies the same box. In ordinary matching,
the one exception is
rect <-> chamfer, which is counted as a match: that corner detail is what
a photo's glow erases, so it separates photo from render, not right from
wrong. Unmatched templates may also recover an unclassified component
with compatible latent template/ink/corners and at least .95 observed-mask
IoU at the original canvas coordinates; legacy specs lack this evidence.

Two gate modes, chosen with --gate:

  shapes (default)  the shape-inventory gate described above. Reliable for
                    axis-aligned design languages, where the extractor's
                    templates fit whole widgets. Fails when a class is
                    missing outright or under --min-area-match of source
                    shape area is matched.
  inks              per-ink-family placement. Rotated, overlapping, or
                    translucent geometry (kitsch's fans, neokitsch's
                    cascades) fragments differently on a photo and on a
                    clean render, so fragment identity is not a stable
                    invariant there — but where each colour sits on the
                    canvas is. Families are paired by colour, their 80x45
                    occupancy grids compared by IoU; fails when the
                    coverage-weighted IoU is under --min-ink-iou or a major
                    source family has no counterpart. The inventory is
                    still printed for information.

Exit status is 1 on gate failure, or (with --strict, shapes mode) on any
count difference.
"""

import argparse
import base64
import binascii
import json
import math
import sys
import zlib


# A class-unstable pair must agree in the observed pixels, not merely its
# fitted box. This is deliberately much stricter than the template cutoff.
MIN_OBSERVED_MASK_IOU = 0.95


def decoded_mask(shape, canvas):
    """Validate and decode one lossless observed component mask, if present.

    Old specs have neither evidence field and retain the old gate behavior.
    An incomplete or malformed new record is an error, never a fallback.
    """
    has_mask = "observed_mask" in shape
    has_template = "template_class" in shape
    if not has_mask and not has_template:
        return None
    if not (has_mask and has_template):
        raise ValueError("incomplete component evidence")
    if shape["template_class"] not in ("rect", "chamfer", "diamond", "rule"):
        raise ValueError("invalid latent template class")
    if shape["class"] != "blob" and shape["template_class"] != shape["class"]:
        raise ValueError("classified shape disagrees with latent template")
    mask = shape["observed_mask"]
    if not isinstance(mask, dict) or mask.get("encoding") != "zlib-packbits-little-row":
        raise ValueError("invalid component mask encoding")
    origin, size = mask.get("origin"), mask.get("size")
    if (not isinstance(origin, list) or not isinstance(size, list) or
            len(origin) != 2 or len(size) != 2 or
            any(type(v) is not int for v in origin + size)):
        raise ValueError("invalid component mask bounds")
    x, y = origin
    w, h = size
    if w <= 0 or h <= 0 or x < 0 or y < 0 or x + w > canvas[0] or y + h > canvas[1]:
        raise ValueError("component mask extends outside canvas")
    encoded = mask.get("data")
    if not isinstance(encoded, str):
        raise ValueError("missing component mask data")
    try:
        compressed = base64.b64decode(encoded, validate=True)
        expected = ((w + 7) // 8) * h
        inflater = zlib.decompressobj()
        raw = inflater.decompress(compressed, expected + 1)
    except (ValueError, binascii.Error, zlib.error) as error:
        raise ValueError("invalid component mask data") from error
    if (len(raw) != expected or not inflater.eof or inflater.unused_data or
            inflater.unconsumed_tail):
        raise ValueError("component mask data length mismatch")
    stride = (w + 7) // 8
    rows = [int.from_bytes(raw[i * stride:(i + 1) * stride], "little")
            for i in range(h)]
    if any(row >> w for row in rows):
        raise ValueError("nonzero component mask padding")
    count = sum(row.bit_count() for row in rows)
    if type(shape.get("area")) is not int or count == 0 or count != shape["area"]:
        raise ValueError("component mask area mismatch")
    return (x, y, w, h, rows, count)


def observed_iou(a, b):
    ax, ay, _, ah, ar, acount = a
    bx, by, _, bh, br, bcount = b
    hit = 0
    for y in range(max(ay, by), min(ay + ah, by + bh)):
        hit += ((ar[y - ay] << ax) & (br[y - by] << bx)).bit_count()
    return hit / (acount + bcount - hit)


def compatible_template(source, candidate):
    a, b = source["class"], candidate["template_class"]
    if a != b and frozenset((a, b)) != frozenset(("rect", "chamfer")):
        return False
    if a == b == "chamfer":
        sp, cp = source.get("params"), candidate.get("params")
        if not isinstance(sp, dict) or not isinstance(cp, dict):
            return False
        sc, cc = sp.get("corners"), cp.get("corners")
        def valid(corners):
            return (isinstance(corners, list) and bool(corners) and
                    all(type(c) is str and c in ("tl", "tr", "br", "bl")
                        for c in corners) and len(set(corners)) == len(corners))
        return valid(sc) and valid(cc) and set(sc) == set(cc)
    return True


def compatible_ink(source, candidate):
    def rgb(shape):
        value = shape.get("ink")
        if not isinstance(value, str) or len(value) != 7 or value[0] != "#":
            return None
        try:
            return [int(value[i:i + 2], 16) for i in (1, 3, 5)]
        except ValueError:
            return None
    a, b = rgb(source), rgb(candidate)
    return a is not None and b is not None and math.dist(a, b) < 110


def recover_class_unstable(missing, blobs, source_masks, candidate_masks, bbox_threshold):
    """One-to-one, strict pixel-backed recovery of unmatched template/blob pairs."""
    options = []
    for i, source in enumerate(missing):
        sm = source_masks.get(id(source))
        if sm is None:
            continue
        for j, candidate in enumerate(blobs):
            cm = candidate_masks.get(id(candidate))
            if (cm is None or not compatible_template(source, candidate) or
                    not compatible_ink(source, candidate)):
                continue
            box_score = iou(source["bbox"], candidate["bbox"])
            if box_score < bbox_threshold:
                continue
            pixel_score = observed_iou(sm, cm)
            if pixel_score >= MIN_OBSERVED_MASK_IOU:
                options.append((pixel_score, box_score, i, j))
    options.sort(key=lambda item: (-item[0], -item[1], item[2], item[3]))
    used_source, used_candidate, recovered = set(), set(), []
    for pixel_score, box_score, i, j in options:
        if i not in used_source and j not in used_candidate:
            used_source.add(i)
            used_candidate.add(j)
            recovered.append((box_score, missing[i], blobs[j], pixel_score))
    return recovered, [s for i, s in enumerate(missing) if i not in used_source]


def iou(a, b):
    ax, ay, aw, ah = a
    bx, by, bw, bh = b
    x0, y0 = max(ax, bx), max(ay, by)
    x1, y1 = min(ax + aw, bx + bw), min(ay + ah, by + bh)
    if x1 <= x0 or y1 <= y0:
        return 0.0
    inter = (x1 - x0) * (y1 - y0)
    return inter / (aw * ah + bw * bh - inter)


def centre(b):
    return (b[0] + b[2] / 2.0, b[1] + b[3] / 2.0)


def match(src, cand, thresh):
    """Greedy best-IoU matching. Returns (pairs, unmatched_src, unmatched_cand)."""
    cands = []
    for i, s in enumerate(src):
        for j, c in enumerate(cand):
            v = iou(s["bbox"], c["bbox"])
            if v >= thresh:
                cands.append((v, i, j))
    cands.sort(key=lambda t: (-t[0], t[1], t[2]))
    used_s, used_c, pairs = set(), set(), []
    for v, i, j in cands:
        if i in used_s or j in used_c:
            continue
        used_s.add(i)
        used_c.add(j)
        pairs.append((v, src[i], cand[j]))
    return (pairs,
            [s for i, s in enumerate(src) if i not in used_s],
            [c for j, c in enumerate(cand) if j not in used_c])


def area(shapes):
    return sum(s["bbox"][2] * s["bbox"][3] for s in shapes)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("source")
    ap.add_argument("candidate")
    ap.add_argument("--match-iou", type=float, default=0.3)
    ap.add_argument("--min-area-match", type=float, default=0.60,
                    help="fail below this share of source shape area matched")
    ap.add_argument("--min-class-share", type=float, default=0.10,
                    help="a class holding under this share of source shape "
                         "area may be absent from the candidate without "
                         "gating. Calibrated 2026-09-02 on the twelve "
                         "login/mailbox/store traces: the extractor fits "
                         "faces in portrait photos, logotype glyphs and "
                         "badge glow as small diamonds and chamfers (3-9%% "
                         "of a screen), which a trace rightly does not draw; "
                         "a class that carries the screen, like the neomil "
                         "hub's diamonds at 89%%, is also caught by the area "
                         "rule when it is missing")
    ap.add_argument("--ignore-blobs", action="store_true", default=True,
                    help="skip shapes that fitted no template (default on)")
    ap.add_argument("--strict", action="store_true",
                    help="also fail on any per-class count difference")
    ap.add_argument("--gate", choices=["shapes", "inks"], default="shapes",
                    help="which comparison carries the verdict (see doc)")
    ap.add_argument("--min-ink-iou", type=float, default=0.45,
                    help="inks mode: fail below this weighted placement IoU")
    a = ap.parse_args()

    src = json.load(open(a.source))
    cand = json.load(open(a.candidate))
    if src["canvas"] != cand["canvas"]:
        print("canvas mismatch: %s vs %s" % (src["canvas"], cand["canvas"]), file=sys.stderr)
        return 2

    try:
        source_masks = {id(shape): decoded_mask(shape, src["canvas"])
                        for shape in src["shapes"]}
        candidate_masks = {id(shape): decoded_mask(shape, cand["canvas"])
                           for shape in cand["shapes"]}
    except ValueError as error:
        print("invalid component evidence: %s" % error, file=sys.stderr)
        return 2

    keep = lambda ss: [s for s in ss if not (a.ignore_blobs and s["class"] == "blob")]
    ss, cs = keep(src["shapes"]), keep(cand["shapes"])

    pairs, miss, spurious = match(ss, cs, a.match_iou)
    # rect <-> chamfer is not a reclass: the corner detail that separates
    # the two templates is exactly what a photo's glow erases (a rounded
    # outline fits as a chamfer in the source and as a rect in a clean
    # render). Every other class change stays a trace error.
    soft = {frozenset(("rect", "chamfer"))}
    same = lambda s, c: s["class"] == c["class"] or frozenset((s["class"], c["class"])) in soft
    reclass = [(v, s, c) for v, s, c in pairs if not same(s, c)]
    good = [(v, s, c) for v, s, c in pairs if same(s, c)]
    softened = [(v, s, c) for v, s, c in good if s["class"] != c["class"]]
    recovered, miss = recover_class_unstable(
        miss, [s for s in cand["shapes"] if s["class"] == "blob"],
        source_masks, candidate_masks, a.match_iou)

    print("== shape inventory ==")
    print("  %-10s %8s %10s   %s" % ("class", "source", "candidate", "verdict"))
    classes = sorted({s["class"] for s in ss} | {s["class"] for s in cs})
    # The matcher treats these two classes as one family because a photo's
    # glow can erase the corner detail. The absent-class gate must use the
    # same equivalence; matched area still requires a one-to-one box match.
    equivalent = {"rect": {"rect", "chamfer"},
                  "chamfer": {"rect", "chamfer"}}
    missing_class = []
    count_diff = False
    src_area = area(ss) or 1
    for cl in classes:
        n_s = sum(1 for s in ss if s["class"] == cl)
        n_c = sum(1 for s in cs if s["class"] == cl)
        cl_area = area([s for s in ss if s["class"] == cl])
        verdict = ""
        n_equiv = (sum(1 for s in cs if s["class"] in equivalent.get(cl, {cl})) +
                   sum(1 for _, s, _, _ in recovered
                       if s["class"] in equivalent.get(cl, {cl})))
        if n_s and not n_equiv:
            if cl_area / src_area < a.min_class_share:
                verdict = "absent, but %.0f%% of source area — not gating" % (100 * cl_area / src_area)
            else:
                verdict = "ABSENT — source has %d, candidate draws none" % n_s
                missing_class.append(cl)
        elif n_s != n_c:
            verdict = "%+d" % (n_c - n_s)
            count_diff = True
        print("  %-10s %8d %10d   %s" % (cl, n_s, n_c, verdict))

    matched_area = area([s for _, s, _ in good] + [s for _, s, _, _ in recovered])
    total_area = area(ss) or 1
    share = matched_area / total_area

    print("\n== matching (bbox IoU >= %.2f) ==" % a.match_iou)
    print("  matched      %d/%d source shapes (%.0f%% of source shape area)"
          % (len(good) + len(recovered), len(ss), 100 * share))
    if good or recovered:
        errs = sorted(((centre(s["bbox"])[0] - centre(c["bbox"])[0]) ** 2 +
                       (centre(s["bbox"])[1] - centre(c["bbox"])[1]) ** 2) ** 0.5
                      for _, s, c in good + [(v, s, c) for v, s, c, _ in recovered])
        print("  centre error median %.1fpx, worst %.1fpx" % (errs[len(errs) // 2], errs[-1]))
    if softened:
        print("  rect/chamfer swaps counted as matches: %d" % len(softened))
    if recovered:
        print("  class-unstable components recovered by observed mask: %d" % len(recovered))
        for _, s, c, pixel_score in recovered[:10]:
            print("     %-12s -> %-12s at %s  mask IoU %.3f"
                  % (s["id"], c["id"], s["bbox"], pixel_score))
    if reclass:
        print("  reclassified %d (same box, different shape):" % len(reclass))
        for v, s, c in reclass[:10]:
            print("     %-12s -> %-12s at %s" % (s["id"], c["class"], s["bbox"]))
    if miss:
        print("  unmatched in source (candidate draws nothing here): %d" % len(miss))
        for s in sorted(miss, key=lambda s: -s["bbox"][2] * s["bbox"][3])[:10]:
            print("     %-12s %-9s bbox=%s" % (s["id"], s["class"], s["bbox"]))
    if spurious:
        print("  invented by candidate (no source shape): %d" % len(spurious))
        for c in sorted(spurious, key=lambda s: -s["bbox"][2] * s["bbox"][3])[:10]:
            print("     %-12s %-9s bbox=%s" % (c["id"], c["class"], c["bbox"]))

    print("\n== palette (informational) ==")
    for role in ("ink", "ground"):
        f = lambda sp: ", ".join("%s %.1f%%" % (e["hex"], 100 * e["coverage"])
                                 for e in sp["palette"] if e["role"] == role and e["coverage"] > 0)
        print("  %-7s source:    %s" % (role, f(src)))
        print("  %-7s candidate: %s" % ("", f(cand)))
    if cand.get("palette_mode") == "source_anchored":
        residual = cand["palette_residual"]
        print("  candidate pixels outside source palette (RGB distance >= 110): %.2f%%"
              % (100 * residual["rgb_distance_at_least_110"]))
        # This is evidence for review, not an extra failure threshold: a
        # candidate-only annotation is permitted by the existing ink gate.
        for i, residual_share in enumerate(residual["outside_by_nearest_bin"]):
            if residual_share >= 0.001:
                print("    nearest source bin %d: %.2f%% of canvas" % (i, 100 * residual_share))

    # ---- ink-family placement -------------------------------------------
    def fams(sp):
        return [e for e in sp["palette"]
                if e["role"] == "ink" and e.get("ink_grid") and e["coverage"] >= 0.002]

    def grid_iou(ga, gb):
        inter = un = 0
        for ra, rb in zip(ga, gb):
            for ca, cb in zip(ra, rb):
                x, y = ca == "1", cb == "1"
                inter += x and y
                un += x or y
        return inter / un if un else 0.0

    sf, cf = fams(src), fams(cand)
    ink_score, ink_missing = None, []
    if sf and cf:
        print("\n== ink placement (families paired by colour, IoU of 80x45 occupancy) ==")
        pairs = []
        for e in sf:
            for f_ in cf:
                d = sum((x - y) ** 2 for x, y in zip(e["rgb"], f_["rgb"])) ** 0.5
                if d < 110:
                    pairs.append((d, e, f_))
        pairs.sort(key=lambda t: t[0])
        used_s, used_c, matched = set(), set(), {}
        for d, e, f_ in pairs:
            if e["hex"] in used_s or f_["hex"] in used_c:
                continue
            used_s.add(e["hex"]); used_c.add(f_["hex"])
            matched[e["hex"]] = (f_, grid_iou(e["ink_grid"], f_["ink_grid"]))
        wsum = num = 0.0
        for e in sf:
            w = e["coverage"]
            if e["hex"] in matched:
                f_, iou_v = matched[e["hex"]]
                dx = dy = float("nan")
                if e.get("ink_centroid") and f_.get("ink_centroid"):
                    dx = f_["ink_centroid"][0] - e["ink_centroid"][0]
                    dy = f_["ink_centroid"][1] - e["ink_centroid"][1]
                print("  %s (%4.1f%%) -> %s  placement IoU %.2f  centroid delta (%+.0f,%+.0f)"
                      % (e["hex"], 100 * w, f_["hex"], iou_v, dx, dy))
                num += w * iou_v
            else:
                print("  %s (%4.1f%%) -> NO COUNTERPART in candidate" % (e["hex"], 100 * w))
                if w >= 0.01:
                    ink_missing.append(e["hex"])
            wsum += w
        ink_score = num / wsum if wsum else 0.0
        print("  weighted placement IoU: %.2f" % ink_score)

    if a.gate == "inks":
        if ink_score is None:
            print("\nVERDICT: FAIL")
            print("  inks gate requested but a spec lacks ink_grid data — re-extract")
            return 1
        fail = ink_score < a.min_ink_iou or bool(ink_missing)
        print("\nVERDICT: %s  (gate: ink placement)" % ("FAIL" if fail else "PASS"))
        if ink_score < a.min_ink_iou:
            print("  weighted placement IoU %.2f is under %.2f"
                  % (ink_score, a.min_ink_iou))
        for hx in ink_missing:
            print("  source family %s has no counterpart in the candidate" % hx)
        return 1 if fail else 0

    fail = bool(missing_class) or share < a.min_area_match or (a.strict and count_diff)
    print("\nVERDICT: %s  (gate: shape inventory)" % ("FAIL" if fail else "PASS"))
    if missing_class:
        print("  candidate draws no %s at all" % ", ".join(missing_class))
    if share < a.min_area_match:
        print("  only %.0f%% of source shape area is accounted for (need %.0f%%)"
              % (100 * share, 100 * a.min_area_match))
    return 1 if fail else 0


if __name__ == "__main__":
    sys.exit(main())
