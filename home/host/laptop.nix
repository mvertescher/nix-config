# laptop specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  home.username = "mvertescher";
  home.homeDirectory = "/home/mvertescher";
  home.stateVersion = "25.05";
}