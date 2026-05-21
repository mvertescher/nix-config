{ pkgs, config, lib, ... }:

{
  imports = [
    ./starship.nix
    ./firefox
    ./hyprlock.nix
    ./hyprpaper.nix
    ./swaync.nix
    ./waybar
  ];

  stylix = {
    enable = true;
    base16Scheme = ./cybrcolors.yaml;
    image = lib.mkDefault (config.lib.stylix.pixel "base0C");
  };

  programs.alacritty.settings.window.opacity = lib.mkForce 0.10;

  programs.alacritty.settings.colors.selection = {
    background = lib.mkForce "#${config.lib.stylix.colors.base0F}";
    text = lib.mkForce "#${config.lib.stylix.colors.base00}";
  };
}
