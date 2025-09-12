# server specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../home.nix
  ];

  home.homeDirectory = "/home/mvertescher";
  home.stateVersion = "25.05";
}