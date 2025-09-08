{ extraHomeConfig, inputs, system, pkgs, ... }:

let
  modules' = [
    extraHomeConfig
  ];

  mkHome = { mut ? false, mods ? [ ] }:
    inputs.home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        # extraSpecialArgs = pkgs.xargs;
        # modules = modules' ++ mods ++ [
        #   { inherit hidpi; dotfiles.mutable = mut; }
        # ];

        modules = modules' ++ mods ++ [ ../home/home.nix ];
    };

  mkHyprlandHome = { mut ? false }: mkHome {
    inherit mut;
    mods = [
      ../home/hyprland
    ];
  };
in
{
  # TODO: Add multiple wm outputs
  # mvertescher = mkHome { };
  
  desktop = mkHyprlandHome { };
  laptop = mkHyprlandHome { };
  server = mkHome { };
}
