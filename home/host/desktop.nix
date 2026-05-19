# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  stylix = {
    enable = true;
    base16Scheme = ../cybrcolors.yaml;
    image = config.lib.stylix.pixel "base0C";
  };

  home.stateVersion = "25.05";
}