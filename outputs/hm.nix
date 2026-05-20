{ extraHomeConfig, inputs, pkgs, ... }:

let
  lib = pkgs.lib;

  modules' = [
  ];

  privateConfig = target:
    if builtins.typeOf extraHomeConfig == "path" then
      if builtins.pathExists (extraHomeConfig + "/host/${target}.nix") then
        [ (extraHomeConfig + "/host/${target}.nix") ]
      else if builtins.pathExists (extraHomeConfig + "/host/${target}") then
        [ (extraHomeConfig + "/host/${target}") ]
      else
        []
    else
      [];

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
