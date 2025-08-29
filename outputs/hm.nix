{ inputs, system, pkgs, ... }:

let
  mkHome = { mut ? false, mods ? [ ] }:
    inputs.home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        # extraSpecialArgs = pkgs.xargs;
        # modules = modules' ++ mods ++ [
        #   { inherit hidpi; dotfiles.mutable = mut; }
        # ];

        modules = [ ../home/home.nix ];
    };
in
{
    # TODO: Add multiple wm outputs
    mvertescher = mkHome { };
}
