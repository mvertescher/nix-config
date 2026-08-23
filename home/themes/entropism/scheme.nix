# entropism's binding of the shared role machinery.
#
# The resolution and base16 projection live in ../lib/roles.nix so the
# other generated eras (neomil, kitsch, neokitsch) reuse them; this file
# just binds entropism's palettes to it.
#
# It stays a plain function file rather than a module because a wrapper
# needs the same scheme outside home-manager: NixOS-level stylix themes
# the console and greeter and cannot see the home config.
#
#   scheme = import .../entropism/scheme.nix;
#   stylix.base16Scheme = scheme.forVariant { variant = "burn-in"; };
let
  roles = import ../lib/roles.nix;
in
rec {
  palettes = import ./palettes.nix;

  resolve =
    {
      variant ? "nexus",
      overrides ? { },
    }:
    roles.resolve { inherit palettes variant overrides; };

  toBase16 =
    variant: resolved:
    roles.toBase16 {
      name = "Entropism";
      inherit variant;
      roles = resolved;
    };

  forVariant = args: toBase16 (args.variant or "nexus") (resolve args);
}
