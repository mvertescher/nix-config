# Standalone home-manager configurations, reached as
# `pkgs.builders.mkHome { extraHomeConfig }` (or `lib.mkHome`). This is
# the frozen entry point for the home-manager-only wrapper: its call
# sites can't be updated from here, so signature changes must be
# additive. Per host, any matching modules under
# <extraHomeConfig>/host are merged in (see ./private-config.nix).
{ extraHomeConfig, inputs, pkgs, ... }:

let
  lib = pkgs.lib;

  privateConfig = import ./private-config.nix { inherit lib; } extraHomeConfig;

  # mkDefault so the bare library evaluates on its own; a wrapper's
  # definitions (via extraHomeConfig) override cleanly.
  defaults = {
    home.username = lib.mkDefault "mverte";
    home.homeDirectory = lib.mkDefault "/home/mverte";
  };

  mkHome = host: mods:
    inputs.home-manager.lib.homeManagerConfiguration {
      inherit pkgs;
      modules = mods ++ privateConfig host ++ [
        inputs.stylix.homeModules.stylix
        defaults
      ];
    };
in
{
  desktop = mkHome "desktop" [ ../home/host/desktop.nix ];
  laptop = mkHome "laptop" [ ../home/host/laptop ];
}
