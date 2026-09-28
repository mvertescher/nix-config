#!/usr/bin/env python3
"""Focused segmentation regressions; run with Python + numpy/scipy/Pillow."""
import unittest
import json
import subprocess
import sys
import tempfile
from pathlib import Path

import numpy as np
from PIL import Image
from scipy import ndimage

from extract_spec import (assign_palette, palette_spec, reference_palette,
                          repair_striped_regions, extract, collapse_shared_outlines,
                          fit_shape, t_chamfer, t_diamond, split_blob, components)


class ShapeSupportTests(unittest.TestCase):
    def test_whole_panel_edges_override_texture_peaks_but_not_overlapping_panels(self):
        panel = np.zeros((200, 300), bool)
        panel[30:90, 20:240] = True
        panel[75:90, 126:132] = False
        # This deeper, narrow interruption has persistent distance peaks,
        # but all four external panel edges remain independently measured.
        self.assertEqual(len(split_blob(panel, ndimage.distance_transform_edt(panel))[0]), 2)
        shapes, _ = components(panel, panel, panel.shape, "ink")
        self.assertEqual([(s["class"], s["bbox"]) for s in shapes],
                         [("rect", [20, 30, 220, 60])])

        overlap = np.zeros_like(panel)
        overlap[20:100, 20:160] = True
        overlap[70:150, 100:240] = True
        shapes, _ = components(overlap, overlap, overlap.shape, "ink")
        self.assertEqual(len(shapes), 2)
        self.assertTrue(all(s["class"] == "rect" for s in shapes))

    def test_raster_notch_does_not_split_one_bar_but_overlapping_lobes_do(self):
        bar = np.zeros((120, 280), bool)
        bar[30:90, 20:240] = True
        # A three-pixel notch creates two local distance maxima without
        # making two bodies. It models a caption/antialiasing interruption.
        bar[87:90, 128:132] = False
        cells, _ = split_blob(bar, ndimage.distance_transform_edt(bar))
        self.assertEqual(len(cells), 1)
        np.testing.assert_array_equal(cells[0], bar)

        overlap = (t_diamond((180, 260), (80, 90, 55)) |
                   t_diamond((180, 260), (155, 90, 55)))
        cells, _ = split_blob(overlap, ndimage.distance_transform_edt(overlap))
        self.assertEqual(len(cells), 2)
        self.assertFalse(np.any(cells[0] & cells[1]))
        np.testing.assert_array_equal(cells[0] | cells[1], overlap)

    def test_rectangular_material_with_thin_extensions_is_not_a_diamond(self):
        material = np.zeros((500, 500), bool)
        material[200:360, 160:340] = True
        material[70:410, 248:252] = True
        # A maximal inscribed circle plus long thin extensions used to
        # infer a 238px-wide diamond around this 180px-wide rectangle.
        shape = fit_shape(material, material,
                          ndimage.distance_transform_edt(material),
                          material.shape, material)
        self.assertNotEqual(shape["class"], "diamond")

    def test_occluded_diamond_tips_keep_supported_diagonal_sides(self):
        for occlusion in ("left", "left-top", "left-right"):
            with self.subTest(occlusion=occlusion):
                visible = t_diamond((260, 260), (130, 130, 95))
                visible[:, :55] = False
                if occlusion == "left-top":
                    visible[:55] = False
                if occlusion == "left-right":
                    visible[:, 205:] = False
                shape = fit_shape(visible, visible,
                                  ndimage.distance_transform_edt(visible),
                                  visible.shape, visible)
                self.assertEqual(shape["class"], "diamond")
                self.assertEqual((shape["params"]["cx"], shape["params"]["cy"]),
                                 (130, 130))
                self.assertLessEqual(abs(shape["params"]["half_diagonal"] - 95), 2)

    def test_extracted_overlapping_diamonds_remain_required_by_gate(self):
        self.assert_overlapping_widgets_required("diamonds")

    def test_extracted_overlapping_rectangles_remain_required_by_gate(self):
        self.assert_overlapping_widgets_required("rectangles")

    def assert_overlapping_widgets_required(self, kind):
        w, h = 500, 200
        palette = {"canvas": [w, h], "palette": [
            {"index": 0, "rgb": [10, 10, 10], "role": "ground"},
            {"index": 1, "rgb": [40, 180, 170], "role": "ink"},
        ]}
        checker = Path(__file__).with_name("spec_diff.py")
        with tempfile.TemporaryDirectory() as temp:
            paths = {}
            for name in ("present", "missing", "moved"):
                if kind == "diamonds":
                    mask = t_diamond((h, w), (80, 100, 55))
                    if name != "missing":
                        mask |= t_diamond((h, w), (350 if name == "moved" else 155,
                                                   100, 55))
                    shape_class = "diamond"
                else:
                    mask = np.zeros((h, w), bool)
                    mask[20:100, 20:160] = True
                    if name != "missing":
                        x = 340 if name == "moved" else 100
                        mask[70:150, x:x + 140] = True
                    shape_class = "rect"
                pixels = np.full((h, w, 3), 10, dtype=np.uint8)
                pixels[mask] = [40, 180, 170]
                image = Path(temp) / (name + ".png")
                Image.fromarray(pixels).save(image)
                spec, _, _ = extract(image, (w, h), 2, palette)
                self.assertEqual([s["class"] for s in spec["shapes"]],
                                 [shape_class] * (1 if name == "missing" else 2))
                paths[name] = Path(temp) / (name + ".json")
                paths[name].write_text(json.dumps(spec))
            for name, passes in (("present", True), ("missing", False), ("moved", False)):
                with self.subTest(name=name):
                    result = subprocess.run([sys.executable, str(checker),
                                             "--match-iou", "0.65",
                                             str(paths["present"]), str(paths[name])],
                                            capture_output=True, text=True)
                    self.assertEqual(result.returncode == 0, passes, result.stdout)

    def test_large_chamfer_requires_a_missing_straight_corner(self):
        canvas = (120, 160)
        chamfer = t_chamfer(canvas, (20, 10, 120, 110, 30, 3))
        perimeter = np.zeros(canvas, bool)
        perimeter[10:110, 20] = perimeter[10:110, 119] = True
        perimeter[10, 20:120] = perimeter[109, 20:120] = True
        dist = ndimage.distance_transform_edt(chamfer)
        supported = fit_shape(chamfer, chamfer, dist, canvas, chamfer)
        contradicted = fit_shape(chamfer, chamfer, dist, canvas, perimeter)
        self.assertEqual(supported["class"], "chamfer")
        self.assertNotEqual(contradicted["class"], "chamfer")

    def test_shared_outline_fragment_only_drops_without_own_top_edge(self):
        def panel(box, top):
            return {"class": "rect", "bbox": box, "_top_edge_support": top}
        outer = panel([10, 10, 200, 200], 0.95)
        fragment = panel([11, 50, 198, 159], 0.0)
        child = panel([10, 50, 200, 160], 0.9)
        self.assertEqual(collapse_shared_outlines([outer, fragment]), [outer])
        self.assertEqual(collapse_shared_outlines([outer, child]), [outer, child])

    def test_missing_moved_and_nested_widgets_still_fail_shape_gate(self):
        def shape(kind, box, number):
            return {"class": kind, "bbox": box, "id": str(number)}
        parent = shape("rect", [10, 10, 200, 200], 1)
        child = shape("rect", [10, 50, 200, 160], 2)
        parent["_top_edge_support"] = child["_top_edge_support"] = 0.9
        supported = collapse_shared_outlines([parent, child])
        self.assertEqual(len(supported), 2)
        source = {"canvas": [500, 260], "palette": [], "shapes": supported}
        checker = Path(__file__).with_name("spec_diff.py")
        with tempfile.TemporaryDirectory() as temp:
            src = Path(temp) / "source.json"
            src.write_text(json.dumps(source))
            for name, shapes, passes in (
                ("intact", [parent, child], True),
                ("missing", [parent], False),
                ("moved", [parent, shape("rect", [260, 50, 200, 160], 2)], False),
                ("chamfer-equivalent", [shape("chamfer", parent["bbox"], 1), child], True),
            ):
                with self.subTest(name=name):
                    cand = Path(temp) / (name + ".json")
                    cand.write_text(json.dumps({**source, "shapes": shapes}))
                    result = subprocess.run([sys.executable, str(checker),
                                             str(src), str(cand)],
                                            capture_output=True, text=True)
                    self.assertEqual(result.returncode == 0, passes, result.stdout)

    def test_extracted_docked_child_survives_and_is_required(self):
        w, h = 500, 260
        palette = {"canvas": [w, h], "palette": [
            {"index": i, "rgb": [10 + i] * 3, "role": "ground"}
            for i in range(6)
        ] + [
            {"index": 6, "rgb": [40, 180, 170], "role": "ink"},
            {"index": 7, "rgb": [180, 70, 130], "role": "ink"},
        ]}

        def picture(child="present"):
            pixels = np.full((h, w, 3), 10, dtype=np.uint8)
            def outline(x0, y0, x1, y1, colour):
                pixels[y0:y1, x0:x0 + 2] = colour
                pixels[y0:y1, x1 - 2:x1] = colour
                pixels[y0:y0 + 2, x0:x1] = colour
                pixels[y1 - 2:y1, x0:x1] = colour
            outline(20, 10, 220, 190, [40, 180, 170])
            if child != "missing":
                x0 = 270 if child == "moved" else 21
                outline(x0, 55, x0 + 198, 189, [180, 70, 130])
            return pixels

        checker = Path(__file__).with_name("spec_diff.py")
        with tempfile.TemporaryDirectory() as temp:
            paths = {}
            for name in ("present", "missing", "moved"):
                image = Path(temp) / (name + ".png")
                Image.fromarray(picture(name)).save(image)
                spec, _, _ = extract(image, (w, h), 8, palette)
                paths[name] = Path(temp) / (name + ".json")
                paths[name].write_text(json.dumps(spec))
                self.assertEqual(len(spec["shapes"]), 1 if name == "missing" else 2)
            for name, passes in (("present", True), ("missing", False), ("moved", False)):
                with self.subTest(name=name):
                    result = subprocess.run([sys.executable, str(checker),
                                             str(paths["present"]), str(paths[name])],
                                            capture_output=True, text=True)
                    self.assertEqual(result.returncode == 0, passes, result.stdout)
                    if name == "missing":
                        self.assertIn("58% of source shape area", result.stdout)


