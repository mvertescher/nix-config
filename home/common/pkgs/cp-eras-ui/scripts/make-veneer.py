"""Generate the Neo-kitsch wood-veneer textures.

Owner decision 2026-11-13 (docs/review-2026-11-13/DECISIONS.md, 3): the
selected surfaces wear a pre-rendered texture clipped to their shapes,
not drawn vector grain. The reference art is third-party and never
redistributed, so nothing here is copied from it: every texel is
computed from the parameters below, which were *measured* off the
references (docs/neokitsch/veneer-texture.md and veneer-construction.md
have the numbers) and then tuned by eye. This script reads no image.

Third pass (2026-11-15): the noise-field *figure* of the first two passes
drawn with the *line, ground and grain* measured for the blend generator.

1. Figure. A surface holds line *families*, each a scalar field

       phi(s, t) = s + D(s, t)

   (`s` across the grain, `t` along it) shown inside a window along `t`.
   A family can be the mirror of the field about t = m: the book-match,
   placed on each surface's measured axis; where two windows overlap
   both leaves are drawn and cross. `D` sums a long lattice wander,
   random small `kinks`, and the figure terms: `bend` (a shift near
   t = ts, profile exp(-(d/w)^p), or a knee exp(-(d^2 + 2cd)) whose
   corner is a crease), `lean` (one-sided: past `ts`, which may vary
   across the grain, the lines run at slope `amount`, steepening by
   `accel` -- the run into a streak, crossing the axis), `ramp` (a band
   between two curves across which the lines step sideways: the panel's
   sweep and crease), `fold` (a ridge), `shear` (lean off an edge),
   `ripple` (zigzag), `knot`. Amounts and curves may be piecewise linear
   across the grain ([(s, value), ...]). A family may also be cut to a
   strip across the grain (`swindow`); its lines end at the cut, a jog.
   With `trim` its lines end exactly on its box, so two strips whose
   fields agree meet end to end without a dotted seam.
   A term marked `comb` ('main' or 'twin') shapes that comb only, so the
   other runs straight through it and the two cross (the EMAIL card's
   top arcs).

2. Lines. The lines are the levels phi = (k + offset) * pitch: constant
   pitch away from the figure, `pitch` the measured per-comb spacing
   (native px / R), and a `twin` comb at `offset` (the measured gap ratio)
   whose figure sits `shift` px further along the grain, so the two
   combs knee on alternate rows. Each level is traced as a contour from
   where it crosses its family's box (predictor-corrector, steps that
   halve at sharp turns and step straight through a crease).

3. Straight runs and knees. A traced line keeps its direction in
   `step`-degree bins (30, the measured 25-35 deg knee) with hysteresis;
   where the bin changes it gets a vertex, so neighbouring lines knee on
   one row (the fold), and the run between is a chord, split only where
   it strays more than `tol`. Every corner is filleted at `radius`
   texels (measured 8-11).

4. Rendering, as measured. Each line is a 1-texel box stroke composited
   *over* the ground towards the fixed ink CORE with per-channel opacity
   (ground - single) / (ground - CORE), so overlapping lines compound
   towards CORE and stop; the ground is flat (no tone field), plus
   per-channel grain (normal noise through [1, 3, 1], measured std).
   `look` holds ground / single line / grain per surface.

Output: one 8-bit *indexed* PNG per surface at 2.4 texels per design px
(3840x2160 samples it 1:1), the palette a deterministic 256-colour k-means
of the rendered RGB. Reproducible: fixed seeds (numpy's PCG64), float64,
fixed iteration orders, and a hand-written PNG encoder with fixed zlib
settings, so two runs give the same bytes. Run through the crate's dev
shell:

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
PERIM = 0.02     # design px between perimeter samples when seeding lines
STEP = 0.5       # design px per tracing step along a line

SPACING = dict(amp=0.0, cell=6.0, along=150.0)
# Straightening: direction bins (deg), hysteresis (deg), chord tolerance
# (design px) and the knee fillet radius (texels), measured 8-11.
QUANT = dict(step=30.0, hyst=3.0, tol=0.5, radius=9.5)
CORE = (150.0, 61.0, 30.0)   # the store streak core, darkest compounded ink

# The store streak (2026-11-15 refit): the lean's acceleration and the
# core bend peak at s ~208-235 (native crop x 500-565) and ease off at the
# right edge, so the core is darkest mid-card and thins to the right, as
# the reference's spindle does (it was thickest at the edge).
STREAK_LEAN = dict(kind='lean', ts=(110.0, 115.5), w=1.2, amount=(-2.8, -3.0),
                   accel=[(0.0, 0.0), (150.0, -0.45), (208.0, -1.6), (235.0, -1.6), (262.0, -1.3)])
STREAK_BEND = dict(kind='bend', ts=121.46, w=16.0, p=2.0,
                   amount=[(0.0, 0.0), (200.0, -3.0), (235.0, -4.0), (262.0, -3.5)])

# The EMAIL card (2026-11-15 second refinement). LEAN_L is the lower leaf's
# lean in its mirrored coordinates; the two are not symmetric, so the
# crossing band sits a little above the axis, as the reference's does.
EMAIL_LEAN_U = dict(kind='lean', ts=(122.0, 110.0), w=6.0, amount=(-0.3, -0.15))
EMAIL_LEAN_L = dict(kind='lean', ts=(140.0, 118.0), w=6.0, amount=(-0.3, -0.15))
EMAIL_CUSP = dict(kind='bend', ts=150.8, w=18.0, c=0.6, amount=(0.0, -50.0), power=3.0)
EMAIL_EDGE = dict(kind='knot', at=(97.0, 160.0), r=(14.0, 1e6), amount=5.0, real=True)
EMAIL_TOP = dict(kind='bend', ts=70.0, w=110.0, p=2.0, amount=(0.0, 30.0), real=True)
EMAIL_SWIRL = dict(kind='shear', ts=55.0, w=15.0, amount=(0.0, 0.2), accel=(0.012, 0.02), comb='main')
EMAIL_SWIRL_TWIN = dict(kind='shear', ts=55.0, w=15.0, amount=(0.0, -0.1), accel=(-0.008, -0.012), comb='twin')
# Both EMAIL leaves: 10-degree direction bins (the 30-degree knees drew the
# swirl's arcs and the band's crossing lines as chords), and the crossing
# comb fades over 10 px at its window end instead of stopping on a row.
EMAIL_QUANT = dict(step=10.0, hyst=1.5)
MAIL_STEP = [(0.0, 50.0), (17.0, 70.0), (32.0, 60.0), (55.0, 35.0)]
MAIL_SEAM = dict(kind='bend', ts=238.1, w=6.0, p=2.0, amount=(-1.0, -12.0))
MAIL_CREASE = dict(kind='bend', ts=238.1, w=2.0, p=2.0, amount=(0.0, -4.0), power=1.5)
SMG_SEAM = dict(kind='bend', ts=99.6, w=6.0, p=2.0, amount=(4.0, 9.0))
SMG_CREASE = dict(kind='bend', ts=99.6, w=1.5, p=2.0, amount=(1.0, 3.0))
BASKET_SEAM = dict(kind='bend', ts=85.0, w=6.0, p=2.0, amount=(4.5, 5.5))
BASKET_CREASE = dict(kind='bend', ts=85.0, w=1.5, p=2.0, amount=(2.5, 3.0))

# The dashboard panel's lower leaf (2026-11-15 third refinement), traced
# off the reference with a structure tensor (native 4K crop / R = design
# px). Each line is an S: vertical, a knee down-LEFT into PANEL_SWEEP (45
# deg at the left edge, ~75 mid-panel), a steep run, a second knee into
# PANEL_UNDER_* and, in the bottom-left corner, horizontal to the edge.
# The steep row slopes down to the right at ~0.4 (native (10, 620) to
# (210, 710)); PANEL_UNDER_L's band follows it with a constant amount, so
# phi's slope across the grain cancels there (1 - amount / width * 0.4 is
# ~0) and the corner's lines run horizontal at an even spacing instead of
# bunching (the old R2's amount fell 2.2 per px across s 79-94, which
# packed them into a dark clump and made the stepped kinks at native x
# ~135 / ~200). Right of s 78 (the reference's jog at native x 187, its
# lower end only: above PANEL_UNDER_* the two strips' fields agree and
# their lines meet end to end, `trim`) PANEL_UNDER_R only leans the lines
# ~30 deg. PANEL_CREASE is a thin band (2.5 px) the lines cross almost
# along: the reference's thin dark crease line from native (200, 664) to
# the strip edge at (450, 615).
PANEL_UNDER = [(0.0, 265.0), (35.4, 278.3), (66.7, 290.8), (87.5, 300.8), (108.3, 300.8),
               (139.6, 298.3), (170.8, 294.2), (187.5, 295.0)]
PANEL_SWEEP = dict(kind='ramp', k=[(0.0, 2.0), (110.0, 2.0), (150.0, 4.0), (187.5, 5.0)], a=[(14.6, 220.8), (56.2, 225.0), (97.9, 230.4), (139.6, 248.0), (170.8, 256.0), (187.5, 255.0)], b=[(4.2, 250.0), (35.4, 262.5), (66.7, 275.0), (87.5, 286.7), (108.3, 288.3), (139.6, 289.6), (170.8, 291.7), (187.5, 293.8)], amount=[(4.2, 35.0), (35.4, 55.0), (66.7, 90.0), (108.3, 130.0), (139.6, 115.0), (181.0, 122.0), (187.5, 107.0)])
PANEL_CREASE = dict(kind='ramp', k=1.0,
                    a=[(83.3, 276.7), (113.8, 267.5), (139.6, 264.2), (170.8, 259.2), (187.5, 256.2)],
                    b=[(83.3, 279.2), (113.8, 270.0), (139.6, 266.7), (170.8, 261.7), (187.5, 258.8)],
                    amount=[(75.0, 0.0), (104.2, 18.0), (187.5, 18.0)])
PANEL_EDGE = dict(kind='ramp', k=3.0, a=[(0.0, 232.0)], b=[(0.0, 252.0)], amount=[(181.0, 0.0), (187.5, 15.0)])
PANEL_RIGHT = dict(kind='ramp', k=3.0, a=[(187.5, 260.4), (230.4, 266.7)], b=[(187.5, 291.7), (230.4, 316.7)], amount=[(187.5, 22.0), (230.4, 0.0)])
PANEL_UNDER_L = dict(kind='ramp', k=3.0, a=PANEL_UNDER, b=[(s, t + 60.0) for s, t in PANEL_UNDER],
                     amount=[(0.0, 170.0), (77.9, 170.0)])
PANEL_UNDER_R = dict(kind='ramp', k=3.0, a=PANEL_UNDER, b=[(s, t + 62.5) for s, t in PANEL_UNDER],
                     amount=[(77.9, 45.0), (187.5, 40.0)])

# The store card's upper streak (2026-11-15 third refinement). In the
# reference the lines above it run down-right at 45-75 deg, flatten to
# ~80 deg in a dark band at native y 70-100 (TOP_SWEEP: a ramp the lines
# cross almost along, which packs them) and knee back to vertical on a
# row that rises towards the right edge, native y 140 at x 200 to y 95 at
# the edge (TOP_KNEE). The old shear at t 34 plus a bend at t 42 drew a
# zigzag that leaned only ~60 deg and curved the wrong way at the right.
TOP_EDGE = [(0.0, 102 / R), (170.0, 100 / R), (262.1, 88 / R)]
TOP_SWEEP = dict(kind='ramp', k=2.0, a=[(0.0, 70 / R), (262.1, 78 / R)], b=TOP_EDGE,
                 amount=[(0.0, -75.0), (262.1, -75.0)])
TOP_KNEE = dict(kind='ramp', k=3.0, a=TOP_EDGE,
                b=[(83.0, 140 / R), (140.0, 136 / R), (181.0, 120 / R), (202.0, 106 / R), (233.0, 99 / R),
                   (262.1, 95 / R)],
                amount=[(0.0, -25.0), (262.1, -25.0)])

SURFACES = {
    # Store, the selected card's body and socket band (262.1 x 241.7).
    # Vertical grain; the streak at t 121.46 (texel 291.5) is a book-match:
    # each leaf knees into a lean that steepens towards the right edge and
    # crosses the axis, so the leaves cross in a lattice and pile into a
    # core at the right. A smaller streak near the top right (t 42), a
    # slanted zigzag top, and below t 192.6 a separate leaf.
    'store-card': dict(
        size=(262.1, 241.7), axis='v', seed=1505, pitch=11.6 / R,
        twin=dict(offset=0.5, shift=-2.0),
        look=dict(ground=(262, 206, 131), single=(211, 144, 67), grain=(3.7, 5.3, 6.4)),
        wave=dict(amp=2.0, along=150.0, across=40.0),
        kinks=dict(n=10, amp=1.4, rs=(3.0, 8.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 123.0), terms=[
                dict(kind='shear', ts=29.0, w=4.0, amount=(2.0, 1.5)),
                dict(kind='ripple', ts=32.0, w=4.0, amount=1.8, wavelength=(14.0, 45.0)),
                TOP_SWEEP, TOP_KNEE, STREAK_LEAN, STREAK_BEND,
            ]),
            dict(mirror=121.46, window=(119.9, 192.6), terms=[STREAK_LEAN, STREAK_BEND]),
            dict(window=(192.6, None), terms=[
                dict(kind='bend', ts=192.6, w=16.0, c=0.3, amount=(4.0, 26.0), power=1.5, real=True),
                dict(kind='bend', ts=241.7, w=18.0, amount=(8.0, 14.0), real=True),
            ]),
        ],
    ),
    # Dashboard, the selected EMAIL cascade card (#ncardsel, 90.5 x 325.2).
    # Vertical grain, denser in the upper half (EMAIL_TOP) and towards the
    # right edge (EMAIL_EDGE); book-match about t 150.8 (texel 362). Both
    # leaves lean into the seam (EMAIL_LEAN_*: about 15-20 deg) and on
    # across it, so at the left they cross in a lattice band from t ~125
    # to ~160 (texel 300-384, the reference's); towards the right edge
    # EMAIL_CUSP, a knee at the axis gathered to the edge (power 3), turns
    # them into a spindle whose point is on the right edge at the axis.
    # At the top the main comb alone shears up-left above t 44
    # (EMAIL_SWIRL, `comb`), the twin running on straight through it: the
    # reference's crossing arcs under the chamfer. The foot leans a little
    # left along the diagonal cut.
    'dashboard-email': dict(
        size=(90.5, 325.2), axis='v', seed=1101, pitch=8.9 / R,
        twin=dict(offset=0.62, shift=-1.0),
        look=dict(ground=(262, 191, 108), single=(199, 138, 65), grain=(3.7, 4.8, 5.2)),
        wave=dict(amp=1.6, along=160.0, across=40.0),
        kinks=dict(n=10, amp=1.2, rs=(3.0, 8.0), rt=(4.0, 12.0)),
        families=[
            dict(window=(None, 160.0), fade=(0.0, 10.0), quant=EMAIL_QUANT,
                 terms=[EMAIL_LEAN_U, EMAIL_CUSP, EMAIL_EDGE, EMAIL_TOP, EMAIL_SWIRL, EMAIL_SWIRL_TWIN]),
            dict(mirror=150.8, window=(125.0, None), fade=(10.0, 0.0), quant=EMAIL_QUANT, terms=[
                EMAIL_LEAN_L, EMAIL_CUSP, EMAIL_EDGE, EMAIL_TOP,
                dict(kind='bend', ts=325.2, w=34.0, amount=(8.0, 0.0), real=True),
            ]),
        ],
    ),
    # Dashboard, the right-hand EMAIL panel body (230.4 x 309). Vertical
    # grain, book-matched about its middle (t 154.4, texel 370.5). The lower
    # leaf's fold (2026-11-15 refit, traced off the reference): the lines
    # knee down-LEFT at a row from (15, 221) to (188, 261) and sweep nearly
    # horizontal (ramp A1) to a crease from (83, 287) up to the tip at
    # (188, 262); below it they run on down-left (ramp R2), and in the
    # bottom-left corner nearly horizontal again, with a jog at s ~85
    # where R2's amount drops. Right of s 187.5 a separate strip (the
    # reference's vertical jog at native x ~450) whose lines only knee
    # to a gentle lean below the crease tip. A1's amount tapers towards
    # the strip edge by less than 1 per px, so the field stays monotone
    # across the grain and few lines enter the crease from the strip
    # edge. The upper leaf mirrors the grain but not the fold, which the
    # reference's top half lacks.
    'dashboard-panel': dict(
        size=(230.4, 309.0), axis='v', seed=1202, pitch=10.2 / R,
        twin=dict(offset=0.42, shift=-1.5),
        look=dict(ground=(262, 194, 110), single=(220, 158, 79), grain=(4.1, 4.6, 4.9), width=1.3),
        wave=dict(amp=1.8, along=200.0, across=50.0),
        kinks=dict(n=10, amp=1.0, rs=(3.0, 9.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(153.4, None), swindow=(None, 77.7), trim=True, terms=[PANEL_EDGE, PANEL_SWEEP, PANEL_CREASE, PANEL_UNDER_L]),
            dict(window=(153.4, None), swindow=(78.1, 187.6), trim=True, terms=[PANEL_EDGE, PANEL_SWEEP, PANEL_CREASE, PANEL_UNDER_R]),
            dict(window=(153.4, None), swindow=(187.4, None), terms=[
                PANEL_RIGHT,
                dict(kind='ramp', k=3.0, a=[(187.5, 291.7), (230.4, 316.7)], b=[(0.0, 341.7)],
                     amount=[(187.5, 20.0), (230.4, 20.0)]),
            ]),
            dict(mirror=154.4, window=(None, 155.4), terms=[
                dict(kind='bend', ts=0.0, w=30.0, amount=(10.0, 0.0), real=True),
            ]),
        ],
    ),
    # Mailbox, the selected row (477 x 55). Horizontal grain; at the left
    # end the lines rise to the top edge and step down across a band that
    # bulges right at mid-height (the ramp, centre MAIL_STEP, 36 wide);
    # book-match about t 238.1 (texel 571.5): lines from both sides curve
    # DOWN into it (the reference's arcs; the third pass had them rising),
    # more towards the bottom edge.
    'mailbox-row': dict(
        size=(477.0, 55.0), axis='h', seed=1303, pitch=5.7 / R,
        twin=dict(offset=0.74),
        look=dict(ground=(258, 202, 130), single=(207, 146, 83), grain=(3.4, 5.0, 6.1)),
        wave=dict(amp=2.0, along=120.0, across=20.0),
        kinks=dict(n=10, amp=0.8, rs=(2.0, 5.0), rt=(6.0, 18.0)),
        families=[
            dict(window=(None, 239.0), terms=[
                dict(kind='bend', ts=-3.0, w=14.0, amount=(32.0, 12.0)),
                dict(kind='ramp', k=5.0, a=[(x, t - 18.0) for x, t in MAIL_STEP],
                     b=[(x, t + 18.0) for x, t in MAIL_STEP], amount=[(0.0, -16.0), (55.0, -16.0)]),
                MAIL_SEAM, MAIL_CREASE,
            ]),
            dict(mirror=238.1, window=(237.2, None), terms=[MAIL_SEAM, MAIL_CREASE]),
        ],
    ),
    # Store, the selected SMG category (200.6 x 38.6). Horizontal grain,
    # lines pinching at the left end; book-match about t 99.6 (texel 239,
    # the button's middle) where the lines rise into a narrow crease
    # (about +-10 texels, 20-45 deg, as measured); at the right end they
    # all fall towards the right edge.
    'store-nav': dict(
        size=(200.6, 38.6), axis='h', seed=1404, pitch=4.9 / R,
        twin=dict(offset=0.61),
        look=dict(ground=(249, 198, 128), single=(202, 140, 74), grain=(6.0, 5.2, 6.2)),
        wave=dict(amp=1.6, along=100.0, across=20.0),
        kinks=dict(n=6, amp=0.8, rs=(2.0, 5.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 100.4), terms=[
                dict(kind='bend', ts=-3.0, w=16.0, amount=(17.0, -12.0)),
                SMG_SEAM, SMG_CREASE,
            ]),
            dict(mirror=99.6, window=(98.8, None), terms=[
                SMG_SEAM, SMG_CREASE,
                dict(kind='bend', ts=200.6, w=20.0, p=2.0, amount=(-16.0, -8.0), real=True),
            ]),
        ],
    ),
    # Store, the BASKET plate's lower band (204.6 x 45). Horizontal grain,
    # one comb, a crease at t 85 (design x 1377) that the lines bow up
    # into evenly top to bottom -- two leaves meeting, not a mirror (none
    # was measured). At the left end the lower lines fall to the edge; at
    # the right end the lines fall towards the edge, most at the bottom.
    'store-basket': dict(
        size=(204.6, 45.0), axis='h', seed=1606, pitch=4.4 / R,
        look=dict(ground=(249, 181, 107), single=(202, 120, 79), grain=(6.0, 6.0, 6.0)),
        wave=dict(amp=1.6, along=100.0, across=20.0),
        kinks=dict(n=6, amp=0.8, rs=(2.0, 5.0), rt=(5.0, 14.0)),
        families=[
            dict(window=(None, 85.5), terms=[
                BASKET_SEAM, BASKET_CREASE,
                dict(kind='bend', ts=-3.0, w=14.0, amount=(0.0, -18.0), real=True),
            ]),
            dict(window=(84.5, None), terms=[
                BASKET_SEAM, BASKET_CREASE,
                dict(kind='bend', ts=204.6, w=26.0, p=2.0, amount=(-4.0, -18.0), real=True),
            ]),
        ],
    ),
}


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
    or, with `power` > 1, gathered towards s = S; a list of (s, value)
    points is piecewise linear instead."""
    if isinstance(amount, list):
        return pw(s, amount)
    a0, a1 = amount
    return a0 + (a1 - a0) * np.clip(s / S, 0.0, 1.0) ** power



