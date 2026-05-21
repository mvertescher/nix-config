# laptop specific home manager configuration

{ pkgs, config, lib, ... }:

{
  imports = [
    ../../common/home.nix
    ../../common/hyprland
  ];

  stylix = {
    enable = true;
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/gruvbox-dark-hard.yaml";
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/nord.yaml";
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/tarot.yaml";
    base16Scheme = ../../common/themes/cybr/cybrcolors.yaml;
    image = config.lib.stylix.pixel "base0B";
  };

  wayland.windowManager.hyprland.settings = {
    monitor = [
      "eDP-1, preferred, auto, 1"
    ];
  };

  home.homeDirectory = "/home/mvertescher";
  home.stateVersion = "25.05";
}
