# server specific nixos configuration

{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.disko.nixosModules.disko
    ./disko.nix
    ./hardware-configuration.nix
  ];

  # Legacy BIOS boot; disko sets boot.loader.grub.devices from the
  # EF02 (bios_grub) partition in disko.nix.
  boot.loader.grub.enable = true;

  networking = {
    hostName = "server";
    useDHCP = true;
  };

  time.timeZone = "UTC";

  # Static fallback keys — login keeps working if GitHub is unreachable.
  users.users.mverte.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILBrJP87O00JevRDmMIOvR23XvB820Ta62on5GGyTvMZ"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGnJOapYZ5xo+PGkl6pa9PKn5Oa86gyRXe/MYK/tGiPG mverte@devbox"
  ];

  security.sudo.wheelNeedsPassword = false;

  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    settings.PermitRootLogin = "no";

    # Also accept any key on the GitHub account, fetched at login time.
    # New machine access = `gh ssh-key add ~/.ssh/id_ed25519.pub` — no
    # rebuild/redeploy needed. Consulted in addition to the static keys
    # above, so a GitHub outage can't lock us out.
    authorizedKeysCommand = "${pkgs.writeShellScript "github-authorized-keys" ''
      [ "$1" = "mverte" ] || exit 0
      exec ${pkgs.curl}/bin/curl -sf --max-time 5 https://github.com/mvertescher.keys
    ''} %u";
    authorizedKeysCommandUser = "nobody";
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
