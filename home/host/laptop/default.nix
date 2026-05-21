# laptop specific home manager configuration

{ pkgs, config, lib, ... }:

{
  imports = [
    ../../common/home.nix
    ../../common/hyprland
    ../../common/themes/cybr
  ];

  stylix.image = config.lib.stylix.pixel "base0B";

  wayland.windowManager.hyprland.settings = {
    monitor = [
      "eDP-1, preferred, auto, 1"
    ];
  };

  home.homeDirectory = "/home/mvertescher";
  home.stateVersion = "25.05";
}
