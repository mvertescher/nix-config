# server specific home manager configuration

{ pkgs, config, ... }:

{
  imports = [
    ../common/home.nix
    ../common/themes/cybr
  ];

  home.stateVersion = "25.05";
}