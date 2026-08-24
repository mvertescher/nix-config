# nixos hyprland configuration

{ pkgs, lib, config, ... }:

let
  cfg = config.custom.greetd;
in
{
  options.custom.greetd = {
    lastUser = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      example = "alice";
      description = ''
        Account name to seed tuigreet's `lastuser` cache with, so the
        very first boot of a freshly installed host already has the
        username filled in.

        `null` leaves the cache empty, which is the right default for a
        shared module: which account sits at the console is a per-host
        fact, and pre-filling the wrong name is worse than pre-filling
        none. Only the *first* login is affected either way -- tuigreet
        runs with `--remember` and writes the file itself afterwards.
      '';
    };
  };

  config = {
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
    # Found the hard way 2026-08-23: SUPER+backspace produced a locked
    # session and a dead hyprlock. Checking that the binary, config and
    # keybind all existed was not the same as checking that it locks and
    # unlocks, and the earlier "lock screen works" note said the former
    # while meaning the latter.
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

    # Only the file, not the directory: nixpkgs' own greetd module already
    # emits `d '/var/cache/tuigreet' - greeter greeter - -` whenever greetd
    # is enabled (see nixos/modules/services/display-managers/greetd.nix,
    # "Create directories potentially required by supported greeters"), so a
    # second rule for the same path here was redundant and had systemd-tmpfiles
    # reconciling two specs for it. Seeding the file is opt-in; without it
    # tuigreet's --remember still populates the cache after the first login.
    systemd.tmpfiles.rules = lib.optional (cfg.lastUser != null)
      "f /var/cache/tuigreet/lastuser 0644 greeter greeter - ${cfg.lastUser}";
  };
}
