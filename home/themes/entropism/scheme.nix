# Role resolution and base16 derivation for entropism, as plain
# functions.
#
# The home-manager module in ./default.nix drives these from its own
# options, but a wrapper also needs the identical scheme outside
# home-manager — NixOS-level stylix themes the console and greeter, and
# it has no access to the home config. Keeping the maths here means the
# two cannot drift, which is exactly the failure mode of naming a theme
# in two places.
#
#   scheme = import .../entropism/scheme.nix;
#   stylix.base16Scheme = scheme.forVariant { variant = "burn-in"; };
rec {
  palettes = import ./palettes.nix;

  # variant palette < overrides, then `tape` falls back to the resolved
  # `fg` so retinting the foreground carries the label accent along.
  resolve =
    {
      variant ? "burn-in",
      overrides ? { },
    }:
    let
      base = palettes.${variant} // (builtins.intersectAttrs palettes.dead-pixel overrides);
    in
    base // { tape = base.tape or base.fg; };

  # Collapsing syntax highlighting into fg/dim/alert is the point, not an
  # oversight: an entropism display has one working colour. base08/09/0F
  # are alert, base0A carries the tape accent so a marker-written label
  # still reads as a label, everything else is fg or dim. No rainbow.
  toBase16 =
    variant: roles:
    let
      hex = role: builtins.replaceStrings [ "#" ] [ "" ] roles.${role};
    in
    {
      scheme = "Entropism ${variant}";
      author = "generated from themes/entropism";

      base00 = hex "bg";
      base01 = hex "panel";
      base02 = hex "border";
      base03 = hex "dim";
      base04 = hex "dim";
      # No lighten helper exists in this repo, and this is not worth a
      # colour-maths dependency, so the light end of the ramp is fg.
      base05 = hex "fg";
      base06 = hex "fg";
      base07 = hex "fg";

      base08 = hex "alert";
      base09 = hex "alert";
      base0A = hex "tape";
      base0B = hex "fg";
      base0C = hex "dim";
      base0D = hex "fg";
      base0E = hex "dim";
      base0F = hex "alert";
    };

  forVariant = args: toBase16 (args.variant or "burn-in") (resolve args);
}
