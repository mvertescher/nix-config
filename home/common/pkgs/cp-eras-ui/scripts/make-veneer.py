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

where `s` runs across the grain and `t` along it. A surface holds one
or more *families* of lines, each its own `phi`, each shown inside a
window along `t`. Where two windows overlap both families are drawn and
their lines cross, which is how the references' seams read. A family
can be the mirror image of the field about a line t = m: the
book-match, two leaves cut from one log. `D` sums:

  * `wave`  -- smooth anisotropic lattice noise: the long, lazy
               cathedral wander (measured: the drift is carried by
               wavelengths of 50-200 design px, not by wiggle);
  * `bend`  -- lines pushed sideways near t = ts by an amount that
               varies across the grain, with profile exp(-(d/w)^p) or a
               tent. Steep enough and the lines turn to run along the
               seam: the streaks and converging seams. A small `w`
               makes the knee sharp, as the references' are;
  * `fold`  -- a shift across a sloping ridge: the panel's figure;
  * `fan`   -- lines spread away from a centre line near t = ts: the
               open book;
  * `knot`  -- a local gaussian bump the lines bend around; `kinks`
               scatters small ones at random (fixed seed).

The lines are then *spaced* irregularly: they are the integer levels of
u = (phi + B(phi, t)) / pitch, where B is smooth noise in phi itself, so
a bunch of close lines stays a bunch as it travels along the grain
(the references' pitch runs 1.4-3.7 design px around a mean of about
2.2). Each line has its own weight, hashed from its index and drifting
slowly along it: some lines are bold, some faint, as in the reference.

Rendering keeps converging lines as lines. A line is an ink stroke of
fixed width in *native* pixels (`core`, FWHM), whatever |grad phi| does;
nearby lines' ink is summed as optical depth, tau = sum ink * w_n *
gauss(dist_n), and the texel is exp(-tau) of the base colour, per
channel (`tint`). So where lines bunch, the light gaps close first while
each line stays sharp, and only where several lines overlap does the
ink compound into a dark core -- what the references show, measured
through the store streak: line minima stay at G ~140 while the gaps
close, then the core falls to G ~57, about 4-7 design px tall. Each
texel is the mean transmission of SUPER x SUPER samples.

Then fine pore noise stretched along the grain, a faint blotch, a mild
edge shade. The palette is the light-to-base ramp below `base_at` and
base * exp(-tau * tint) above it, so index is linear in optical depth.

Output: one 8-bit *indexed* PNG per surface at 2.4 texels per design px
(3840x2160 is 2.4x the 1600x900 design), the ramp stored as its PLTE so
that the same file is what rsvg draws in the traces and what the crate
decodes. Indices run 0 (lightest) to 255 (darkest).

Reproducible: a fixed seed per surface (numpy's PCG64, whose stream is
stable across numpy releases), integer hashing for the per-line weights,
float64 arithmetic in fixed-size chunks, and a hand-written PNG encoder
with fixed zlib settings, so two runs give the same bytes. Run through
the crate's dev shell:

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
SUPER = 4        # supersamples per texel, per axis
NEAR = 4         # lines summed on each side of a sample
CHUNK = 32       # texel rows per chunk (fixed: part of the recipe)

# Shared defaults, overridden per surface.
LINE = dict(core=1.2, ink=0.5, weight=(0.45, 1.25), width=(0.7, 1.15), along=70.0)
SPACING = dict(amp=0.38, cell=6.0, along=150.0)
TINT = (0.45, 1.0, 1.45)

