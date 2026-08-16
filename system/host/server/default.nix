# server specific nixos configuration

{ config, pkgs, ... }:

{
  imports = [
    ./hardware-configuration.nix
  ];

  boot.loader.grub.enable = true;
  boot.loader.grub.device = "/dev/vda";

  networking = {
    hostName = "server";
    useDHCP = true;
  };

  time.timeZone = "UTC";

  users.users.mverte.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILBrJP87O00JevRDmMIOvR23XvB820Ta62on5GGyTvMZ"
  ];

  security.sudo.wheelNeedsPassword = false;

  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    settings.PermitRootLogin = "no";
  };

  environment.systemPackages = with pkgs; [
    # A bare `rustup` package has no default toolchain until someone runs
    # `rustup default stable` by hand - not reproducible, and easy to end
    # up with a `cargo`/`rustc` that just errors "no default toolchain
    # configured." Pin a real toolchain declaratively instead, via
    # rust-overlay (already in the global overlay list - see
    # lib/overlays.nix) - same pattern as home/common/cli/programming.nix
    # (currently unused by any host, but the one place this was already
    # done once).
    (rust-bin.stable.latest.default.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];

  networking.firewall.allowedTCPPorts = [ 22 ];

  stylix = {
    enable = true;
    base16Scheme = "${pkgs.base16-schemes}/share/themes/nord.yaml";
    # home/themes/cybr sets its own image at mkDefault priority; letting the
    # system image auto-copy at the same priority causes a conflicting
    # definition, so don't propagate it to home-manager.
    homeManagerIntegration.followSystem = false;
  };

  # Never change this.
  system.stateVersion = "26.05";
}
