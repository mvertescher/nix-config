{ extraHomeConfig, inputs, pkgs, ... }:

let
  lib = pkgs.lib;

  modules' = [
  ];

  hasPrivateConfig = target:
    builtins.typeOf extraHomeConfig == "path" &&
    builtins.pathExists (extraHomeConfig + "/host/${target}.nix");

  privateConfig = target:
    if hasPrivateConfig target then [ (extraHomeConfig + "/host/${target}.nix") ] else [];

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

  mkServerHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [ ../home/host/server.nix ] ++ privateConfig "server";
  };

in
{
  desktop = mkDesktopHome { };
  laptop = mkLaptopHome { };
  server = mkServerHome { };
}
