# terra specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  home.username = "mverte";
  home.homeDirectory = "/home/mverte";
  home.stateVersion = "25.05";

  home.packages = with pkgs; [
  ];
}

