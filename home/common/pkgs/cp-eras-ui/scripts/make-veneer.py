"""Generate the Neo-kitsch wood-veneer textures.

Owner decision 2026-11-13 (docs/review-2026-11-13/DECISIONS.md, 3): the
selected surfaces wear a pre-rendered texture clipped to their shapes,
not drawn vector grain. The reference art is third-party and never
redistributed, so nothing here is copied from it: every texel is
computed from the parameters below, which were *measured* off the
references (docs/neokitsch/veneer-texture.md has the numbers) and then
tuned by eye. This script reads no image at all.

The model. The source veneer is a dense field of fine dark lines -- a
contour drawing rather than a photograph -- that bend into book-match
seams and figures. So a texture is the iso-lines of scalar fields

    phi(s, t) = s + D(s, t)

where `s` runs across the grain and `t` along it, one line every
`pitch` design px. A surface holds one or more *families* of lines,
each its own `phi`, each shown inside a window along `t`. Where two
windows overlap both families are drawn and their lines cross, which is
how the references' seams read. A family can be the mirror image of the
field about a line t = m: the book-match, two leaves cut from one log.
`D` sums:

  * `wave`  -- smooth anisotropic lattice noise (fBm): the cathedral
               wander, and with a short across-grain cell the bunching
               of the lines;
  * `bend`  -- lines pushed sideways near t = ts, decaying as
               exp(-|t - ts| / w), by an amount that varies linearly
               across the grain. Steep enough and the lines turn to run
               along the seam: the streaks and converging seams;
  * `fold`  -- a sigmoid shift across a sloping line: the panel's figure,
               where the grain folds down-left;
  * `fan`   -- lines spread away from a centre line near t = ts: the
               mailbox's open book.

Each line is a gaussian core `core` native px wide (the references'
cores measure about one native px), evaluated on a 3x3 supersample from
the distance |phi - nearest line| / |grad phi|, so converging lines
merge into a darker band rather than aliasing. Then fine pore noise
stretched along the grain, a faint blotch, a mild edge shade, and a
three-stop colour ramp per surface.

Output: one 8-bit *indexed* PNG per surface at 2.4 texels per design px
(3840x2160 is 2.4x the 1600x900 design), the ramp stored as its PLTE so
that the same file is what rsvg draws in the traces and what the crate
decodes. Indices run 0 (lightest) to 255 (darkest).

Reproducible: a fixed seed per surface (numpy's PCG64, whose stream is
stable across numpy releases), float64 arithmetic, and a hand-written
PNG encoder with fixed zlib settings, so two runs give the same bytes.
Run through the crate's dev shell:

    nix-shell shell.nix --run 'python3 scripts/make-veneer.py docs/neokitsch/veneer'

`default.nix`'s `passthru.tests.veneer` re-runs it in the build sandbox
and compares the output with the committed files byte for byte.
"""
import argparse
import hashlib
import struct
import sys
import zlib
from pathlib import Path

import numpy as np

R = 2.4          # texels per design px
SUPER = 3        # supersamples per texel, per axis