# Per-surface recipes. `size` is the surface's bounding box in design
# px, the texture's extent. `colours` are (light, base) in sRGB hex:
# light at index 0, base at `base_at`; above it the ramp is the base
# darkened by optical depth up to `taumax`. A family's `window` is
# (t0, t1, fade): shown for t0 <= t <= t1, faded in and out over `fade`
# design px (None for an open end).
SURFACES = {
    # Dashboard, the selected EMAIL cascade card (#ncardsel, 90.5 x 325.2).
    # Vertical grain. A second leaf crosses the first at the top-left
    # chamfer (the swirl); a horizontal book-match at t 160 where both
    # leaves hook right into a point at the right edge; the foot bends
    # left along the diagonal cut.
    'dashboard-email': dict(
        size=(90.5, 325.2), axis='v', seed=1101, pitch=2.0,
        colours=('ffd488', 'ffc56e'), base_at=30, taumax=2.2, tint=(0.62, 1.0, 1.4),
        wave=dict(amp=1.6, along=160.0, across=40.0),
        kinks=dict(n=10, amp=1.6, rs=(3.0, 8.0), rt=(4.0, 12.0)),
        families=[
            dict(window=(None, 162.0, 3.0), terms=[
                dict(kind='bend', ts=0.0, w=30.0, amount=(-34.0, -10.0)),
                dict(kind='bend', ts=160.0, w=14.0, c=0.15, amount=(0.0, 34.0), power=1.6),
                dict(kind='bend', ts=160.0, w=1.4, amount=(0.0, 16.0), power=2.5),
            ]),
            dict(mirror=160.0, window=(158.0, None, 3.0), terms=[
                dict(kind='bend', ts=160.0, w=14.0, c=0.15, amount=(0.0, 34.0), power=1.6),
                dict(kind='bend', ts=160.0, w=1.4, amount=(0.0, 16.0), power=2.5),
                dict(kind='bend', ts=325.2, w=34.0, amount=(-26.0, -4.0), real=True),
            ]),
            # The crossing leaves: lines curving the other way, shown
            # 60 px either side of the seam, so the lines cross in a
            # lattice the way the reference's do around the book-match.
            dict(window=(96.0, 154.0, 24.0), weight=0.7, terms=[
                dict(kind='bend', ts=160.0, w=40.0, amount=(0.0, -40.0), power=1.4),
            ]),
            dict(mirror=160.0, window=(166.0, 224.0, 24.0), weight=0.7, terms=[
                dict(kind='bend', ts=160.0, w=40.0, amount=(0.0, -40.0), power=1.4),
            ]),
            dict(mirror=30.0, window=(None, 90.0, 30.0), weight=0.7, terms=[
                dict(kind='bend', ts=0.0, w=16.0, amount=(-34.0, -8.0)),
                dict(kind='bend', ts=30.0, w=10.0, amount=(6.0, 0.0)),
            ]),
        ],
        edge=dict(width=7.0, depth=8.0, right=1.6),
    ),
    # Dashboard, the right-hand EMAIL panel body (230.4 x 309). Vertical
    # grain, calm under the text, folding in the lower third: lines lean
    # down-left above a ridge from local (184, 259) to (84, 299), turn
    # hard at it, and open to a 3.4-3.8 pitch below it.
    'dashboard-panel': dict(
        size=(230.4, 309.0), axis='v', seed=1202, pitch=2.0,
        colours=('ffd68d', 'ffc872'), base_at=30, taumax=2.2,
        line=dict(ink=0.36),
        wave=dict(amp=1.8, along=200.0, across=50.0),
        kinks=dict(n=14, amp=1.4, rs=(3.0, 9.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, None, None), terms=[
                dict(kind='fold', at=(84.0, 299.0), to=(184.0, 259.0), beyond=(-0.25, 0.05),
                     w=32.0, c=0.25, below=70.0, amount=(-10.0, -100.0)),
                dict(kind='bend', ts=0.0, w=30.0, amount=(10.0, 0.0)),
                dict(kind='bend', ts=309.0, w=30.0, amount=(-10.0, -2.0)),
            ]),
        ],
        edge=dict(width=8.0, depth=8.0),
    ),
    # Mailbox, the selected row (477 x 55). Horizontal grain, dense and
    # bunched; the left end's lines rise to the top edge; a vertical
    # open-book seam at t 239 (design x 274): lines from both sides hook
    # into it, a narrow dark crease.
    'mailbox-row': dict(
        size=(477.0, 55.0), axis='h', seed=1303, pitch=1.75,
        colours=('ffe0a4', 'ffd78e'), base_at=30, taumax=2.2,
        line=dict(ink=0.4),
        wave=dict(amp=3.0, along=120.0, across=20.0),
        kinks=dict(n=16, amp=1.2, rs=(2.0, 5.0), rt=(6.0, 18.0)),
        families=[
            dict(window=(None, 239.6, 0.6), terms=[
                dict(kind='bend', ts=0.0, w=24.0, amount=(22.0, 10.0)),
                dict(kind='bend', ts=239.0, w=12.0, c=0.2, amount=(4.0, 40.0), power=1.3),
                dict(kind='bend', ts=239.0, w=1.0, amount=(0.0, 10.0), power=1.5),
            ]),
            dict(mirror=239.0, window=(238.4, None, 0.6), terms=[
                dict(kind='bend', ts=239.0, w=12.0, c=0.2, amount=(4.0, 40.0), power=1.3),
                dict(kind='bend', ts=239.0, w=1.0, amount=(0.0, 10.0), power=1.5),
            ]),
        ],
        edge=dict(width=6.0, depth=8.0),
    ),
    # Store, the selected SMG category (200.6 x 38.6). Horizontal grain,
    # lines pinching into nested arcs at the left end, a book-match at
    # t 100 (design x 193, the button's middle) where the lines bend down
    # into a vertical crease, as the mailbox row's do.
    'store-nav': dict(
        size=(200.6, 38.6), axis='h', seed=1404, pitch=1.8,
        colours=('ffdc9c', 'fccc86'), base_at=30, taumax=2.2,
        wave=dict(amp=2.0, along=100.0, across=20.0),
        kinks=dict(n=6, amp=1.0, rs=(2.0, 5.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 100.6, 0.6), terms=[
                dict(kind='bend', ts=0.0, w=16.0, amount=(14.0, -10.0)),
                dict(kind='bend', ts=100.0, w=10.0, c=0.2, amount=(3.0, 26.0), power=1.3),
                dict(kind='bend', ts=100.0, w=1.0, amount=(0.0, 7.0), power=1.5),
            ]),
            dict(mirror=100.0, window=(99.4, None, 0.6), terms=[
                dict(kind='bend', ts=100.0, w=10.0, c=0.2, amount=(3.0, 26.0), power=1.3),
                dict(kind='bend', ts=100.0, w=1.0, amount=(0.0, 7.0), power=1.5),
                dict(kind='bend', ts=200.6, w=14.0, amount=(-8.0, 8.0), real=True),
            ]),
        ],
        edge=dict(width=5.0, depth=8.0),
    ),
    # Store, the selected card's body and socket band (262.1 x 241.7).
    # Vertical grain with the strong horizontal streak at t 120 (design
    # y 531): the lines run straight until close to it, turn hard and
    # converge into a thin dark core toward the right edge, the two
    # leaves crossing at its left. Below t 192.6 (design 603.8, the
    # socket rule) a separate leaf fans across.
    'store-card': dict(
        size=(262.1, 241.7), axis='v', seed=1505, pitch=2.0,
        colours=('ffe0a0', 'ffd286'), base_at=30, taumax=2.2, sat=1.35,
        spacing=dict(amp=0.5, cell=5.0),
        wave=dict(amp=2.0, along=150.0, across=40.0),
        kinks=dict(n=10, amp=1.4, rs=(3.0, 8.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 122.0, 3.0), terms=[
                dict(kind='shear', ts=44.0, w=6.0, amount=(1.6, 0.9)),
                dict(kind='ripple', ts=40.0, w=4.0, amount=2.6, wavelength=(14.0, 45.0)),
                dict(kind='bend', ts=36.0, w=7.0, c=0.15, amount=(0.0, 22.0), power=2.5),
                dict(kind='bend', ts=36.0, w=1.2, amount=(0.0, 10.0), power=3.0),
                dict(kind='bend', ts=120.0, w=8.0, c=0.1, amount=(0.0, 60.0), power=2.5),
                dict(kind='bend', ts=120.0, w=1.2, amount=(0.0, 28.0), power=2.5),
            ]),
            dict(mirror=120.0, window=(118.0, 192.6, 3.0), terms=[
                dict(kind='bend', ts=120.0, w=8.0, c=0.1, amount=(0.0, 60.0), power=2.5),
                dict(kind='bend', ts=120.0, w=1.2, amount=(0.0, 28.0), power=2.5),
            ]),
            dict(window=(192.6, None, None), terms=[
                dict(kind='bend', ts=192.6, w=16.0, c=0.3, amount=(4.0, 26.0), power=1.5, real=True),
                dict(kind='bend', ts=241.7, w=18.0, amount=(8.0, 14.0), real=True),
            ]),
        ],
        edge=dict(width=6.0, depth=8.0),
    ),
    # Store, the BASKET plate's lower band (204.6 x 45). Horizontal
    # grain with a book-match at t 85 (design x 1377): a sharp vertical
    # crease the lines barely bend into; they rise at the right end.
    'store-basket': dict(
        size=(204.6, 45.0), axis='h', seed=1606, pitch=1.8,
        colours=('ffc77e', 'fab56b'), base_at=30, taumax=2.2,
        wave=dict(amp=2.0, along=100.0, across=20.0),
        kinks=dict(n=6, amp=1.0, rs=(2.0, 5.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 85.5, 0.5), terms=[
                dict(kind='bend', ts=85.0, w=6.0, c=0.3, amount=(2.0, 8.0)),
                dict(kind='bend', ts=85.0, w=0.8, amount=(1.0, 5.0)),
                dict(kind='bend', ts=0.0, w=14.0, amount=(6.0, -10.0), real=True),
            ]),
            dict(mirror=85.0, window=(84.5, None, 0.5), terms=[
                dict(kind='bend', ts=85.0, w=6.0, c=0.3, amount=(2.0, 8.0)),
                dict(kind='bend', ts=85.0, w=0.8, amount=(1.0, 5.0)),
                dict(kind='bend', ts=204.6, w=22.0, amount=(14.0, 4.0), real=True),
            ]),
        ],
        edge=dict(width=5.0, depth=8.0),
    ),
}


