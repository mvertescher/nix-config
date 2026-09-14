#!/usr/bin/env python3
"""Focused segmentation regressions; run with Python + numpy/scipy/Pillow."""
import unittest

import numpy as np
from scipy import ndimage

from extract_spec import repair_striped_regions


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