def pw(s, pts):
    """Piecewise linear in s through [(s, value), ...], flat beyond."""
    xs, ys = zip(*pts)
    return np.interp(s, xs, ys)


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
        self.twin = spec.get('twin', {})


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
        elif kind == 'ramp':
            # A band across the grain between the curves t = a(s) and
            # t = b(s) (piecewise linear through the given (s, t) points)
            # inside which the lines lean at amount / (b - a): D steps by
            # `amount` (also piecewise in s) over the band, with softplus
            # knees `k` wide at both edges. Stacked ramps give the panel's
            # sweep, its crease and the run below it.
            a = pw(s, term['a']); b = pw(s, term['b'])
            k = term.get('k', 2.0)
            if isinstance(k, list):
                k = pw(s, k)  # knee width varying across the grain
            u = (k * np.logaddexp(0.0, (tt - a) / k) - k * np.logaddexp(0.0, (tt - b) / k)) / np.maximum(b - a, 1e-6)
            phi = phi + pw(s, term['amount']) * u
        elif kind == 'shear':
            # One-sided: beyond ts, on the side t < ts, the lines lean
            # at `amount` (ds/dt); on the other side they run straight;
            # a soft knee `w` wide between: grain running off an edge.
            z = (term['ts'] - tt) / term['w']
            knee = term['w'] * np.logaddexp(0.0, z)
            shear = across(s, S, term['amount'], term.get('power', 1.0)) * knee
            if 'accel' in term:
                # The lean steepens further from ts: the lines curve
                # into arcs instead of running off the edge straight.
                shear = shear + across(s, S, term['accel']) * knee * knee / 2.0
            phi = phi + shear
        elif kind == 'lean':
            # One-sided, the other way: past ts (t > ts) the lines lean
            # at `amount` (ds/dt, negative leans them towards +s), after
            # a knee `w` wide -- the run into a streak, straight to it.
            # `accel` (ds/dt per px) steepens the lean further on, so the
            # lines flatten as they near the seam and pile into it.
            ts = term['ts']
            ts = across(s, S, ts) if isinstance(ts, (tuple, list)) else ts
            z = (tt - ts) / term['w']
            d = term['w'] * np.logaddexp(0.0, z)
            lean = across(s, S, term['amount'], term.get('power', 1.0)) * d
            if 'accel' in term:
                lean = lean + across(s, S, term['accel'], term.get('accel_power', 1.0)) * d * d / 2.0
            phi = phi + lean
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
    if sp['amp']:
        phi = phi + sp['amp'] * sp['cell'] / 3.0 * surf.space(phi, tm)
    return phi, tm


