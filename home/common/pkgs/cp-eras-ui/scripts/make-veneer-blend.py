"""Neo-kitsch veneer as *blended polylines* (candidate replacement for the
noise-field generator in make-veneer.py; see
docs/neokitsch/veneer-construction.md for how the construction was recovered).

The reference grain is vector line art built the way an Illustrator blend
is: a few *key* polylines with the same number of vertices, and `steps`
equally spaced intermediate lines between consecutive keys, every vertex
interpolated linearly. So within one key interval

    line(k)[v] = lerp(key[i][v], key[i+1][v], k / steps[i])

Equivalently a quad mesh: rows v are the *folds* (straight loci where every
line turns), columns i the keys. Corners are rounded with a per-row fillet
radius (0 = sharp). A surface is a few such *sheets*, each replicated by
affine *copies* (identity, mirror about the book-match axis); the
interleaved "twin" comb is a second sheet offset by part of a pitch.

Coordinates are texels of the output (2.4 per design px, i.e. native 4K
pixels of the surface's box). Strokes are Gaussian (sigma `SIGMA` texels)
summed as optical depth, softly saturated, coloured with make-veneer.py's
measured palettes. No image is read; geometry is the numbers below.

    nix-shell shell.nix --run 'python3 scripts/make-veneer-blend.py OUTDIR'
"""
import argparse, hashlib, importlib.util, sys
from pathlib import Path
import numpy as np

_spec = importlib.util.spec_from_file_location('mv', Path(__file__).with_name('make-veneer.py'))
mv = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(mv)

# Line look, measured on the 4K references (veneer-construction.md,
# "Line, ground and grain"). A line is a 1-texel-wide box stroke (coverage =
# its overlap with the texel), composited *over* the ground in the line
# colour `core` with per-channel opacity `alpha`; n overlapping strokes
# leave the ground showing through prod(1 - alpha*coverage), so a pile of
# strokes darkens towards `core` and stops there (the streak cores).
# alpha follows from one centred line: (ground - single) / (ground - core).
# Ground: the measured paper colour (R before clipping), plus per-channel
# pixel grain: independent normal noise per channel through a separable
# [1, 3, 1] filter (lag-1 autocorrelation 0.55, as measured), std `grain`.
CORE = (150.0, 61.0, 30.0)   # the store streak core, darkest compounded ink
LOOK = {
    #                 ground (R, G, B)        single centred line        grain std (R, G, B)
    'store-card':      dict(ground=(262, 206, 131), single=(211, 144, 67), grain=(3.7, 5.3, 6.4)),
    'dashboard-panel': dict(ground=(262, 194, 110), single=(217, 154, 76), grain=(4.1, 4.6, 4.9)),
    'dashboard-email': dict(ground=(262, 191, 108), single=(199, 138, 65), grain=(3.7, 4.8, 5.2)),
    'mailbox-row':     dict(ground=(258, 202, 130), single=(207, 146, 83), grain=(3.4, 5.0, 6.1)),
    'store-nav':       dict(ground=(249, 198, 128), single=(202, 140, 74), grain=(6.0, 5.2, 6.2)),
    'store-basket':    dict(ground=(249, 181, 107), single=(202, 120, 79), grain=(6.0, 6.0, 6.0)),
}
WIDTH = 1.0      # stroke width, texels

