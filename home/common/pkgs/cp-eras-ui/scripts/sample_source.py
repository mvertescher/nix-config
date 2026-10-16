#!/usr/bin/env python3
"""Project a source image into a scene viewport at one top-left uniform scale.

Unknown viewport space is transparent. Edge pixels may have partial alpha;
Lanczos filter support near the boundary repeats source edge colours. Alpha
records geometric coverage, not filter provenance: even fully covered pixels
near an edge can use synthetic filter support. This is a sampling aid, not a
pixel-fidelity score.

Usage: sample_source.py ORIGINAL --size WxH --out SAMPLE.png [--metadata DATA.json]
"""

import argparse
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import re

from PIL import Image


def parse_size(value):
    match = re.fullmatch(r"([1-9][0-9]*)x([1-9][0-9]*)", value)
    if not match:
        raise argparse.ArgumentTypeError("size must be positive WxH, e.g. 1537x947")
    return tuple(map(int, match.groups()))


def _coverage_alpha(size, content):
    """Area coverage, with every positive partial fraction encoded as 1..254."""
    def partial(fraction):
        return min(254, max(1, round(255 * fraction)))

    width, height = size
    cw, ch = content
    full_w, full_h = min(width, cw.numerator // cw.denominator), min(height, ch.numerator // ch.denominator)
    alpha = Image.new("L", size, 0)
    if full_w and full_h:
        alpha.paste(255, (0, 0, full_w, full_h))
    if full_w < width and full_w < cw:
        ax = partial(cw - full_w)
        if full_h:
            alpha.paste(ax, (full_w, 0, full_w + 1, full_h))
    if full_h < height and full_h < ch:
        ay = partial(ch - full_h)
        if full_w:
            alpha.paste(ay, (0, full_h, full_w, full_h + 1))
    if full_w < width and full_h < height and full_w < cw and full_h < ch:
        alpha.putpixel((full_w, full_h), partial((cw - full_w) * (ch - full_h)))
    return alpha


def project_source(source, size):
    """Return (RGBA sample, geometry) without changing the input image.

    A float source box avoids rounding the content height or width before
    Lanczos prefiltering. Only the content tile is filtered. Its padded
    canvas repeats edge colours; exact coverage alpha marks all space outside
    the original image as unknown.
    """
    width, height = size
    if width <= 0 or height <= 0:
        raise ValueError("output dimensions must be positive")
    rgb = source.convert("RGB")
    sw, sh = rgb.size
    if sw <= 0 or sh <= 0:
        raise ValueError("source dimensions must be positive")
    if width * sh <= height * sw:
        scale = Fraction(width, sw)
    else:
        scale = Fraction(height, sh)
    content = (sw * scale, sh * scale)
    source_box = (0.0, 0.0, float(Fraction(width, 1) / scale), float(Fraction(height, 1) / scale))
    tile_size = (min(width, math.ceil(content[0])), min(height, math.ceil(content[1])))
    resample_box = (0.0, 0.0, float(Fraction(tile_size[0], 1) / scale),
                    float(Fraction(tile_size[1], 1) / scale))

    if content == (width, height):
        # Preserve the historical 1600x900 whole-RGB Lanczos sample exactly.
        tile = rgb.resize(size, Image.Resampling.LANCZOS)
    else:
        padded_w = max(sw, math.ceil(tile_size[0] / scale))
        padded_h = max(sh, math.ceil(tile_size[1] / scale))
        padded = Image.new("RGB", (padded_w, padded_h))
        padded.paste(rgb, (0, 0))
        if padded_w > sw:
            edge = rgb.crop((sw - 1, 0, sw, sh))
            padded.paste(edge.resize((padded_w - sw, sh)), (sw, 0))
        if padded_h > sh:
            edge = padded.crop((0, sh - 1, padded_w, sh))
            padded.paste(edge.resize((padded_w, padded_h - sh)), (0, sh))
        tile = padded.resize(tile_size, Image.Resampling.LANCZOS, box=resample_box)

    tile.putalpha(_coverage_alpha(tile_size, content))
    sampled = Image.new("RGBA", size, (0, 0, 0, 0))
    sampled.paste(tile, (0, 0))
    geometry = {
        "method": "uniform-top-left-lanczos-content-tile-float-box-edge-extended-rgb-coverage-alpha",
        "scale": float(scale),
        "source_box": list(source_box),
        "resample_box": list(resample_box),
        "resample_size": list(tile_size),
        "content_bounds": [0.0, 0.0, float(content[0]), float(content[1])],
    }
    return sampled, geometry


def _same_file(a, b):
    return a.resolve() == b.resolve() or (a.exists() and b.exists() and a.samefile(b))


def sample_source(source_path, size, out_path, metadata_path=None):
    """Write a PNG sample and optional JSON; return metadata with file hashes."""
    source_path, out_path = Path(source_path), Path(out_path)
    metadata_path = Path(metadata_path) if metadata_path is not None else None
    if _same_file(source_path, out_path) or (metadata_path is not None and
        (_same_file(source_path, metadata_path) or _same_file(out_path, metadata_path))):
        raise ValueError("source, output, and metadata paths must be distinct")
    with Image.open(source_path) as source:
        sampled, geometry = project_source(source, size)
        source_size = list(source.size)
    source_bytes = source_path.read_bytes()
    out_path.parent.mkdir(parents=True, exist_ok=True)
    sampled.save(out_path, format="PNG")
    output_bytes = out_path.read_bytes()
    metadata = {
        **geometry,
        "source": {"path": str(source_path), "size": source_size,
                   "sha256": hashlib.sha256(source_bytes).hexdigest(), "bytes": len(source_bytes)},
        "output": {"path": str(out_path), "size": list(sampled.size),
                   "sha256": hashlib.sha256(output_bytes).hexdigest(), "bytes": len(output_bytes)},
        "alpha_semantics": "255 = full geometric source coverage; 1..254 = positive partial coverage quantized to 8-bit alpha; 0 = unknown viewport padding; boundary RGB may use synthetic edge extension",
    }
    if metadata_path is not None:
        metadata_path.parent.mkdir(parents=True, exist_ok=True)
        metadata_path.write_text(json.dumps(metadata, indent=2) + "\n")
    return metadata


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("original", type=Path)
    parser.add_argument("--size", required=True, type=parse_size, metavar="WxH")
    parser.add_argument("--out", required=True, type=Path, metavar="PNG")
    parser.add_argument("--metadata", type=Path, metavar="JSON")
    args = parser.parse_args(argv)
    try:
        sample_source(args.original, args.size, args.out, args.metadata)
    except (OSError, ValueError) as exc:
        parser.error(str(exc))


if __name__ == "__main__":
    main()