# ---------------------------------------------------------------------------
# Lines: iso-contours of each family's field, traced, then made straight.

def phi_at(surf, fam, s, t, twin):
    """phi of a family, for the main comb or the twin (whose figure terms
    sit `shift` design px further along the grain, so the two combs knee
    on alternate rows, as the reference's do)."""
    # A term marked `comb` ('main' or 'twin') bends that comb only, so the
    # other runs straight through it: two leaves crossing in one family.
    which = 'twin' if twin else 'main'
    terms = [term for term in fam['terms'] if term.get('comb', which) == which]
    if twin and surf.twin.get('shift'):
        terms = [shifted(term, surf.twin['shift']) for term in terms]
    return family_phi(surf, dict(fam, terms=terms), s, t)[0]


def shifted(term, dt):
    term = dict(term)
    if isinstance(term.get('ts'), list):
        term['ts'] = [(x, v + dt) for x, v in term['ts']]
    elif isinstance(term.get('ts'), tuple):
        term['ts'] = tuple(v + dt for v in term['ts'])
    elif 'ts' in term:
        term['ts'] = term['ts'] + dt
    if term['kind'] == 'ramp':
        term['a'] = [(x, y + dt) for x, y in term['a']]
        term['b'] = [(x, y + dt) for x, y in term['b']]
    if 'at' in term and term['kind'] == 'fold':
        term['at'] = (term['at'][0], term['at'][1] + dt)
        term['to'] = (term['to'][0], term['to'][1] + dt)
    return term


