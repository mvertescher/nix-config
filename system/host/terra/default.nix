# terra specific nixos configuration

{ pkgs, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ../../wm/hyprland.nix
  ];

  # Use systemd boot
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;
 
  # Use latest kernel.
  boot.kernelPackages = pkgs.linuxPackages_latest;

  networking = {
    hostName = "terra";
  };

  # Never change this.
  system.stateVersion = "25.05";
}
