# neomil's binding of the shared role machinery (see ../lib/roles.nix).
#
# Kept as a plain function file for the same reason entropism's is: a
# wrapper needs the identical scheme outside home-manager, because
# NixOS-level stylix themes the console and greeter and cannot see the
# home config.
#
#   scheme = import .../neomil/scheme.nix;
#   stylix.base16Scheme = scheme.forVariant { variant = "bleach"; };
let
  roles = import ../lib/roles.nix;
in
rec {
  palettes = import ./palettes.nix;

  resolve =
    {
      variant ? "reference",
      overrides ? { },
    }:
    roles.resolve { inherit palettes variant overrides; };

  # The reference era is red-monochrome, so the default projection --
  # everything onto fg/dim/alert -- is faithful rather than lossy. The
  # one reclaim is base0A, which roles.nix already points at `tape`, so
  # the off-white keeps carrying labels and values.
  toBase16 =
    variant: resolved:
    roles.toBase16 {
      name = "Neomil";
      inherit variant;
      roles = resolved;
    };

  forVariant = args: toBase16 (args.variant or "reference") (resolve args);

  polarityOf = resolved: roles.polarityOf resolved;
}
