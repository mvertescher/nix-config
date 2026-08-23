# neokitsch's binding of the shared role machinery (see ../lib/roles.nix).
#
# Kept as a plain function file for the same reason entropism's and
# neomil's are: a wrapper needs the identical scheme outside
# home-manager, because NixOS-level stylix themes the console and
# greeter and cannot see the home config.
#
#   scheme = import .../kitsch/scheme.nix;
#   stylix.base16Scheme = scheme.forVariant { variant = "reference"; };
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

  # Neokitsch reuses the hook kitsch established, with a narrower
  # hand: where kitsch has three working chromatics, this era has one
  # family. Spreading gold across the syntax slots keeps an editor
  # legible without inventing hues the references do not contain -- the
  # separation is by brightness, which is how the era does hierarchy
  # everywhere else too.
  accents = {
    base0B = "d3b279"; # champagne -- strings
    base0C = "b08a4a"; # mid gold
    base0D = "e7c686"; # gold text -- functions
    base0E = "fcc474"; # amber -- keywords
  };

  toBase16 =
    variant: resolved:
    roles.toBase16 {
      name = "Neokitsch";
      inherit variant accents;
      roles = resolved;
    };

  forVariant = args: toBase16 (args.variant or "reference") (resolve args);

  polarityOf = resolved: roles.polarityOf resolved;
}
