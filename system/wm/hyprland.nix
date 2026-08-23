# nixos hyprland configuration

{ pkgs, lib, ... }:

{
  programs.hyprland = {
    enable = true;
    withUWSM = true;
  };

  # hyprlock authenticates through PAM, and a PAM service can only be
  # declared at the system level. The themes configure hyprlock through
  # home-manager (see home/themes/lib/era.nix), which installs the
  # binary and writes its config but cannot create /etc/pam.d/hyprlock
  # -- so without this the lock screen takes the session lock, fails to
  # start PAM, and exits, leaving the compositor holding a lock with no
  # client to accept a password.
  #
  # Found the hard way on terra 2026-08-23: SUPER+backspace produced a
  # locked session and a dead hyprlock. Checking that the binary, config
  # and keybind all existed was not the same as checking that it locks
  # and unlocks, and the earlier "lock screen works" note said the
  # former while meaning the latter.
  #
  # Left as the bare service so it inherits the system defaults, which
  # is what NixOS's own programs.hyprlock module does. We do not use
  # that module because the theme owns the configuration and enabling it
  # would install and configure hyprlock a second time.
  security.pam.services.hyprlock = { };

  # Enable greetd display manager with tuigreet
  systemd.services.greetd.serviceConfig = {
    Type = "idle";
    StandardInput = "tty";
    StandardOutput = "tty";
    StandardError = "journal";
    TTYReset = true;
    TTYHangup = true;
    TTYVDisallocate = true;
  };

  services.greetd = {
    enable = true;
    settings = {
      default_session = {
        command = "${pkgs.tuigreet}/bin/tuigreet --time --remember --remember-session --cmd '${pkgs.uwsm}/bin/uwsm start hyprland-uwsm.desktop'";
        user = "greeter";
      };
    };
  };

  # Pre-populate tuigreet cache so username is automatically pre-filled on first boot
  systemd.tmpfiles.rules = [
    "d /var/cache/tuigreet 0755 greeter greeter - -"
    "f /var/cache/tuigreet/lastuser 0644 greeter greeter - mverte"
  ];

}
