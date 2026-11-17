# Shared pkgs constructor: one definition of "this repo's nixpkgs" so
# the flake's own pkgs and the builders (mkNixos, mkHome, mkDarwin)
# can't drift.
{ inputs, overlays }:

system:
import inputs.nixpkgs {
  inherit overlays system;
  config.allowUnfree = true;
  config.allowUnfreePredicate = _: true;
}
