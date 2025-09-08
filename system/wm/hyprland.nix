# nixos hyprland configuration

{ pkgs, lib, ... }:

{
  programs.hyprland.enable = true;

  # TODO: maybe enable greetd
  # systemd.services.greetd.serviceConfig = {
  #   Type = "idle";
  #   StandardInput = "tty";
  #   StandardOutput = "tty";
  #   StandardError = "journal";
  #   TTYReset = true;
  #   TTYHangup = true;
  #   TTYVDisallocate = true;
  # };

  # services.greetd = {
  #   enable = true;
  #   settings =
  # };

}