def lattice_noise(rng, cell, out_shape):
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


class Lattice:
    """Smooth 2-D value noise over a rectangle of (a, b) space, `octaves`
    of it, each half the cell of the last and `gain` times the amplitude.
    Evaluated anywhere; outside its extent it clamps."""

    def __init__(self, rng, lo, hi, cell, octaves=1, gain=0.5):
        self.lo, self.cell, self.gain = lo, cell, gain
        self.grids = []
        for o in range(octaves):
            f = 2.0 ** o
            n = [int(np.ceil((hi[i] - lo[i]) * f / cell[i])) + 3 for i in (0, 1)]
            self.grids.append(rng.uniform(-1.0, 1.0, size=(n[1], n[0])))

    def __call__(self, a, b):
        total = np.zeros_like(a)
        norm = 0.0
        for o, grid in enumerate(self.grids):
            f = 2.0 ** o
            u = (a - self.lo[0]) * f / self.cell[0]
            v = (b - self.lo[1]) * f / self.cell[1]
            u0 = np.clip(np.floor(u).astype(int), 0, grid.shape[1] - 2)
            v0 = np.clip(np.floor(v).astype(int), 0, grid.shape[0] - 2)
            fu = np.clip(u - u0, 0.0, 1.0)
            fv = np.clip(v - v0, 0.0, 1.0)
            su, sv = fu * fu * (3 - 2 * fu), fv * fv * (3 - 2 * fv)
            top = grid[v0, u0] + (grid[v0, u0 + 1] - grid[v0, u0]) * su
            bot = grid[v0 + 1, u0] + (grid[v0 + 1, u0 + 1] - grid[v0 + 1, u0]) * su
            amp = self.gain ** o
            total += amp * (top + (bot - top) * sv)
            norm += amp
        return total / norm