def domain(surf, fam):
    """The family's tracing box in (s, t): the surface plus a margin, cut
    to the family's window along the grain."""
    m = 2.0
    t0, t1 = fam.get('window', (None, None))[:2]
    ta = -m if t0 is None else max(t0, -m)
    tb = surf.T + m if t1 is None else min(t1, surf.T + m)
    # An optional window across the grain: a strip whose lines end at
    # its edges, a vertical jog where it meets its neighbour.
    u0, u1 = fam.get('swindow', (None, None))
    sa = -m if u0 is None else max(u0, -m)
    sb = surf.S + m if u1 is None else min(u1, surf.S + m)
    return (sa, sb, ta, tb)


def trace_family(surf, fam, twin):
    """Every iso-line of one comb of one family inside its box, as dense
    polylines in (s, t) design px. Levels are (k + offset) * pitch. Each
    open contour is found where it crosses the box's perimeter and traced
    inwards (predictor along the contour's tangent, two Newton steps back
    onto the level); a contour traced from both of its ends is kept once."""
    P = surf.spec['pitch']
    off = surf.twin['offset'] if twin else 0.0
    f = lambda s, t: phi_at(surf, fam, s, t, twin) / P - off
    s0, s1, t0, t1 = domain(surf, fam)
    # The perimeter, anticlockwise, finely sampled.
    d = PERIM
    sides = []
    for (a, b) in (((s0, t0), (s1, t0)), ((s1, t0), (s1, t1)), ((s1, t1), (s0, t1)), ((s0, t1), (s0, t0))):
        n = int(np.ceil(np.hypot(b[0] - a[0], b[1] - a[1]) / d))
        u = np.arange(n) / n
        sides.append(np.c_[a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u])
    per = np.vstack(sides + [sides[0][:1]])
    v = f(per[:, 0], per[:, 1])
    seeds, levels = [], []
    for i in np.nonzero(np.floor(v[:-1]) != np.floor(v[1:]))[0]:
        lo, hi = sorted((v[i], v[i + 1]))
        for L in range(int(np.floor(lo)) + 1, int(np.floor(hi)) + 1):
            w = (L - v[i]) / (v[i + 1] - v[i])
            seeds.append(per[i] + (per[i + 1] - per[i]) * w)
            levels.append(L)
    if not seeds:
        return []
    X = np.array(seeds)
    lev = np.array(levels, float)
    n = len(X)
    e = 0.01

    def grad(Y):
        q = np.vstack([Y + [e, 0], Y - [e, 0], Y + [0, e], Y - [0, e]])
        r = f(q[:, 0], q[:, 1]).reshape(4, -1)
        return np.c_[(r[0] - r[1]) / (2 * e), (r[2] - r[3]) / (2 * e)]

    g = grad(X)
    tan = np.c_[-g[:, 1], g[:, 0]]
    tan /= np.maximum(np.hypot(tan[:, 0], tan[:, 1]), 1e-12)[:, None]
    cs, ct = (s0 + s1) / 2, (t0 + t1) / 2
    inward = (cs - X[:, 0]) * tan[:, 0] + (ct - X[:, 1]) * tan[:, 1]
    # pick the sign that enters the box: test a small step
    Yp = X + tan * 0.5
    inside = (Yp[:, 0] > s0) & (Yp[:, 0] < s1) & (Yp[:, 1] > t0) & (Yp[:, 1] < t1)
    tan = np.where(inside[:, None], tan, -tan)
    paths = [[X[i].copy()] for i in range(n)]
    alive = np.ones(n, bool)
    h = STEP
    steps = int(4 * (s1 - s0 + t1 - t0) / h)
    Y = X.copy()
    for it in range(steps):
        if it % 40 == 39:
            # A line that is going nowhere (a closed loop, or caught at a
            # point where the field's gradient vanishes) is dropped there.
            back = np.array([p[-40] if len(p) >= 40 else p[0] for p in paths])
            alive &= np.hypot(*(Y - back).T) > 8 * h
        idx = np.nonzero(alive)[0]
        if idx.size == 0:
            break
        Yi = Y[idx]
        g = grad(Yi)
        tn = np.c_[-g[:, 1], g[:, 0]]
        tn /= np.maximum(np.hypot(tn[:, 0], tn[:, 1]), 1e-12)[:, None]
        sg = np.sign((tn * tan[idx]).sum(1))
        sg[sg == 0] = 1
        tn *= sg[:, None]
        hh = np.full(len(idx), h)
        Z = np.empty_like(Yi)
        todo = np.arange(len(idx))
        for _r in range(4):
            Zt = Yi[todo] + hh[todo, None] * tn[todo]
            gz = grad(Zt)
            for _k in range(2):
                r = f(Zt[:, 0], Zt[:, 1]) - lev[idx[todo]]
                Zt -= (r / np.maximum((gz * gz).sum(1), 1e-12))[:, None] * gz
            Z[todo] = Zt
            dz = Zt - Yi[todo]
            cosv = (dz * tan[idx[todo]]).sum(1) / np.maximum(np.hypot(*dz.T), 1e-12)
            bad = cosv < 0.7
            if _r == 3 or not bad.any():
                break
            if _r == 2:
                # Still turning back: a cusp (a seam whose profile has a
                # corner). Step straight on through it instead; Newton
                # lands on the far arm.
                tn[todo[bad]] = tan[idx[todo[bad]]]
            # A turn sharper than the step: retry those with a quarter step.
            todo = todo[bad]
            hh[todo] /= 4
        tan[idx] = (Z - Yi) / np.maximum(np.hypot(*(Z - Yi).T), 1e-12)[:, None]
        Y[idx] = Z
        out = (Z[:, 0] < s0) | (Z[:, 0] > s1) | (Z[:, 1] < t0) | (Z[:, 1] > t1)
        for j, i in enumerate(idx):
            paths[i].append(Z[j].copy())
        alive[idx[out]] = False
    # Each contour once: drop the trace whose far end is an earlier seed.
    keep = []
    for i in range(n):
        end = paths[i][-1]
        same = np.nonzero(lev == lev[i])[0]
        dd = np.hypot(X[same, 0] - end[0], X[same, 1] - end[1])
        j = same[np.argmin(dd)]
        if dd.min() < 4 * h and j < i:
            continue
        Q = np.array(paths[i])
        if len(Q) > 2 and np.hypot(*(Q[-1] - Q[0])) < 4 * h:
            # Out and back: the tracer stepped through a cusp and came
            # back along the same arm. Keep the way out only, to its
            # farthest point, so the arm is not stroked twice.
            Q = Q[:int(np.argmax(np.hypot(*(Q - Q[0]).T))) + 1]
        keep.append((Q, int(lev[i])))
    return keep