class ReferencePaletteTests(unittest.TestCase):
    def test_fixed_palette_keeps_source_roles_and_measures_candidate_colour(self):
        source = {"palette": [
            {"index": 0, "rgb": [10, 10, 10], "role": "ground"},
            {"index": 1, "rgb": [30, 160, 150], "role": "ink"},
        ]}
        centres, roles = reference_palette(source, 2)
        image = np.full((10, 10, 3), [10, 10, 10], dtype=np.float32)
        image[0, :5] = [35, 165, 155]  # ink touching border stays ink
        labels, residual = assign_palette(image, centres)
        entries = palette_spec(labels, centres, 10, 10, roles, image)
        ink = next(e for e in entries if e["index"] == 1)
        self.assertEqual(ink["role"], "ink")
        self.assertEqual(ink["rgb"], [35, 165, 155])
        self.assertEqual(residual["rgb_distance_at_least_110"], 0)

    def test_foreign_colour_does_not_poison_a_source_ink_bin(self):
        centres = np.array([[10, 10, 10], [30, 160, 150]], dtype=np.float32)
        image = np.full((10, 10, 3), [10, 10, 10], dtype=np.float32)
        image[:5] = [35, 165, 155]
        image[5:7] = [230, 230, 20]
        labels, residual = assign_palette(image, centres)
        self.assertTrue(np.all(labels[5:7] == -1))
        self.assertEqual(residual["rgb_distance_at_least_110"], 0.2)
        entries = palette_spec(labels, centres, 10, 10, ["ground", "ink"], image)
        ink = next(e for e in entries if e["index"] == 1)
        self.assertEqual(ink["rgb"], [35, 165, 155])

    def test_empty_reference_bin_has_no_invented_candidate_colour(self):
        centres = np.array([[10, 10, 10], [30, 160, 150]], dtype=np.float32)
        image = np.full((10, 10, 3), [10, 10, 10], dtype=np.float32)
        labels, _ = assign_palette(image, centres)
        entries = palette_spec(labels, centres, 10, 10, ["ground", "ink"], image)
        ink = next(e for e in entries if e["index"] == 1)
        self.assertEqual(ink["coverage"], 0)
        self.assertIsNone(ink["rgb"])

    def test_paired_ink_gate_keeps_foreground_and_rejects_bad_edits(self):
        w, h = 160, 90
        palette = {"canvas": [w, h], "palette": [
            {"index": i, "rgb": [12 + 7 * i, 14 + 6 * i, 18 + 5 * i],
             "role": "ground"} for i in range(7)
        ] + [{"index": 7, "rgb": [40, 180, 170], "role": "ink"}]}

        def picture(lift=0, ink="present"):
            image = np.zeros((h, w, 3), dtype=np.uint8)
            for y in range(h):
                shade = int(12 + y * 0.4 + lift)
                image[y, :] = [shade, shade + 2, shade + 6]
            if ink != "missing":
                x0 = 95 if ink == "moved" else 30
                colour = [230, 20, 220] if ink == "wrong-colour" else [40, 180, 170]
                image[25:65, x0:x0 + 50] = colour
            if ink == "novel":
                image[68:86, 125:155] = [230, 20, 220]
            return image

        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            def spec(name, image):
                path = tmp / (name + ".png")
                Image.fromarray(image).save(path)
                data, _, _ = extract(path, (w, h), 8, palette)
                out = tmp / (name + ".json")
                out.write_text(json.dumps(data))
                return out

            source = spec("source", picture())
            candidate = {
                name: spec(name, picture(lift=12, ink=name))
                for name in ("present", "missing", "moved", "wrong-colour", "novel")
            }
            checker = Path(__file__).with_name("spec_diff.py")
            for name, path in candidate.items():
                with self.subTest(name=name):
                    result = subprocess.run(
                        [sys.executable, str(checker), "--gate", "inks", str(source), str(path)],
                        capture_output=True, text=True, check=False)
                    self.assertEqual(result.returncode == 0,
                                     name in ("present", "novel"), result.stdout)
                    if name == "novel":
                        self.assertIn("candidate pixels outside source palette", result.stdout)
                        self.assertGreater(json.loads(path.read_text())["palette_residual"]
                                           ["rgb_distance_at_least_110"], 0.02)


