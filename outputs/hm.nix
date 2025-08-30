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
      ../home/gui/hyprland.nix
    ];
  };
in
{
    # TODO: Add multiple wm outputs
    mvertescher = mkHome { };
}
