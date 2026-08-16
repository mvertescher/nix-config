# server specific home manager configuration
#
# Headless host: imports CLI tooling and stylix terminal theming only,
# skipping common/home.nix's GUI packages and cybr's Hyprland/Wayland
# programs (firefox, hyprlock, waybar, swaync), which don't apply here.

{ pkgs, config, ... }:

let
  theme = import ../themes/cybr/colors/cybrcolors.nix;
in
{
  imports = [
    ../common/cli
    ../themes/cybr/starship.nix
  ];

  programs.home-manager.enable = true;

  # notifications about home-manager news
  news.display = "silent";

  stylix = {
    enable = true;
    base16Scheme = theme;
  };

  programs.vivid.enable = true;
  stylix.targets.vivid.enable = true;

  home.stateVersion = "25.05";
}
