"""Coordinate and evidence-boundary checks for sample_source.py."""

import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

from PIL import Image

from sample_source import project_source, sample_source


SCRIPT = Path(__file__).with_name("sample_source.py")


def patterned_image(size=(96, 54)):
    image = Image.new("RGB", size)
    pixels = image.load()
    for y in range(size[1]):
        for x in range(size[0]):
            pixels[x, y] = (13 * x % 256, 31 * y % 256, (7 * x + 11 * y) % 256)
    return image


class SourceSamplingTests(unittest.TestCase):
    def test_whole_frame_matches_existing_rgb_lanczos_pixels(self):
        original = patterned_image()
        for size in ((160, 90), (48, 27)):
            with self.subTest(size=size):
                sample, metadata = project_source(original, size)
                expected = original.convert("RGB").resize(size, Image.Resampling.LANCZOS)
                self.assertEqual(sample.convert("RGB").tobytes(), expected.tobytes())
                self.assertEqual(sample.getchannel("A").getextrema(), (255, 255))
                self.assertEqual(metadata["source_box"], [0.0, 0.0, 96.0, 54.0])

    def test_same_width_taller_viewport_keeps_interior_coordinates(self):
        original = patterned_image()
        near, near_geometry = project_source(original, (137, 78))
        tall, tall_geometry = project_source(original, (137, 91))
        # Both sample the same original x/y landmarks despite different
        # viewport heights. A viewport stretch or rounded content height fails.
        self.assertEqual(near.crop((0, 0, 137, 75)).tobytes(),
                         tall.crop((0, 0, 137, 75)).tobytes())
        self.assertEqual(near_geometry["scale"], tall_geometry["scale"])
        self.assertEqual(tall_geometry["content_bounds"], [0.0, 0.0, 137.0, 77.0625])
        self.assertAlmostEqual(tall_geometry["source_box"][3], 91 * 96 / 137)

    def test_fractional_content_is_not_resized_to_rounded_height(self):
        original = patterned_image()
        sample, geometry = project_source(original, (136, 91))
        # Width scale places the source bottom at 76.5 px. Resizing source
        # directly to either integer height subtly moves landmarks throughout.
        interior = sample.crop((0, 0, 136, 70)).convert("RGB").tobytes()
        for rounded_height in (76, 77):
            rounded = original.resize((136, rounded_height), Image.Resampling.LANCZOS)
            self.assertNotEqual(interior, rounded.crop((0, 0, 136, 70)).tobytes())
        self.assertEqual(geometry["content_bounds"][3], 76.5)
        self.assertEqual(sample.getpixel((40, 76))[3], 128)

    def test_short_wide_viewport_uses_height_scale_and_unknown_right_side(self):
        original = patterned_image()
        wide, geometry = project_source(original, (160, 70))
        fitted, _ = project_source(original, (125, 70))
        self.assertEqual(wide.crop((0, 0, 120, 70)).tobytes(),
                         fitted.crop((0, 0, 120, 70)).tobytes())
        self.assertAlmostEqual(geometry["scale"], 70 / 54)
        self.assertAlmostEqual(geometry["content_bounds"][2], 96 * 70 / 54)
        self.assertEqual(wide.getpixel((159, 35))[3], 0)

    def test_fractional_edge_and_padding_are_marked_explicitly(self):
        sample, _ = project_source(patterned_image(), (137, 91))
        self.assertEqual(sample.getpixel((40, 76))[3], 255)
        self.assertEqual(sample.getpixel((40, 77))[3], 16)  # 1/16 pixel coverage
        self.assertEqual(sample.getpixel((40, 78))[3], 0)
        self.assertEqual(sample.getpixel((40, 90))[3], 0)

    def test_partial_coverage_cannot_quantize_to_opaque_or_unknown(self):
        almost_full, geometry = project_source(Image.new("RGB", (1024, 1023), "red"), (1, 2))
        self.assertEqual(geometry["content_bounds"][3], 1023 / 1024)
        self.assertEqual(almost_full.getpixel((0, 0))[3], 254)
        self.assertEqual(almost_full.getpixel((0, 1))[3], 0)

        tiny, geometry = project_source(Image.new("RGB", (1024, 1), "blue"), (1, 2))
        self.assertEqual(geometry["content_bounds"][3], 1 / 1024)
        self.assertEqual(tiny.getpixel((0, 0))[3], 1)
        self.assertEqual(tiny.getpixel((0, 1))[3], 0)

    def test_extreme_viewports_filter_only_small_content_tiles(self):
        original = patterned_image()
        real_new = Image.new
        for viewport, expected_tile in (((1, 10000), (1, 1)),
                                        ((10000, 1), (2, 1))):
            with self.subTest(viewport=viewport), patch("sample_source.Image.new", wraps=real_new) as new:
                sample, geometry = project_source(original, viewport)
            rgb_allocations = [call.args[1] for call in new.call_args_list
                               if call.args[0] == "RGB"]
            self.assertTrue(rgb_allocations)
            self.assertTrue(all(w <= 192 and h <= 108 for w, h in rgb_allocations))
            self.assertEqual(geometry["resample_size"], list(expected_tile))
            self.assertEqual(sample.size, viewport)
            self.assertEqual(sample.getpixel((viewport[0] - 1, viewport[1] - 1))[3], 0)

    def test_cli_metadata_hashes_and_source_unchanged(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            source = root / "source.png"
            output = root / "sample.png"
            report = root / "metadata.json"
            patterned_image().save(source)
            before = source.read_bytes()
            result = subprocess.run([sys.executable, str(SCRIPT), str(source),
                                     "--size", "137x91", "--out", str(output),
                                     "--metadata", str(report)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            data = json.loads(report.read_text())
            self.assertEqual(source.read_bytes(), before)
            with Image.open(output) as image:
                self.assertEqual((image.mode, image.size), ("RGBA", (137, 91)))
            self.assertEqual(data["source"]["size"], [96, 54])
            self.assertEqual(data["output"]["size"], [137, 91])
            self.assertEqual(data["source"]["sha256"], hashlib.sha256(before).hexdigest())
            self.assertEqual(data["output"]["sha256"], hashlib.sha256(output.read_bytes()).hexdigest())
            self.assertEqual(data["source"]["bytes"], len(before))
            self.assertEqual(data["output"]["bytes"], output.stat().st_size)

    def test_invalid_sizes_and_source_overwrite_fail_cleanly(self):
        with tempfile.TemporaryDirectory() as temp:
            source = Path(temp) / "source.png"
            patterned_image().save(source)
            before = source.read_bytes()
            for bad in ("0x90", "160x0", "160", "160X90", "-1x90", "foo"):
                result = subprocess.run([sys.executable, str(SCRIPT), str(source),
                                         f"--size={bad}", "--out", str(Path(temp) / "out.png")],
                                        capture_output=True, text=True)
                self.assertEqual(result.returncode, 2, (bad, result.stderr))
                self.assertIn("size must be positive WxH", result.stderr)
            result = subprocess.run([sys.executable, str(SCRIPT), str(source),
                                     "--size", "160x90", "--out", str(source)],
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("paths must be distinct", result.stderr)
            self.assertEqual(source.read_bytes(), before)
            with self.assertRaisesRegex(ValueError, "positive"):
                project_source(patterned_image(), (0, 90))
            with self.assertRaisesRegex(ValueError, "distinct"):
                sample_source(source, (160, 90), Path(temp) / "out.png", source)


if __name__ == "__main__":
    unittest.main()
