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
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/nord.yaml";
    # base16Scheme = "${pkgs.base16-schemes}/share/themes/tarot.yaml";
    base16Scheme = "${pkgs.base16-schemes}/share/themes/heetch.yaml";
    image = config.lib.stylix.pixel "base0B";
  };

  home.homeDirectory = "/home/mvertescher";
  home.stateVersion = "25.05";
}