def across(s, S, amount, power=1.0):
    """Across the grain: amount[0] at s = 0, amount[1] at s = S, linear
    or, with `power` > 1, gathered towards s = S."""
    a0, a1 = amount
    return a0 + (a1 - a0) * np.clip(s / S, 0.0, 1.0) ** power


def hash01(n, j, seed):
    """A uniform [0, 1) per (line index, along-grain cell), from integer
    arithmetic only (splitmix64), so it is the same on every machine."""
    x = (n.astype(np.int64).astype(np.uint64) * np.uint64(0x9E3779B97F4A7C15)
         + j.astype(np.int64).astype(np.uint64) * np.uint64(0xBF58476D1CE4E5B9)
         + np.uint64(seed))
    x = (x ^ (x >> np.uint64(30))) * np.uint64(0xBF58476D1CE4E5B9)
    x = (x ^ (x >> np.uint64(27))) * np.uint64(0x94D049BB133111EB)
    x = x ^ (x >> np.uint64(31))
    return (x >> np.uint64(11)).astype(np.float64) / float(1 << 53)


class Surface:
    """Everything random about one surface, drawn once from its seed."""

    def __init__(self, spec):
        self.spec = spec
        W, H = spec['size']
        self.S, self.T = (W, H) if spec['axis'] == 'v' else (H, W)
        rng = np.random.Generator(np.random.PCG64(spec['seed']))
        m = max(self.S, self.T)
        wv = spec['wave']
        self.wave = Lattice(rng, (-m, -m), (self.S + m, self.T + m),
                            (wv['across'], wv['along']), octaves=wv.get('octaves', 2),
                            gain=wv.get('gain', 0.35))
        sp = dict(SPACING, **spec.get('spacing', {}))
        self.spacing = sp
        lo = -m - 200.0
        self.space = Lattice(rng, (lo, -m), (self.S + m + 200.0, self.T + m),
                             (sp['cell'], sp['along']), octaves=2, gain=0.5)
        self.knots = []
        k = spec.get('kinks')
        if k:
            for _ in range(k['n']):
                s0 = rng.uniform(0.0, self.S)
                t0 = rng.uniform(0.0, self.T)
                a = rng.uniform(0.5, 1.0) * k['amp'] * (1 if rng.uniform() < 0.5 else -1)
                rs = rng.uniform(*k['rs'])
                rt = rng.uniform(*k['rt'])
                self.knots.append((s0, t0, rs, rt, a))
        self.line = dict(LINE, **spec.get('line', {}))


