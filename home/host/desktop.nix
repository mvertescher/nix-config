# laptop specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
    ../common/hyprland
    ../common/themes/cybr
  ];

  home.stateVersion = "25.05";
}