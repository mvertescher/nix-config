# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  stylix = {
    enable = true;
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/gruvbox-dark-hard.yaml";
    base16Scheme = "${pkgs.base16-schemes}/share/themes/nord.yaml";
    image = config.lib.stylix.pixel "base0C";
  };

  home.stateVersion = "25.05";
}