def family_phi(surf, fam, s, t):
    """phi of one family, in design px. A mirrored family reads the
    field -- the wander included -- at the reflected along-grain
    coordinate; a term marked `real` is placed in the surface's own
    coordinates even so."""
    spec, S = surf.spec, surf.S
    m = fam.get('mirror')
    tm = 2 * m - t if m is not None else t
    phi = s + spec['wave']['amp'] * surf.wave(s, tm)
    for (s0, t0, rs, rt, a) in surf.knots:
        phi = phi + a * np.exp(-((s - s0) / rs) ** 2 - ((tm - t0) / rt) ** 2)
    fan = None
    for term in fam['terms']:
        tt = t if term.get('real') else tm
        kind = term['kind']
        if kind == 'bend':
            d = np.abs(tt - term['ts']) / term['w']
            if 'c' in term:
                # A knee: exponential at the seam (the lines keep a
                # slope there), gaussian beyond it, so they run straight
                # until close and then turn hard.
                prof = np.exp(-(d * d + 2.0 * term['c'] * d))
            elif 'q' in term:
                # A tent: the lines lean at a near-constant angle inside
                # `w` of the seam and turn at a knee where it ends.
                prof = np.maximum(1.0 - d, 0.0) ** term['q']
            else:
                prof = np.exp(-d ** term.get('p', 1.0))
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
            z = np.maximum(line - tt, 0.0) / term['w']
            if 'c' in term:
                prof = np.exp(-(z * z + 2.0 * term['c'] * z))
            else:
                prof = np.exp(-z ** term.get('p', 1.0))
            if 'below' in term:
                # Below the ridge the shift relaxes again over `below`:
                # the lines lean back the other way, their pitch opening.
                zb = np.maximum(tt - line, 0.0) / term['below']
                prof = np.where(tt < line, prof, np.exp(-zb))
            phi = phi + across(s, S, term['amount']) * prof
        elif kind == 'shear':
            # One-sided: beyond ts, on the side t < ts, the lines lean
            # at `amount` (ds/dt); on the other side they run straight;
            # a soft knee `w` wide between: grain running off an edge.
            z = (term['ts'] - tt) / term['w']
            knee = term['w'] * np.logaddexp(0.0, z)
            phi = phi + across(s, S, term['amount'], term.get('power', 1.0)) * knee
        elif kind == 'ripple':
            # Cathedral zigzag: a sideways wave along the grain, on the
            # side t < ts (soft edge `w`), its phase drifting across it.
            env = 1.0 / (1.0 + np.exp((tt - term['ts']) / term['w']))
            lam, lam_s = term['wavelength']
            vary = 0.6 + 0.6 * surf.wave(S - s, surf.T - tt)
            phase = tt / lam + s / lam_s + 0.35 * surf.wave(s, surf.T - tt)
            phi = phi + term['amount'] * env * vary * np.sin(2.0 * np.pi * phase)
        elif kind == 'knot':
            (s0, t0), (rs, rt) = term['at'], term['r']
            phi = phi + term['amount'] * np.exp(-((s - s0) / rs) ** 2 - ((tt - t0) / rt) ** 2)
        elif kind == 'fan':
            fan = term
    if fan is not None:
        g = 1.0 + fan['gain'] * np.exp(-np.abs(tm - fan['ts']) / fan['w'])
        phi = fan['centre'] + (phi - fan['centre']) / g
    # Irregular spacing: B is noise in phi itself, so a bunch of lines
    # stays a bunch along the grain. |dB/dphi| <= amp keeps it monotone.
    sp = surf.spacing
    phi = phi + sp['amp'] * sp['cell'] / 3.0 * surf.space(phi, tm)
    return phi, tm


