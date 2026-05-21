# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
    ../common/hyprland
  ];

  stylix = {
    enable = true;
    base16Scheme = ../common/themes/cybr/cybrcolors.yaml;
    image = config.lib.stylix.pixel "base0C";
  };

  home.stateVersion = "25.05";
}