def knees(Q, step, hyst, tol):
    """Straighten a dense line: its direction is held in bins `step` wide
    (degrees, from the grain), with `hyst` of hysteresis, and a vertex is
    put where the bin changes -- a knee of about `step` -- so the knee rows
    of neighbouring lines are the iso-lines of the field's direction, the
    folds. Each run between knees is a chord, split further (Douglas-
    Peucker) only where it strays more than `tol` design px from the line."""
    dQ = np.diff(Q, axis=0)
    ang = np.degrees(np.unwrap(np.arctan2(dQ[:, 0], dQ[:, 1])))
    b = np.round(ang[0] / step)
    cut = [0]
    for i in range(1, len(ang)):
        if abs(ang[i] - b * step) > step / 2 + hyst:
            b = np.round(ang[i] / step)
            cut.append(i)
    cut.append(len(Q) - 1)
    out = []
    for a, c in zip(cut[:-1], cut[1:]):
        out.extend(dp(Q[a:c + 1], tol)[:-1])
    out.append(Q[-1])
    return np.array(out)


def dp(Q, tol):
    if len(Q) < 3:
        return [Q[0], Q[-1]]
    a, b = Q[0], Q[-1]
    d = b - a
    L = np.hypot(*d)
    if L < 1e-9:
        dist = np.hypot(*(Q - a).T)
    else:
        dist = np.abs((Q[:, 0] - a[0]) * d[1] - (Q[:, 1] - a[1]) * d[0]) / L
    i = int(np.argmax(dist))
    if dist[i] <= tol:
        return [a, b]
    return dp(Q[:i + 1], tol)[:-1] + dp(Q[i:], tol)


