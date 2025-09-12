# server specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../home.nix
  ];

  home.stateVersion = "25.05";
}