# Per-surface recipes. `size` is the surface's bounding box in design
# px, the texture's extent. `colours` are (light, base, dark) in sRGB
# hex: light at index 0, base at `base_at`, dark at 255. A family's
# `window` is (t0, t1, fade): shown for t0 <= t <= t1, faded in and out
# over `fade` design px (None for an open end).
SURFACES = {
    # Dashboard, the selected EMAIL cascade card (#ncardsel, 90.5 x 325.2).
    # Vertical grain. Lines swing left into the top chamfer, crossing a
    # second leaf there; a horizontal book-match about halfway down
    # (t 160) where both leaves bend right and cross; the foot bends left
    # along the diagonal cut.
    'dashboard-email': dict(
        size=(90.5, 325.2), axis='v', seed=1101, pitch=2.2, core=0.8,
        colours=('ffcf84', 'ffbf6c', '7a4818'), base_at=30,
        wave=dict(amp=2.0, along=80.0, across=14.0),
        families=[
            dict(window=(None, 186.0, 26.0), terms=[
                dict(kind='bend', ts=0.0, w=38.0, amount=(-40.0, -12.0)),
                dict(kind='bend', ts=160.0, w=14.0, q=1.5, amount=(0.0, 42.0)),
            ]),
            dict(mirror=160.0, window=(134.0, None, 26.0), terms=[
                dict(kind='bend', ts=160.0, w=14.0, q=1.5, amount=(0.0, 42.0)),
                dict(kind='bend', ts=325.2, w=40.0, amount=(-26.0, -4.0), real=True),
            ]),
            dict(mirror=24.0, window=(None, 46.0, 22.0), terms=[
                dict(kind='bend', ts=0.0, w=14.0, amount=(-30.0, -10.0)),
            ]),
        ],
        edge=dict(width=7.0, depth=8.0, right=1.6),
    ),
    # Dashboard, the right-hand EMAIL panel body (230.4 x 309). Vertical
    # grain, calm under the text, folding down-left in the lower third
    # (the ridge runs from local (179, 259) down to (84, 299)).
    'dashboard-panel': dict(
        size=(230.4, 309.0), axis='v', seed=1202, pitch=2.15, core=0.8,
        colours=('ffd189', 'ffc26e', '8a5824'), base_at=30,
        wave=dict(amp=2.0, along=90.0, across=14.0),
        families=[
            dict(window=(None, None, None), terms=[
                dict(kind='fold', at=(84.0, 299.0), to=(184.0, 259.0), beyond=(-0.25, 0.10),
                     w=26.0, amount=(-95.0, -55.0)),
                dict(kind='bend', ts=309.0, w=30.0, amount=(-10.0, -2.0)),
            ]),
        ],
        edge=dict(width=8.0, depth=10.0),
    ),
    # Mailbox, the selected row (477 x 55). Horizontal grain, wavy and
    # bunched; the left end's lines rise to the top edge; a vertical
    # book-match at t 239 (design x 274) where the lines fan away from
    # the bar's middle height into the seam: the open book.
    'mailbox-row': dict(
        size=(477.0, 55.0), axis='h', seed=1303, pitch=2.2, core=0.8,
        colours=('ffda98', 'ffca84', '8c561f'), base_at=30,
        wave=dict(amp=5.0, along=70.0, across=10.0),
        families=[
            dict(window=(None, 239.0, None), terms=[
                dict(kind='bend', ts=0.0, w=24.0, amount=(22.0, 10.0)),
                dict(kind='fan', ts=239.0, w=20.0, centre=30.0, gain=2.6),
            ]),
            dict(mirror=239.0, window=(239.0, None, None), terms=[
                dict(kind='fan', ts=239.0, w=20.0, centre=30.0, gain=2.6),
            ]),
        ],
        edge=dict(width=6.0, depth=8.0),
    ),
    # Store, the selected SMG category (200.6 x 38.6). Horizontal grain,
    # lines pinching into nested arcs at the left end, a book-match at
    # t 75 (design x 168).
    'store-nav': dict(
        size=(200.6, 38.6), axis='h', seed=1404, pitch=2.05, core=0.8,
        colours=('ffda98', 'fac883', '8a521c'), base_at=30,
        wave=dict(amp=2.4, along=60.0, across=12.0),
        families=[
            dict(window=(None, 75.0, None), terms=[
                dict(kind='bend', ts=0.0, w=16.0, amount=(14.0, -10.0)),
                dict(kind='fan', ts=75.0, w=12.0, centre=19.0, gain=2.0),
            ]),
            dict(mirror=75.0, window=(75.0, None, None), terms=[
                dict(kind='fan', ts=75.0, w=12.0, centre=19.0, gain=2.0),
            ]),
        ],
        edge=dict(width=5.0, depth=8.0),
    ),
    # Store, the selected card's body and socket band (262.1 x 241.7).
    # Vertical grain with the strong horizontal streak at t 120 (design
    # y 531): both leaves bend right into it, hardest at the right edge.
    # Below t 192.6 (design 603.8, the socket rule) a separate leaf fans
    # across.
    'store-card': dict(
        size=(262.1, 241.7), axis='v', seed=1505, pitch=2.2, core=0.8,
        colours=('ffdc9a', 'ffce84', '7e4614'), base_at=30,
        wave=dict(amp=3.0, along=70.0, across=14.0),
        families=[
            dict(window=(None, 124.0, 4.0), terms=[
                dict(kind='bend', ts=0.0, w=22.0, amount=(-10.0, -4.0)),
                dict(kind='bend', ts=70.0, w=40.0, q=1.2, amount=(-2.0, 14.0)),
                dict(kind='bend', ts=120.0, w=26.0, q=2.0, power=2.0, amount=(0.0, 100.0)),
            ]),
            dict(mirror=120.0, window=(116.0, 192.6, 4.0), terms=[
                dict(kind='bend', ts=120.0, w=26.0, q=2.0, power=2.0, amount=(0.0, 100.0)),
            ]),
            dict(window=(192.6, None, None), terms=[
                dict(kind='bend', ts=192.6, w=14.0, amount=(-18.0, -30.0), real=True),
                dict(kind='bend', ts=241.7, w=18.0, amount=(8.0, 14.0), real=True),
            ]),
        ],
        edge=dict(width=6.0, depth=8.0),
    ),
    # Store, the BASKET plate's lower band (204.6 x 45). Horizontal
    # grain with a book-match at t 37 (design x 1329).
    'store-basket': dict(
        size=(204.6, 45.0), axis='h', seed=1606, pitch=2.0, core=0.8,
        colours=('ffc77e', 'fab56b', '84471c'), base_at=30,
        wave=dict(amp=2.2, along=60.0, across=12.0),
        families=[
            dict(window=(None, 37.0, None), terms=[
                dict(kind='fan', ts=37.0, w=10.0, centre=24.0, gain=1.8),
            ]),
            dict(mirror=37.0, window=(37.0, None, None), terms=[
                dict(kind='fan', ts=37.0, w=10.0, centre=24.0, gain=1.8),
            ]),
        ],
        edge=dict(width=5.0, depth=8.0),
    ),
}


