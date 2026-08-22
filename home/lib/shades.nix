# Derived shades for base16 palettes.
#
# base16 gives exactly one value per accent, but the desktop components
# want dimmer variants of each: a mid shade for borders and inactive
# text, and a dark shade for filled backgrounds. Those used to be
# written out as literals per component, which meant they kept pointing
# at cybr's reds and greens no matter which scheme was active.
#
# Blending the accent toward base00 reproduces the cybr values exactly
# (its palette was built at these ratios) while following whatever
# scheme is loaded. Usage:
#
#   shades = import ../../lib/shades.nix { inherit lib; };
#   inherit (shades.forColors config.lib.stylix.colors) mid dark;
#   # mid "base08" -> "631f21" on cybr
{ lib }:

let
  clamp = v: if v < 0 then 0 else if v > 255 then 255 else v;

  # ratio = how much of the accent survives; the remainder is base00.
  mixChannel =
    ratio: accent: background:
    clamp (builtins.floor (((accent * ratio) + (background * (1.0 - ratio))) + 0.5));

  toHex2 = n: lib.toLower (lib.fixedWidthString 2 "0" (lib.toHexString n));
in
rec {
  # Blend one base16 slot toward base00 at an arbitrary ratio, returning
  # a bare six-digit hex string (no leading '#', so callers can append an
  # alpha suffix).
  mix =
    colors: ratio: name:
    let
      channel = c: lib.toInt colors."${name}-rgb-${c}";
      background = c: lib.toInt colors."base00-rgb-${c}";
      component = c: toHex2 (mixChannel ratio (channel c) (background c));
    in
    "${component "r"}${component "g"}${component "b"}";

  # The two ratios the cybr components actually use. Named rather than
  # inlined so a component asking for "the dark variant" cannot drift
  # from another component asking for the same thing.
  midRatio = 0.4;
  darkRatio = 0.2;

  # Convenience wrapper: bind the palette once, then ask for shades.
  forColors = colors: {
    mid = mix colors midRatio;
    dark = mix colors darkRatio;
    inherit colors;
  };
}
