{ pkgs, config, lib, ... }:

{
  imports = [
    ./starship.nix
  ];

  stylix = {
    enable = true;
    base16Scheme = ./cybrcolors.yaml;
    image = lib.mkDefault (config.lib.stylix.pixel "base0C");
  };

  programs.alacritty.settings.window.opacity = lib.mkForce 0.10;
}
