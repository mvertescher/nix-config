# terra specific nixos configuration

{ config, pkgs, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ../../wm/hyprland.nix
    ../../fonts
  ];

  # Use systemd boot
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Use latest kernel.
  boot.kernelPackages = pkgs.linuxPackages_latest;

  networking = {
    hostName = "terra";
  };

  # Enable OpenGL
  hardware.graphics.enable = true;

  # Load nvidia driver for Wayland
  services.xserver.videoDrivers = [ "nvidia" ];

  hardware.nvidia = {
    # Modesetting is required
    # modesetting.enabled = true;

    powerManagement.enable = false;
    powerManagement.finegrained = false;

    # Use the open source kernel module
    open = true;

    # Enable `nvidia-settings`
    nvidiaSettings = true;

    package = config.boot.kernelPackages.nvidiaPackages.stable;
  };

  programs.steam.enable = true;

  services.hardware.openrgb.enable = true;

  stylix.enable = true;
  # stylix.base16Scheme = "${pkgs.base16-schemes}/share/themes/gruvbox-dark-hard.yaml";
  stylix.base16Scheme = "${pkgs.base16-schemes}/share/themes/nord.yaml";

  # Never change this.
  system.stateVersion = "25.05";
}