def lattice_noise(rng, shape_px, cell, out_shape):
    """Smooth value noise: a random lattice every `cell` texels,
    interpolated with a C1 smoothstep. Returns an array of `out_shape`
    in about [-1, 1]."""
    h, w = out_shape
    gh, gw = int(np.ceil(h / cell)) + 2, int(np.ceil(w / cell)) + 2
    grid = rng.uniform(-1.0, 1.0, size=(gh, gw))
    y = np.arange(h, dtype=np.float64) / cell
    x = np.arange(w, dtype=np.float64) / cell
    y0, x0 = np.floor(y).astype(int), np.floor(x).astype(int)
    fy, fx = y - y0, x - x0
    sy, sx = fy * fy * (3 - 2 * fy), fx * fx * (3 - 2 * fx)
    a = grid[y0][:, x0]
    b = grid[y0][:, x0 + 1]
    c = grid[y0 + 1][:, x0]
    d = grid[y0 + 1][:, x0 + 1]
    top = a + (b - a) * sx[None, :]
    bot = c + (d - c) * sx[None, :]
    return top + (bot - top) * sy[:, None]


class Wave:
    """Anisotropic fBm in design px, evaluated anywhere in a margin
    around the surface: a book-matched leaf reads the same wander at the
    reflected position, so its lines meet the other leaf's at the seam
    instead of doubling them."""

    OCTAVES = 3

    def __init__(self, rng, S, T, along, across, margin):
        self.along, self.across = along, across
        self.lo_s, self.lo_t = -margin, -margin
        self.grids = []
        for o in range(self.OCTAVES):
            f = 2.0 ** o
            gs = int(np.ceil((S + 2 * margin) * f / across)) + 3
            gt = int(np.ceil((T + 2 * margin) * f / along)) + 3
            self.grids.append(rng.uniform(-1.0, 1.0, size=(gt, gs)))

    def __call__(self, s, t):
        total = np.zeros_like(s)
        norm = 0.0
        for o, grid in enumerate(self.grids):
            f = 2.0 ** o
            u = (s - self.lo_s) * f / self.across
            v = (t - self.lo_t) * f / self.along
            u0 = np.clip(np.floor(u).astype(int), 0, grid.shape[1] - 2)
            v0 = np.clip(np.floor(v).astype(int), 0, grid.shape[0] - 2)
            fu, fv = u - u0, v - v0
            su, sv = fu * fu * (3 - 2 * fu), fv * fv * (3 - 2 * fv)
            top = grid[v0, u0] + (grid[v0, u0 + 1] - grid[v0, u0]) * su
            bot = grid[v0 + 1, u0] + (grid[v0 + 1, u0 + 1] - grid[v0 + 1, u0]) * su
            amp = 0.5 ** o
            total += amp * (top + (bot - top) * sv)
            norm += amp
        return total / norm


