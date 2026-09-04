# The CJK glyphs cp-eras-ui compiles in, and nothing else.
#
# The toolkit ships its faces as `include_bytes!` so a render is the
# same on every host and in the golden sandbox, which has no fontconfig
# fonts at all. Noto Sans CJK is ~16MB per weight, far too much to
# embed for the three kanji the neomil store's MASURAO logotype sets
# (`docs/neomil/store-trace.svg:253`, Noto Sans CJK JP 700). So: take
# the JP face out of nixpkgs' variable-font collection, keep only
# `text`, and freeze the weight axis at 700. ~5KB.
#
# The family name is left as "Noto Sans CJK JP" on purpose: cosmic-text's
# script fallback for Han asks for that family by name before it
# resorts to "any face with the glyph", so the bundled face is what a
# Rajdhani run falls back to whether or not the host has the real one.
{ lib
, runCommand
, noto-fonts-cjk-sans
, python3Packages
, text ? "益荒男"
}:

runCommand "noto-cjk-subset"
{
  nativeBuildInputs = [ python3Packages.fonttools ];
  inherit text;
  meta = {
    description = "Noto Sans CJK JP Bold, subset to the glyphs cp-eras-ui embeds";
    license = lib.licenses.ofl;
  };
} ''
  vf=${noto-fonts-cjk-sans}/share/fonts/opentype/noto-cjk/NotoSansCJK-VF.otf.ttc
  # face 0 of the collection is JP
  pyftsubset "$vf" --font-number=0 --text="$text" \
    --no-hinting --name-IDs='*' --output-file=subset-vf.otf
  fonttools varLib.instancer --update-name-table -o subset-bold.otf subset-vf.otf wght=700
  install -D subset-bold.otf $out/share/fonts/opentype/NotoSansCJKjp-Bold-subset.otf
''
