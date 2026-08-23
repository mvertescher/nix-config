# kitsch's binding of the shared role machinery (see ../lib/roles.nix).
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

  # Kitsch is the first era to use `accents`, and it is the reason the
  # hook exists. The default projection collapses syntax onto
  # fg/dim/alert, which is right for a one-colour era; kitsch has three
  # working chromatics -- teal, yellow, orange -- plus a mint that only
  # ever appears as a highlight band. Leaving them collapsed would throw
  # the era's whole palette away in every editor and terminal.
  #
  # The mint is the one value here not carried by a role: it is a
  # highlight *fill* in the references and never text, so it has no
  # business among the seven but is exactly right for base0B.
  accents = {
    base0B = "87f4d9"; # mint -- strings
    base0C = "1cb39b"; # solid teal -- the page-curl colour
    base0D = "7ddec8"; # teal -- functions
    base0E = "f08c1e"; # bezel orange -- keywords
  };

  toBase16 =
    variant: resolved:
    roles.toBase16 {
      name = "Kitsch";
      inherit variant accents;
      roles = resolved;
    };

  forVariant = args: toBase16 (args.variant or "reference") (resolve args);

  polarityOf = resolved: roles.polarityOf resolved;
}