# Per surface: sheets (keys x vertices in texels, steps, per-row fillet
# radii) and the copies each sheet is drawn with. Provenance per surface
# in docs/neokitsch/veneer-construction.md.
GEOMETRY = {
    # middle (keys 0-2 + twin): 10 smooth head points (key 0 enters from the
    # left edge, key 1 from the top-left corner, key 2 is the top family's
    # key 0, so the top sheets skip their first line), the ICP-fitted fold v1
    # and vertical run (rms 0.44 px centre), then the streak tail
    # [knee, crossing knee, convergence]. Streak rule (veneer-construction.md,
    # fourth pass): from the knee (x_v, y_k) a straight run descends at theta
    # past the axis y=291.5 to y=291.5+h, then runs to C=(640.3, 292.05); per
    # key (y_k, theta deg, h) = (268, 24, 17.7), (265.2, 11.9, 9.2),
    # (271.5, 7.2, 5.1), top family's right key (245.8, 10, 0.8); knee and
    # crossing-knee fillets 30 and 16. The mirror copy is the other leaf, so
    # the leaves cross: that is the lattice. Top family (S-curves from the
    # top edge into the right-hand verticals): smooth-key LSQ, rms 1.56 px on
    # 38% of its points. Fitted by e3/fit2.py (pass scratch).
    'store-card': dict(
        copies=[['id'], ['mirror_y', 291.5]],
        sheets=[
            dict(steps=[16, 26], radii=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 29.98, 16.0, 0.0], smooth=2, smooth_upto=10,
                 keys=[[[-3.0, 186.0], [-0.46, 185.24], [2.09, 184.49], [4.63, 183.73], [7.18, 182.98], [9.72, 182.22], [12.27, 181.47], [14.81, 180.71], [17.36, 179.96], [19.9, 179.2], [20.2, 268.0], [112.75, 309.21], [640.28, 292.05]], [[-3.0, 0.43], [19.3, 17.88], [41.6, 36.05], [63.9, 52.85], [86.2, 70.23], [108.5, 89.34], [130.8, 105.75], [153.1, 123.42], [175.4, 145.37], [197.7, 156.2], [198.1, 265.15], [366.45, 300.65], [640.28, 292.05]], [[246.17, 2.84], [275.84, 3.01], [298.27, 31.34], [330.89, 51.27], [366.0, 58.2], [403.12, 66.15], [411.39, 76.96], [441.85, 73.69], [483.82, 104.15], [498.6, 106.3], [498.3, 271.54], [635.28, 296.58], [640.28, 292.05]]]),
            dict(steps=[16, 26], radii=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 29.98, 16.0, 0.0], smooth=2, smooth_upto=10,
                 keys=[[[3.3, 186.6], [5.84, 185.84], [8.39, 185.09], [10.93, 184.33], [13.48, 183.58], [16.02, 182.82], [18.57, 182.07], [21.11, 181.31], [23.66, 180.56], [26.2, 179.8], [24.6, 267.4], [118.5, 309.21], [644.68, 292.05]], [[2.7, -0.37], [25.0, 17.08], [47.3, 35.25], [69.6, 52.05], [91.9, 69.43], [114.2, 88.54], [136.5, 104.95], [158.8, 122.62], [181.1, 144.57], [203.4, 155.4], [203.2, 267.85], [358.75, 300.65], [645.38, 292.05]], [[251.97, 2.84], [281.64, 3.01], [304.07, 31.34], [336.69, 51.27], [371.8, 58.2], [408.92, 66.15], [417.19, 76.96], [447.65, 73.69], [489.62, 104.15], [504.4, 105.2], [504.3, 277.54], [641.28, 296.58], [646.28, 292.05]]]),
            dict(steps=[11], radii=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 29.98, 16.0, 0.0], smooth=2, smooth_upto=10, skip=[0],
                 keys=[[[246.17, 2.84], [275.84, 3.01], [298.27, 31.34], [330.89, 51.27], [366.0, 58.2], [403.12, 66.15], [411.39, 76.96], [441.85, 73.69], [483.82, 104.15], [498.6, 106.3], [498.3, 271.54], [635.28, 296.58], [640.28, 292.05]], [[619.16, 7.16], [621.68, 11.26], [623.06, 10.99], [622.49, 16.85], [629.78, 25.18], [634.52, 32.38], [633.7, 44.62], [645.38, 60.9], [635.73, 59.92], [629.8, 69.1], [629.9, 245.82], [635.28, 292.27], [640.28, 292.05]]]),
            dict(steps=[11], radii=[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 29.98, 16.0, 0.0], smooth=2, smooth_upto=10, skip=[0],
                 keys=[[[251.97, 2.84], [281.64, 3.01], [304.07, 31.34], [336.69, 51.27], [371.8, 58.2], [408.92, 66.15], [417.19, 76.96], [447.65, 73.69], [489.62, 104.15], [504.4, 105.2], [504.3, 277.54], [641.28, 296.58], [646.28, 292.05]], [[624.96, 7.16], [627.48, 11.26], [628.86, 10.99], [628.29, 16.85], [635.58, 25.18], [640.32, 32.38], [639.5, 44.62], [651.18, 60.9], [641.53, 59.92], [634.2, 67.6], [636.3, 218.72], [641.08, 292.27], [646.08, 292.05]]]),
        ]),
    # hand-set from the trace (ICP rms 0.39 px on 53% of points)
    'store-nav': dict(
        copies=[['id'], ['mirror_x', 239.0]],
        sheets=[
            dict(steps=[19], radii=[0.0, 45.0, 4.0, 0.0],
                 keys=[[[15.0, -70.0], [75.0, 1.0], [229.0, 1.0], [239.0, -8.0]], [[15.0, 165.0], [75.0, 95.0], [225.0, 95.0], [239.0, 80.0]]]),
            dict(steps=[19], radii=[0.0, 45.0, 4.0, 0.0],
                 keys=[[[15.0, -68.0], [75.0, 3.0], [229.0, 3.0], [239.0, -6.0]], [[15.0, 167.0], [75.0, 97.0], [225.0, 97.0], [239.0, 82.0]]]),
        ]),
    # hand-set from the trace; axis 355-370 equally good
    'dashboard-email': dict(
        copies=[['id'], ['mirror_y', 362.0]],
        sheets=[
            dict(steps=[6, 17], radii=[0.0, 70.0, 0.0],
                 keys=[[[5.0, -30.0], [5.0, 310.0], [14.0, 362.0]], [[60.0, -30.0], [60.0, 300.0], [245.0, 380.0]], [[214.0, -30.0], [214.0, 240.0], [245.0, 380.0]]]),
            dict(steps=[6, 17], radii=[0.0, 70.0, 0.0],
                 keys=[[[8.4, -30.0], [8.4, 310.0], [17.4, 362.0]], [[63.4, -30.0], [63.4, 300.0], [248.4, 380.0]], [[217.4, -30.0], [217.4, 240.0], [248.4, 380.0]]]),
        ]),
    # lower half: ICP init + hand edits; mirrored to the top
    'dashboard-panel': dict(
        copies=[['id'], ['mirror_y', 370.5]],
        sheets=[
            dict(steps=[20], radii=[0.0, 7.0, 6.0, 6.0, 0.0],
                 keys=[[[5.0, 370.5], [5.0, 522.0], [-30.0, 550.0], [-120.0, 600.0], [-160.0, 620.0]], [[205.0, 370.5], [205.0, 562.0], [150.0, 600.0], [10.0, 720.0], [-40.0, 760.0]]]),
            dict(steps=[20], radii=[0.0, 7.0, 6.0, 6.0, 0.0],
                 keys=[[[9.3, 370.5], [9.3, 522.0], [-25.7, 550.0], [-115.7, 600.0], [-155.7, 620.0]], [[209.3, 370.5], [209.3, 562.0], [154.3, 600.0], [14.3, 720.0], [-35.7, 760.0]]]),
            dict(steps=[22, 12], radii=[0.0, 7.0, 6.0, 6.0, 0.0],
                 keys=[[[205.0, 370.5], [205.0, 562.0], [150.0, 640.0], [-20.0, 705.0], [-60.0, 722.0]], [[428.9, 370.5], [429.0, 603.9], [249.6, 641.4], [417.9, 664.9], [452.4, 733.7]], [[551.9, 370.5], [552.1, 563.3], [519.9, 633.8], [553.9, 651.9], [606.4, 792.5]]]),
            dict(steps=[22, 12], radii=[0.0, 7.0, 6.0, 6.0, 0.0],
                 keys=[[[209.3, 370.5], [209.3, 562.0], [154.3, 640.0], [-15.7, 705.0], [-55.7, 722.0]], [[433.3, 370.5], [433.3, 604.7], [258.0, 640.2], [420.3, 664.9], [455.3, 732.5]], [[557.5, 370.5], [557.2, 561.1], [529.8, 636.5], [559.6, 636.4], [611.1, 808.3]]]),
        ]),
    # hand-set (rough), mirrored about the seam
    'mailbox-row': dict(
        copies=[['id'], ['mirror_x', 571.5]],
        sheets=[
            dict(steps=[22], radii=[0.0, 40.0, 25.0, 25.0, 6.0, 0.0],
                 keys=[[[-10.0, -70.0], [45.0, 1.0], [190.0, 1.0], [260.0, 1.0], [552.0, 1.0], [571.5, 12.0]], [[-10.0, 60.0], [125.0, 131.0], [200.0, 131.0], [280.0, 131.0], [548.0, 131.0], [571.5, 147.0]]]),
            dict(steps=[22], radii=[0.0, 40.0, 25.0, 25.0, 6.0, 0.0],
                 keys=[[[-10.0, -68.5], [45.0, 2.5], [190.0, 2.5], [260.0, 2.5], [552.0, 2.5], [571.5, 13.5]], [[-10.0, 61.5], [125.0, 132.5], [200.0, 132.5], [280.0, 132.5], [548.0, 132.5], [571.5, 148.5]]]),
        ]),
    # hand-set (rough); seam not mirrored (leaves differ)
    'store-basket': dict(
        copies=[['id']],
        sheets=[
            dict(steps=[24], radii=[0.0, 30.0, 5.0, 0.0],
                 keys=[[[-10.0, -60.0], [40.0, -2.0], [190.0, -2.0], [205.0, -12.0]], [[20.0, 160.0], [60.0, 112.0], [186.0, 112.0], [205.0, 100.0]]]),
            dict(steps=[24], radii=[0.0, 30.0, 5.0, 0.0],
                 keys=[[[520.0, -60.0], [460.0, -2.0], [220.0, -2.0], [205.0, -12.0]], [[470.0, 150.0], [420.0, 112.0], [224.0, 112.0], [205.0, 100.0]]]),
        ]),
}


