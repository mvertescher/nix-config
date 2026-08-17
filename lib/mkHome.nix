{ extraHomeConfig, inputs, pkgs, ... }:

let
  lib = pkgs.lib;

  modules' = [
  ];

  privateConfig = import ./private-config.nix { inherit lib; } extraHomeConfig;

  # TODO: refactor these
  mkHome = { mut ? false, mods ? [ ] }:
    inputs.home-manager.lib.homeManagerConfiguration {
        inherit pkgs;

        # extraSpecialArgs = pkgs.xargs;
        # modules = modules' ++ mods ++ [
        #   { inherit hidpi; dotfiles.mutable = mut; }
        # ];

        modules = modules' ++ mods ++ [
          inputs.stylix.homeModules.stylix
        ];
    };

  mkDesktopHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [ ../home/host/desktop.nix ] ++ privateConfig "desktop";
  };

  mkLaptopHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [ ../home/host/laptop ] ++ privateConfig "laptop";
  };

in
{
  desktop = mkDesktopHome { };
  laptop = mkLaptopHome { };
}
