{ extraHomeConfig, inputs, system, pkgs, ... }:

let
  modules' = [
    extraHomeConfig
  ];

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
    mods = [ ../home/host/desktop.nix ];
  };

  mkLaptopHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [ ../home/host/laptop.nix ];
  };

  mkServerHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [ ../home/host/server.nix ];
  };

in
{
  desktop = mkDesktopHome { };
  laptop = mkLaptopHome { };
  server = mkServerHome { };
}
