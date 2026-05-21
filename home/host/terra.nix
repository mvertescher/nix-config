# terra specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../common/home.nix
    ../common/hyprland
    ../common/themes/cybr
  ];

  home.username = "mverte";
  home.homeDirectory = "/home/mverte";
  home.stateVersion = "25.05";

  home.packages = with pkgs; [
  ];
}