class StripedRegionTests(unittest.TestCase):
    def setUp(self):
        self.body = np.zeros((100, 160), bool)
        self.body[10:90, 10:130] = True
        self.colours = [np.array([246., 185., 105.]), np.array([217., 160., 91.])]

    def repair(self, grain, body=None, colours=None):
        body = self.body if body is None else body
        masks = [body, grain]
        filled = [ndimage.binary_fill_holes(ndimage.binary_closing(m, np.ones((5, 5))))
                  for m in masks]
        return filled, repair_striped_regions(masks, filled, colours or self.colours)

    def test_aliased_grain_recovers_body_in_both_orientations(self):
        # Strands mostly 3 px apart, with broad aliasing slots that the old
        # 5x5 close cannot bridge. A second ink measures the complete body.
        grain = self.body & (np.indices(self.body.shape)[1] % 3 == 0)
        grain[:, 55:67] = False
        for transposed in (False, True):
            body, strands = (self.body.T, grain.T) if transposed else (self.body, grain)
            before, after = self.repair(strands, body)
            self.assertFalse(np.all(before[1][body]))
            np.testing.assert_array_equal(after[1], body)
            np.testing.assert_array_equal(after[0], before[0])

    def test_neighbouring_body_and_background_gap_are_not_joined(self):
        body = self.body.copy()
        body[10:90, 135:155] = True
        grain = self.body & (np.indices(body.shape)[1] % 3 == 0)
        grain[:, 55:67] = False
        _, after = self.repair(grain, body)
        np.testing.assert_array_equal(after[1], self.body)

    def test_text_rules_and_sparse_edges_do_not_trigger_repair(self):
        glyphs = np.zeros_like(self.body)
        for y in (25, 45, 65):
            for x in range(15, 125, 10):
                glyphs[y:y+7, x:x+4] = True
        rules = np.zeros_like(self.body)
        rules[20:22, 15:125] = True
        rules[75:77, 15:125] = True
        for mask in (glyphs, rules, self.body):
            before, after = self.repair(mask)
            np.testing.assert_array_equal(after, before)

    def test_different_colour_overlay_does_not_become_body(self):
        grain = self.body & (np.indices(self.body.shape)[1] % 3 == 0)
        grain[:, 55:67] = False
        before, after = self.repair(grain, colours=[self.colours[0], np.array([20., 40., 190.])])
        np.testing.assert_array_equal(after, before)

    def test_stripes_without_independent_body_stay_fragmented(self):
        grain = self.body & (np.indices(self.body.shape)[1] % 3 == 0)
        grain[:, 55:67] = False
        before, after = self.repair(grain, body=np.zeros_like(grain))
        np.testing.assert_array_equal(after, before)


if __name__ == '__main__':
    unittest.main()