def fillet(V, r):
    """Round each corner of polyline V with a circular arc of radius `r`
    (clamped to half the adjacent segments)."""
    pts = [V[0]]
    for j in range(1, len(V) - 1):
        a, b = V[j - 1] - V[j], V[j + 1] - V[j]
        la, lb = np.hypot(*a), np.hypot(*b)
        if la < 1e-9 or lb < 1e-9:
            continue
        a, b = a / la, b / lb
        ang = np.arccos(np.clip(a @ b, -1, 1))
        if r <= 0 or ang > np.pi - 0.02:
            pts.append(V[j])
            continue
        tt = min(r / np.tan(ang / 2), 0.5 * la, 0.5 * lb)
        rr = tt * np.tan(ang / 2)
        T1, T2 = V[j] + a * tt, V[j] + b * tt
        bis = (a + b) / np.hypot(*(a + b))
        c = V[j] + bis * (rr / np.sin(ang / 2))
        a0, a1 = np.arctan2(*(T1 - c)[::-1]), np.arctan2(*(T2 - c)[::-1])
        dd = (a1 - a0 + np.pi) % (2 * np.pi) - np.pi
        m = max(2, int(abs(dd) * rr / 0.5) + 1)
        th = a0 + np.linspace(0, dd, m)
        pts.extend(c + rr * np.c_[np.cos(th), np.sin(th)])
    pts.append(V[-1])
    return np.array(pts)