def transform(kind):
    if kind[0] == 'id':
        return np.array([[1.0, 0, 0], [0, 1.0, 0]])
    if kind[0] == 'mirror_y':
        return np.array([[1.0, 0, 0], [0, -1.0, 2 * kind[1]]])
    if kind[0] == 'mirror_x':
        return np.array([[-1.0, 0, 2 * kind[1]], [0, 1.0, 0]])
    if kind[0] == 'translate':
        return np.array([[1.0, 0, kind[1]], [0, 1.0, kind[2]]])
    raise ValueError(kind)


def sheet_lines(sheet):
    keys = np.asarray(sheet['keys'], float)
    out = []
    skip = set(sheet.get('skip', ()))   # line indices not drawn (e.g. a key another sheet already draws)
    j = -1
    for i, n in enumerate(sheet['steps']):
        last = i == len(sheet['steps']) - 1
        for s in range(n + (1 if last else 0)):
            u = s / n
            j += 1
            if j in skip:
                continue
            out.append((1 - u) * keys[i] + u * keys[i + 1])
    return out


def chaikin(P, rounds):
    """Corner cutting: `rounds` passes of Chaikin's scheme (a smooth key
    drawn through its control polygon), end points kept."""
    for _ in range(rounds):
        Q, R = P[:-1] * 0.75 + P[1:] * 0.25, P[:-1] * 0.25 + P[1:] * 0.75
        P = np.vstack([P[:1], np.stack([Q, R], 1).reshape(-1, 2), P[-1:]])
    return P


