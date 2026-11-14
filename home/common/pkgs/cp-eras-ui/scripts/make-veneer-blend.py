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

SIGMA = 0.55     # stroke gaussian, texels (FWHM ~1.3, measured 1.2-1.9 native px)
INK = 0.5        # optical depth at a stroke's centre
SAT = 1.5        # overlapping strokes compound, saturating (cores stop near G 57)
GROUND_TAU = 0.1 # light -> base step of the palette, in optical depth

# Per surface: sheets (keys x vertices in texels, steps, per-row fillet
# radii) and the copies each sheet is drawn with. Provenance per surface
# in docs/neokitsch/veneer-construction.md.
GEOMETRY = {
    # fitted (ICP, rms 0.43 px centre / 0.80 px upper half); top band approximate
    'store-card': dict(
        copies=[['id'], ['mirror_y', 291.5]],
        sheets=[
            dict(steps=[16, 26, 11], radii=[0.0, 8.0, 6.0, 6.0, 0.0],
                 keys=[[[-35.0, 24.5], [19.9, 179.2], [20.2, 250.7], [38.0, 275.8], [648.1, 290.7]], [[41.4, 43.0], [197.7, 156.2], [198.1, 266.1], [255.1, 284.1], [615.2, 284.0]], [[48.4, 70.4], [498.6, 106.3], [498.3, 250.6], [494.4, 269.4], [675.9, 304.4]], [[-106.2, 60.2], [629.8, 69.1], [629.9, 212.4], [673.0, 254.2], [789.6, 363.9]]]),
            dict(steps=[16, 26, 11], radii=[0.0, 8.0, 6.0, 6.0, 0.0],
                 keys=[[[-20.3, 54.8], [26.2, 179.8], [24.6, 250.1], [35.5, 270.0], [667.7, 294.8]], [[67.4, 59.0], [203.4, 155.4], [203.2, 268.8], [272.1, 293.0], [648.0, 294.8]], [[99.3, 73.8], [504.4, 105.2], [504.3, 274.5], [528.8, 285.4], [584.6, 277.8]], [[-146.0, 56.2], [634.2, 67.6], [636.3, 185.3], [657.5, 236.6], [664.0, 281.9]]]),
            dict(steps=[44], radii=[0.0, 6.0, 10.0, 0.0],
                 keys=[[[150.0, -30.0], [165.7, -21.5], [198.5, 39.7], [-29.6, 31.0]], [[700.0, -30.0], [646.0, 57.3], [561.9, 92.1], [-30.2, 66.1]]]),
            dict(steps=[44], radii=[0.0, 6.0, 10.0, 0.0],
                 keys=[[[156.0, -30.0], [174.6, -12.4], [201.5, 37.5], [-21.5, 34.1]], [[706.0, -30.0], [641.5, 31.0], [582.2, 92.8], [-25.0, 65.6]]]),
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
    raise ValueError(kind)


def sheet_lines(sheet):
    keys = np.asarray(sheet['keys'], float)
    out = []
    for i, n in enumerate(sheet['steps']):
        last = i == len(sheet['steps']) - 1
        for s in range(n + (1 if last else 0)):
            u = s / n
            out.append((1 - u) * keys[i] + u * keys[i + 1])
    return out


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


def stroke(tau, P, pad=4):
    H, W = tau.shape
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
    tau[y0:y1, x0:x1] += INK * np.exp(-0.5 * (D / SIGMA) ** 2)


def render(name):
    spec, geo = mv.SURFACES[name], GEOMETRY[name]
    W, H = spec['size']
    tw, th = int(np.ceil(W * mv.R)), int(np.ceil(H * mv.R))
    tau = np.zeros((th, tw))
    for sheet in geo['sheets']:
        lines = sheet_lines(sheet)
        for c in geo['copies']:
            M = transform(c)
            for V in lines:
                stroke(tau, fillet(V @ M[:, :2].T + M[:, 2], sheet['radii']))
    tau = SAT * (1 - np.exp(-tau / SAT))
    rng = np.random.Generator(np.random.PCG64(spec['seed'] + 7))
    tau = tau + 0.02 * rng.standard_normal(tau.shape)         # pore noise
    b, span = spec['base_at'], 255 - spec['base_at']
    idx = np.where(tau < GROUND_TAU, np.maximum(tau, 0) / GROUND_TAU * b,
                   b + (tau - GROUND_TAU) / spec['taumax'] * span)
    return np.clip(np.round(idx), 0, 255).astype(np.uint8)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('out')
    ap.add_argument('--only', action='append')
    args = ap.parse_args()
    out = Path(args.out); out.mkdir(parents=True, exist_ok=True)
    for name in GEOMETRY:
        if args.only and name not in args.only:
            continue
        idx = render(name)
        data = mv.png_indexed(idx, mv.ramp(mv.SURFACES[name]))
        (out / f'{name}.png').write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {name}.png  {idx.shape[1]}x{idx.shape[0]}', file=sys.stderr)


if __name__ == '__main__':
    main()
