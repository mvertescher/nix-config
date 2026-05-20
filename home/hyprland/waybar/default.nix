{ pkgs, ... }:

{
  programs.waybar = {
    enable = true;
  };

  xdg.configFile."waybar" = {
    source = ./cybr-waybar;
    recursive = true;
  };

  fonts.fontconfig.enable = true;

  home.packages = with pkgs; [
    nerd-fonts.geist-mono
  ];
}
