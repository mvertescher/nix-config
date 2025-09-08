{ ... }:

{
  imports = [
    ../home.nix
    ../hyprland
  ];

  home.username = "mverte";
  home.homeDirectory = "/home/mverte";

  home.stateVersion = "25.05";
}