def trim(Q, box):
    """End a line exactly on its box: the tracer's last step lands up to
    STEP beyond it, which at a strip cut leaves the ends of the lines on
    either side overlapping in a dotted seam."""
    s0, s1, t0, t1 = box
    a, b = Q[-2], Q[-1]
    d = b - a
    f = 1.0
    for i, lo, hi in ((0, s0, s1), (1, t0, t1)):
        if b[i] > hi and d[i] > 0:
            f = min(f, (hi - a[i]) / d[i])
        if b[i] < lo and d[i] < 0:
            f = min(f, (lo - a[i]) / d[i])
    Q = Q.copy()
    Q[-1] = a + d * max(f, 0.0)
    return Q


def lines(spec):
    """All of a surface's drawn lines, as polylines in texels (x, y)."""
    surf = Surface(spec)
    out = []
    for fam in spec['families']:
        q = dict(QUANT, **spec.get('quant', {}), **fam.get('quant', {}))
        fade = fam.get('fade')
        if fade:
            box = domain(surf, fam)
            fade = (box[2], box[3], fade[0], fade[1], spec['axis'])
        for twin in ((False, True) if surf.twin else (False,)):
            for Q, _ in trace_family(surf, fam, twin):
                if len(Q) < 3:
                    continue
                if fam.get('trim'):
                    Q = trim(Q, domain(surf, fam))
                V = knees(Q, q['step'], q['hyst'], q['tol'])
                V = V * R
                if spec['axis'] == 'h':
                    V = V[:, ::-1]
                out.append((fillet(V, q['radius']), fam.get('ink', 1.0), fade))
    return out