def fillet(V, radii):
    """Polyline -> sampled path with each corner v replaced by a circular arc
    of radius radii[v] (clamped to half the adjacent segments)."""
    pts = [V[0]]
    for j in range(1, len(V) - 1):
        a, b = V[j - 1] - V[j], V[j + 1] - V[j]
        la, lb = np.hypot(*a), np.hypot(*b)
        r = radii[j] if j < len(radii) else 0.0
        if la < 1e-9 or lb < 1e-9:
            continue
        a, b = a / la, b / lb
        ang = np.arccos(np.clip(a @ b, -1, 1))
        if r <= 0 or ang > np.pi - 0.02:
            pts.append(V[j]); continue
        t = min(r / np.tan(ang / 2), 0.5 * la, 0.5 * lb)
        rr = t * np.tan(ang / 2)
        T1, T2 = V[j] + a * t, V[j] + b * t
        bis = (a + b) / np.hypot(*(a + b))
        c = V[j] + bis * (rr / np.sin(ang / 2))
        a0, a1 = np.arctan2(*(T1 - c)[::-1]), np.arctan2(*(T2 - c)[::-1])
        d = (a1 - a0 + np.pi) % (2 * np.pi) - np.pi
        m = max(2, int(abs(d) * rr / 0.5) + 1)
        th = a0 + np.linspace(0, d, m)
        pts.extend(c + rr * np.c_[np.cos(th), np.sin(th)])
    pts.append(V[-1])
    return np.array(pts)


