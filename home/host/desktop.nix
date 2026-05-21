# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
    ../common/hyprland
    ../themes/cybr
  ];

  home.stateVersion = "25.05";
}