def window(t, w):
    """1 inside [t0, t1], ramping to 0 over `fade` outside it."""
    t0, t1, fade = w
    out = np.ones_like(t)
    if t0 is not None:
        out = np.where(t < t0, 0.0, out) if not fade else np.minimum(out, np.clip((t - t0) / fade + 1.0, 0, 1))
    if t1 is not None:
        out = np.where(t > t1, 0.0, out) if not fade else np.minimum(out, np.clip((t1 - t) / fade + 1.0, 0, 1))
    return out


def family_tau(surf, fam, s, t, step, pad):
    """Optical depth of one family's ink on a sample grid (with `pad`
    extra rows above and below for the gradient). Each line is a
    gaussian stroke `core` native px wide at half depth, whatever the
    line spacing; the NEAR lines either side are summed, so bunched
    lines keep their own cores and the ink compounds only where they
    overlap."""
    L = surf.line
    pitch = surf.spec['pitch']
    phi, tm = family_phi(surf, fam, s, t)
    gy, gx = np.gradient(phi, step)
    grad = np.maximum(np.sqrt(gx * gx + gy * gy), 1e-6)
    if pad:
        phi, grad, tm, t = phi[pad:-pad], grad[pad:-pad], tm[pad:-pad], t[pad:-pad]
    u = phi / pitch
    n0 = np.floor(u)
    scale = pitch / grad * R                    # native px per unit of u
    inv = 1.0 / (L['core'] / 2.3548)
    j = np.floor(tm / L['along'])
    f = tm / L['along'] - j
    f = f * f * (3 - 2 * f)
    lo, hi = L['weight']
    wlo, whi = L['width']
    seed = surf.spec['seed']
    tau = np.zeros_like(u)
    for k in range(-NEAR + 1, NEAR + 1):
        n = n0 + k
        d = (u - n) * scale * inv
        hit = np.abs(d) < 6.0
        if not hit.any():
            continue
        w = hash01(n, j, seed) * (1 - f) + hash01(n, j + 1, seed) * f
        w = lo + (hi - lo) * w
        # Each line its own width too (measured FWHM p10-p90 about
        # 1.2-2.5 native px): the same ink spread thinner or wider.
        wide = wlo + (whi - wlo) * hash01(n, n * 0, seed + 1)
        dd = d / wide
        tau += np.where(hit, w * np.exp(-0.5 * np.minimum(dd * dd, 36.0)), 0.0)
    return tau * L['ink'] * fam.get('weight', 1.0) * window(t, fam['window'])


