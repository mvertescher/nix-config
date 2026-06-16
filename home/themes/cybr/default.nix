{ pkgs, config, lib, ... }:

let
  theme = import ./colors/cybrcolors.nix;
  staticPixel = pkgs.runCommand "base0C-pixel.png" {
    color = "#${theme.base0C}";
  } "${lib.getExe' pkgs.imagemagick "convert"} xc:$color png32:$out";
in
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
    base16Scheme = theme;
    image = staticPixel;
  };

  programs.alacritty.settings.window.opacity = lib.mkForce 0.10;

  programs.alacritty.settings.colors.selection = {
    background = lib.mkForce "#${config.lib.stylix.colors.base0F}";
    text = lib.mkForce "#${config.lib.stylix.colors.base00}";
  };
}
