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
trace error even though it occupies the same box.

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
import json
import sys


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
    ap.add_argument("--min-class-area", type=float, default=1500,
                    help="a class whose total source area is below this is "
                         "reported but does not gate (text-sized artifacts "
                         "sometimes fit a small diamond or chamfer)")
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

    keep = lambda ss: [s for s in ss if not (a.ignore_blobs and s["class"] == "blob")]
    ss, cs = keep(src["shapes"]), keep(cand["shapes"])

    print("== shape inventory ==")
    print("  %-10s %8s %10s   %s" % ("class", "source", "candidate", "verdict"))
    classes = sorted({s["class"] for s in ss} | {s["class"] for s in cs})
    missing_class = []
    count_diff = False
    for cl in classes:
        n_s = sum(1 for s in ss if s["class"] == cl)
        n_c = sum(1 for s in cs if s["class"] == cl)
        cl_area = area([s for s in ss if s["class"] == cl])
        verdict = ""
        if n_s and not n_c:
            if cl_area < a.min_class_area:
                verdict = "absent, but tiny (%dpx) — not gating" % cl_area
            else:
                verdict = "ABSENT — source has %d, candidate draws none" % n_s
                missing_class.append(cl)
        elif n_s != n_c:
            verdict = "%+d" % (n_c - n_s)
            count_diff = True
        print("  %-10s %8d %10d   %s" % (cl, n_s, n_c, verdict))

    pairs, miss, spurious = match(ss, cs, a.match_iou)
    reclass = [(v, s, c) for v, s, c in pairs if s["class"] != c["class"]]
    good = [(v, s, c) for v, s, c in pairs if s["class"] == c["class"]]

    matched_area = area([s for _, s, _ in good])
    total_area = area(ss) or 1
    share = matched_area / total_area

    print("\n== matching (bbox IoU >= %.2f) ==" % a.match_iou)
    print("  matched      %d/%d source shapes (%.0f%% of source shape area)"
          % (len(good), len(ss), 100 * share))
    if good:
        errs = sorted(((centre(s["bbox"])[0] - centre(c["bbox"])[0]) ** 2 +
                       (centre(s["bbox"])[1] - centre(c["bbox"])[1]) ** 2) ** 0.5
                      for _, s, c in good)
        print("  centre error median %.1fpx, worst %.1fpx" % (errs[len(errs) // 2], errs[-1]))
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
                                 for e in sp["palette"] if e["role"] == role)
        print("  %-7s source:    %s" % (role, f(src)))
        print("  %-7s candidate: %s" % ("", f(cand)))

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