def across(s, S, amount, power=1.0):
    """Across the grain: amount[0] at s = 0, amount[1] at s = S, linear
    or, with `power` > 1, gathered towards s = S."""
    a0, a1 = amount
    return a0 + (a1 - a0) * (s / S) ** power


def family_phi(spec, fam, s, t, S, wave):
    """phi of one family, in design px. A mirrored family reads the
    field -- the wander included -- at the reflected along-grain
    coordinate; a term marked `real` is placed in the surface's own
    coordinates even so."""
    m = fam.get('mirror')
    tm = 2 * m - t if m is not None else t
    phi = s + spec['wave']['amp'] * wave(s, tm)
    fan = None
    for term in fam['terms']:
        tt = t if term.get('real') else tm
        kind = term['kind']
        if kind == 'bend':
            d = np.abs(tt - term['ts']) / term['w']
            if 'q' in term:
                # A tent: the lines lean at a near-constant angle inside
                # `w` of the seam and turn at a knee where it ends.
                prof = np.maximum(1.0 - d, 0.0) ** term['q']
            else:
                prof = np.exp(-d)
            phi = phi + across(s, S, term['amount'], term.get('power', 1.0)) * prof
        elif kind == 'fold':
            (x0, y0), (x1, y1) = term['at'], term['to']
            slope = (y1 - y0) / (x1 - x0)
            lo, hi = term['beyond']
            line = y0 + slope * (s - x0)
            line = np.where(s < x0, y0 + lo * (s - x0), line)
            line = np.where(s > x1, y1 + hi * (s - x1), line)
            # Gradual above the ridge, complete below it: the lines lean
            # more and more as they near the ridge and turn there.
            z = np.minimum(tt - line, 0.0)
            phi = phi + across(s, S, term['amount']) * np.exp(z / term['w'])
        elif kind == 'fan':
            fan = term
    if fan is not None:
        g = 1.0 + fan['gain'] * np.exp(-np.abs(tm - fan['ts']) / fan['w'])
        phi = fan['centre'] + (phi - fan['centre']) / g
    return phi


def window(t, w):
    """1 inside [t0, t1], ramping to 0 over `fade` outside it."""
    t0, t1, fade = w
    out = np.ones_like(t)
    if t0 is not None:
        out = np.where(t < t0, 0.0, out) if not fade else np.minimum(out, np.clip((t - t0) / fade + 1.0, 0, 1))
    if t1 is not None:
        out = np.where(t > t1, 0.0, out) if not fade else np.minimum(out, np.clip((t1 - t) / fade + 1.0, 0, 1))
    return out


def line_darkness(phi, pitch, step, core):
    """Gaussian line cores on the iso-lines phi = n * pitch. `step` is
    the sample spacing in design px; `core` the line's full width at
    half maximum in native (2.4/design px) pixels. Darkness in [0, 1]."""
    gy, gx = np.gradient(phi, step)
    grad = np.maximum(np.sqrt(gx * gx + gy * gy), 1e-6)
    u = phi / pitch
    dist = np.abs(u - np.round(u)) * pitch / grad * R    # native px
    sigma = core / 2.3548
    return np.exp(-0.5 * (dist / sigma) ** 2)