def path(V, sheet):
    """A line's drawn path: the first `smooth_upto` vertices (a smooth key
    part) through `smooth` rounds of Chaikin, the rest filleted per row."""
    k, n = sheet.get('smooth_upto', 0), sheet.get('smooth', 0)
    if k <= 1 or n == 0:
        return fillet(V, sheet['radii'])
    head = chaikin(V[:k], n)
    if k >= len(V):
        return head
    tail = fillet(V[k - 1:], sheet['radii'][k - 1:])
    return np.vstack([head[:-1], tail])


def stroke(taus, P, alpha, width=WIDTH, pad=2):
    """Add one stroke's optical depth -ln(1 - alpha_c * coverage) per
    channel; coverage is a `width`-wide box's overlap with the texel."""
    H, W = taus.shape[1:]
    x0 = max(int(np.floor(P[:, 0].min())) - pad, 0); x1 = min(int(np.ceil(P[:, 0].max())) + pad, W)
    y0 = max(int(np.floor(P[:, 1].min())) - pad, 0); y1 = min(int(np.ceil(P[:, 1].max())) + pad, H)
    if x1 <= x0 or y1 <= y0:
        return
    D = np.full((y1 - y0, x1 - x0), 1e9)
    for a, b in zip(P[:-1], P[1:]):
        bx0 = max(int(np.floor(min(a[0], b[0]))) - pad, x0); bx1 = min(int(np.ceil(max(a[0], b[0]))) + pad, x1)
        by0 = max(int(np.floor(min(a[1], b[1]))) - pad, y0); by1 = min(int(np.ceil(max(a[1], b[1]))) + pad, y1)
        if bx1 <= bx0 or by1 <= by0:
            continue
        X, Y = np.meshgrid(np.arange(bx0, bx1) + 0.5, np.arange(by0, by1) + 0.5)
        d = b - a; L2 = d @ d + 1e-12
        t = np.clip(((X - a[0]) * d[0] + (Y - a[1]) * d[1]) / L2, 0, 1)
        sub = D[by0 - y0:by1 - y0, bx0 - x0:bx1 - x0]
        np.minimum(sub, np.hypot(X - a[0] - t * d[0], Y - a[1] - t * d[1]), out=sub)
    cov = np.clip(np.minimum(D + width / 2, 0.5) - np.maximum(D - width / 2, -0.5), 0, None)
    for c in range(3):
        taus[c, y0:y1, x0:x1] -= np.log1p(-np.minimum(alpha[c] * cov, 0.999))


