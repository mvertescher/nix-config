# server specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
  ];

  # Enable Stylix on server to cleanly colorize headless terminal prompts!
  stylix = {
    enable = true;
    base16Scheme = ../common/cybrcolors.yaml;
    image = config.lib.stylix.pixel "base0C";
  };

  home.stateVersion = "25.05";
}