def render(spec):
    W, H = spec['size']
    tw, th = int(np.ceil(W * R)), int(np.ceil(H * R))
    rng = np.random.Generator(np.random.PCG64(spec['seed']))
    n = SUPER
    fw, fh = tw * n, th * n
    step = 1.0 / (R * n)                # design px per supersample
    X, Y = np.meshgrid((np.arange(fw) + 0.5) * step, (np.arange(fh) + 0.5) * step)
    s, t, S = (X, Y, W) if spec['axis'] == 'v' else (Y, X, H)
    wv = spec['wave']
    T = H if spec['axis'] == 'v' else W
    wave = Wave(rng, S, T, wv['along'], wv['across'], margin=max(S, T))
    dark = np.zeros((fh, fw))
    for fam in spec['families']:
        d = line_darkness(family_phi(spec, fam, s, t, S, wave), spec['pitch'], step, spec['core'])
        dark = np.maximum(dark, d * window(t, fam['window']))
    dark = dark.reshape(th, n, tw, n).mean(axis=(1, 3))

    # Pore noise, a little longer along the grain, and a faint blotch.
    rng2 = np.random.Generator(np.random.PCG64(spec['seed'] + 7))
    pore = rng2.standard_normal((th, tw))
    ax = 0 if spec['axis'] == 'v' else 1
    pore = (pore + np.roll(pore, 1, ax) + np.roll(pore, -1, ax)) / np.sqrt(3)
    blot = lattice_noise(rng2, None, 9.0, (th, tw))

    # Edge shade: darker within `width` design px of the box's sides.
    ex = (np.arange(tw) + 0.5) / R
    ey = (np.arange(th) + 0.5) / R
    E = spec['edge']
    dx = np.minimum(ex, (W - ex) / E.get('right', 1.0))
    dy = np.minimum(ey, H - ey)
    shade = E['depth'] * np.exp(-np.minimum(dx[None, :], dy[:, None]) / E['width'])

    idx = spec['base_at'] + dark * 175.0 + pore * 5.0 + blot * 4.0 + shade
    return np.clip(np.round(idx), 0, 255).astype(np.uint8)


def ramp(colours, base_at):
    """The 256-entry palette: light -> base over [0, base_at], base ->
    dark over [base_at, 255], interpolated in encoded sRGB."""
    light, base, dark = [np.array([int(c[i:i + 2], 16) for i in (0, 2, 4)], float) for c in colours]
    pal = []
    for i in range(256):
        if i <= base_at:
            f = i / base_at
            c = light + (base - light) * f
        else:
            f = (i - base_at) / (255 - base_at)
            c = base + (dark - base) * f
        pal.append(np.clip(np.round(c), 0, 255).astype(np.uint8))
    return np.array(pal, dtype=np.uint8)


def png_indexed(idx, palette):
    """An 8-bit indexed PNG, written by hand so the bytes depend only on
    the pixels and zlib: no encoder heuristics, no timestamps."""
    h, w = idx.shape

    def chunk(kind, data):
        body = kind + data
        return struct.pack('>I', len(data)) + body + struct.pack('>I', zlib.crc32(body) & 0xffffffff)

    raw = bytearray()
    prev = np.zeros(w, dtype=np.int16)
    for row in idx.astype(np.int16):
        # Filter 2 (Up) everywhere: grain runs along rows or columns and
        # Up compresses both well enough; fixed so output is stable.
        raw.append(2)
        raw.extend(((row - prev) & 0xff).astype(np.uint8).tobytes())
        prev = row
    comp = zlib.compressobj(9, zlib.DEFLATED, 15, 9, zlib.Z_DEFAULT_STRATEGY)
    data = comp.compress(bytes(raw)) + comp.flush()
    ihdr = struct.pack('>IIBBBBB', w, h, 8, 3, 0, 0, 0)
    return (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'PLTE', palette.tobytes())
            + chunk(b'IDAT', data) + chunk(b'IEND', b''))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    ap.add_argument('out', help='directory to write <surface>.png into')
    ap.add_argument('--only', action='append', help='surface name (repeatable)')
    args = ap.parse_args()
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    for name, spec in SURFACES.items():
        if args.only and name not in args.only:
            continue
        idx = render(spec)
        data = png_indexed(idx, ramp(spec['colours'], spec['base_at']))
        (out / f'{name}.png').write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {name}.png  {idx.shape[1]}x{idx.shape[0]}  {len(data)} bytes',
              file=sys.stderr)


if __name__ == '__main__':
    main()