# ---------------------------------------------------------------------------
# Rendering, as measured (veneer-construction.md, "Line, ground and grain").

def faded(fade, X, Y):
    """A family's opacity near its window ends: `fade` = (t0, t1, a, b,
    axis); the lines fade in over `a` design px after t0 and out over
    `b` before t1 (smoothstep), so a crossing comb fades into the other
    leaf instead of stopping on a cut row."""
    t0, t1, a, b, axis = fade
    t = (Y if axis == 'v' else X) / R
    f = np.ones_like(t)
    if a:
        u = np.clip((t - t0) / a, 0.0, 1.0); f = f * u * u * (3 - 2 * u)
    if b:
        u = np.clip((t1 - t) / b, 0.0, 1.0); f = f * u * u * (3 - 2 * u)
    return f


def stroke(taus, P, alpha, width=1.0, pad=2, fade=None):
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
        d = b - a
        L2 = d @ d + 1e-12
        t = np.clip(((X - a[0]) * d[0] + (Y - a[1]) * d[1]) / L2, 0, 1)
        sub = D[by0 - y0:by1 - y0, bx0 - x0:bx1 - x0]
        np.minimum(sub, np.hypot(X - a[0] - t * d[0], Y - a[1] - t * d[1]), out=sub)
    cov = np.clip(np.minimum(D + width / 2, 0.5) - np.maximum(D - width / 2, -0.5), 0, None)
    if fade is not None:
        X, Y = np.meshgrid(np.arange(x0, x1) + 0.5, np.arange(y0, y1) + 0.5)
        cov = cov * faded(fade, X, Y)
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
        out[c] = n / 11.0 * std[c]
    return out


def render_rgb(spec, grain_on=True):
    """The surface as float RGB (H x W x 3), before palette quantisation."""
    W, H = spec['size']
    tw, th = int(np.ceil(W * R)), int(np.ceil(H * R))
    look = spec['look']
    ground, single = np.array(look['ground'], float), np.array(look['single'], float)
    store = np.array(SURFACES['store-card']['look']['ground'], float)
    core = np.minimum(np.array(CORE) * ground / store, single - 1)
    alpha = (ground - single) / (ground - core)
    taus = np.zeros((3, th, tw))
    for P, ink, fade in lines(spec):
        stroke(taus, P, alpha * ink, width=look.get('width', 1.0), fade=fade)
    rgb = core[:, None, None] + (ground - core)[:, None, None] * np.exp(-taus)
    if grain_on:
        rgb = rgb + grain((th, tw), look['grain'], spec['seed'] + 7)
    return np.clip(rgb, 0, 255).transpose(1, 2, 0)


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




def render(spec):
    return palette_quantise(render_rgb(spec))


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
        idx, pal = render(spec)
        data = png_indexed(idx, pal)
        (out / f'{name}.png').write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {name}.png  {idx.shape[1]}x{idx.shape[0]}  {len(data)} bytes',
              file=sys.stderr)


if __name__ == '__main__':
    main()