def grain(shape, std, seed):
    """Per-channel pixel grain: normal noise through a separable [1, 3, 1]
    (wrapping), scaled to unit variance and then to `std`."""
    rng = np.random.Generator(np.random.PCG64(seed))
    out = np.empty((3,) + shape)
    for c in range(3):
        n = rng.standard_normal(shape)
        n = np.roll(n, 1, 0) + 3 * n + np.roll(n, -1, 0)
        n = np.roll(n, 1, 1) + 3 * n + np.roll(n, -1, 1)
        out[c] = n / 11.0 * std[c]       # 11 = sum of the 2-D kernel's squares
    return out


def palette_quantise(rgb, k=256, iters=8):
    """Deterministic k-means of the RGB texels into a k-colour palette
    (initialised at luminance quantiles of every 7th texel)."""
    X = rgb.reshape(-1, 3)
    S = X[::7]
    lum = S @ np.array([0.3, 0.59, 0.11])
    order = np.argsort(lum, kind='stable')
    C = S[order[(np.arange(k) * (len(S) - 1)) // (k - 1)]].copy()

    def assign(P):
        out = np.empty(len(P), np.int64)
        cc = (C * C).sum(1)[None]
        for i in range(0, len(P), 65536):
            Q = P[i:i + 65536]
            out[i:i + 65536] = np.argmin(cc - 2 * Q @ C.T, 1)
        return out
    for _ in range(iters):
        a = assign(S)
        cnt = np.bincount(a, minlength=k)
        for c in range(3):
            s = np.bincount(a, weights=S[:, c], minlength=k)
            C[:, c] = np.where(cnt > 0, s / np.maximum(cnt, 1), C[:, c])
    C = np.clip(np.round(C), 0, 255)
    idx = assign(X).reshape(rgb.shape[:2])
    return idx.astype(np.uint8), C.astype(np.uint8)


def render_rgb(name, geo=None, grain_on=True):
    """The surface as float RGB (H x W x 3), before palette quantisation."""
    spec, look = mv.SURFACES[name], LOOK[name]
    geo = GEOMETRY[name] if geo is None else geo
    W, H = spec['size']
    tw, th = int(np.ceil(W * mv.R)), int(np.ceil(H * mv.R))
    ground, single = np.array(look['ground'], float), np.array(look['single'], float)
    core = np.minimum(np.array(CORE) * ground / np.array(LOOK['store-card']['ground'], float), single - 1)
    alpha = (ground - single) / (ground - core)
    taus = np.zeros((3, th, tw))
    for sheet in geo['sheets']:
        a = alpha * sheet.get('ink', 1.0)
        for c in sheet.get('copies', geo['copies']):
            M = transform(c)
            for V in sheet_lines(sheet):
                stroke(taus, path(V @ M[:, :2].T + M[:, 2], sheet), a)
    rgb = core[:, None, None] + (ground - core)[:, None, None] * np.exp(-taus)
    if grain_on:
        rgb = rgb + grain((th, tw), look['grain'], spec['seed'] + 7)
    return np.clip(rgb, 0, 255).transpose(1, 2, 0)


def render(name):
    return palette_quantise(render_rgb(name))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('out')
    ap.add_argument('--only', action='append')
    args = ap.parse_args()
    out = Path(args.out); out.mkdir(parents=True, exist_ok=True)
    for name in GEOMETRY:
        if args.only and name not in args.only:
            continue
        idx, pal = render(name)
        data = mv.png_indexed(idx, pal)
        (out / f'{name}.png').write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {name}.png  {idx.shape[1]}x{idx.shape[0]}', file=sys.stderr)


if __name__ == '__main__':
    main()
