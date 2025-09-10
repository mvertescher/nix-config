# laptop specific home manager configuration

{ pkgs, ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  home.stateVersion = "25.05";
}