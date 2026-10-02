# CP Eras Kitsch Sans Bold

This font is a modified derivative of GNU FreeFont 20120503
`FreeSansBold.ttf`, from the pinned Nix `freefont_ttf` package. The source
font's copyright, license, license URL, vendor, and credits name records
remain embedded. The package also ships upstream `COPYING`, `README`, and
`CREDITS` in this directory. The modified font remains under the upstream
GNU GPL terms and font embedding exception described in those files.

`scripts/make-kitsch-sans.py` is the corresponding reproducible modification
recipe. It verifies upstream SHA-256
`982534a3731416a15e2756601721f26053f68bf4239011550f3dd23ce6308215`,
changes only the M glyph's topology and five contour coordinates, remaps
eight point/phantom-point operands in M's TrueType instruction stream, and
renames the family/full/unique/PostScript name records to avoid changing
FreeSans uses elsewhere. All other glyphs, shared hint tables, metrics,
style weight, and timestamp stay unchanged. Derived TTF SHA-256:
`14d51cf24626c4c3103843d71d63b7a432cdc9945693397d193eeaf06827550c`.

The font is loaded only to render the three ordinary Kitsch Store cards'
second compliance line. The selected card and other FreeSans text retain
the upstream font.