def render(spec):
    W, H = spec['size']
    tw, th = int(np.ceil(W * R)), int(np.ceil(H * R))
    surf = Surface(spec)
    n = SUPER
    step = 1.0 / (R * n)                # design px per supersample
    xs = (np.arange(tw * n) + 0.5) * step
    trans = np.zeros((th, tw))
    pad = 1
    for r0 in range(0, th, CHUNK):
        r1 = min(r0 + CHUNK, th)
        rows = np.arange(r0 * n - pad, r1 * n + pad)
        ys = (rows + 0.5) * step
        X, Y = np.meshgrid(xs, ys)
        s, t = (X, Y) if spec['axis'] == 'v' else (Y, X)
        tau = np.zeros((rows.size - 2 * pad, xs.size))
        for fam in spec['families']:
            tau += family_tau(surf, fam, s, t, step, pad)
        # Ink saturates: a few overlapping strokes darken, a pile of
        # them does not go black (the references' cores stop at about
        # G 57, tau ~1.3).
        sat = spec.get('sat', 1.6)
        T = np.exp(-sat * (1.0 - np.exp(-tau / sat)))
        trans[r0:r1] = T.reshape(r1 - r0, n, tw, n).mean(axis=(1, 3))
    tau = -np.log(np.maximum(trans, 1e-9))

    # Pore noise, a little longer along the grain, and a faint blotch.
    rng2 = np.random.Generator(np.random.PCG64(spec['seed'] + 7))
    pore = rng2.standard_normal((th, tw))
    ax = 0 if spec['axis'] == 'v' else 1
    pore = (pore + np.roll(pore, 1, ax) + np.roll(pore, -1, ax)) / np.sqrt(3)
    blot = lattice_noise(rng2, 9.0, (th, tw))

    # Edge shade: darker within `width` design px of the box's sides.
    ex = (np.arange(tw) + 0.5) / R
    ey = (np.arange(th) + 0.5) / R
    E = spec['edge']
    dx = np.minimum(ex, (W - ex) / E.get('right', 1.0))
    dy = np.minimum(ey, H - ey)
    shade = E['depth'] * np.exp(-np.minimum(dx[None, :], dy[:, None]) / E['width'])

    span = 255 - spec['base_at']
    idx = spec['base_at'] + tau / spec['taumax'] * span + pore * 2.4 + blot * 1.5 + shade / 3.0
    return np.clip(np.round(idx), 0, 255).astype(np.uint8)


def ramp(spec):
    """The 256-entry palette: light -> base over [0, base_at] (linear in
    encoded sRGB), then the base darkened by optical depth tau =
    (i - base_at) / (255 - base_at) * taumax, channel c by exp(-tau *
    tint[c]): ink that takes more blue than red, as measured."""
    light, base = [np.array([int(c[i:i + 2], 16) for i in (0, 2, 4)], float) for c in spec['colours']]
    tint = np.array(spec.get('tint', TINT))
    b = spec['base_at']
    pal = []
    for i in range(256):
        if i <= b:
            c = light + (base - light) * (i / b)
        else:
            tau = (i - b) / (255 - b) * spec['taumax']
            c = base * np.exp(-tau * tint)
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
        data = png_indexed(idx, ramp(spec))
        (out / f'{name}.png').write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {name}.png  {idx.shape[1]}x{idx.shape[0]}  {len(data)} bytes',
              file=sys.stderr)


if __name__ == '__main__':
    main()
