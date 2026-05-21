# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
    ../common/hyprland
  ];

  stylix = {
    enable = true;
    base16Scheme = ../common/cybrcolors.yaml;
    image = config.lib.stylix.pixel "base0C";
  };

  home.stateVersion = "25.05";
}