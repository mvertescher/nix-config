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

    greeter = lib.mkOption {
      type = lib.types.enum [ "tuigreet" "cp-eras-ui" ];
      default = "tuigreet";
      description = ''
        Which greeter greetd runs on the console.

        `tuigreet` is the text greeter on the tty, with the username
        remembered across boots. `cp-eras-ui` is this repo's own login
        screen (`home/common/pkgs/cp-eras-ui`, `cp-eras-ui-login
        --greet`) run under the `cage` kiosk compositor: the era's
        access screen from the traces, password field live, dressed in
        `custom.greetd.era` and signing in `custom.greetd.user`. It
        asks nothing about the account -- the greeter user has no
        theme to follow and no username to remember, so both are
        options here rather than state on disk.

        Left on tuigreet by default. `tests/greeter.nix` (the flake's
        `checks.x86_64-linux.greeter`) signs a user in through this
        greeter on a virtio GPU in a NixOS VM, so the seat, PAM and the
        handover are covered; a real card under the `greeter` account
        is the one thing it cannot stand in for, and a greeter that
        fails to draw is a host with no way in short of a tty.
      '';
    };

    era = lib.mkOption {
      type = lib.types.enum [ "entropism" "kitsch" "neomil" "neokitsch" ];
      default = "neomil";
      description = ''
        The era `cp-eras-ui-login` is dressed in when it is the
        greeter. Only read when `greeter = "cp-eras-ui"`. Not tied to
        `hosts/<host>/theme.nix` on purpose: the greeter runs as the
        `greeter` account, which never sees the user's published
        theme, and the login screen's era is a fact about the console
        rather than about a session.
      '';
    };

    user = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = cfg.lastUser;
      defaultText = lib.literalExpression "config.custom.greetd.lastUser";
      description = ''
        The account `cp-eras-ui-login` signs in. The screen has a
        password field and no username field -- every era's trace
        shows the account already chosen -- so the name is given
        here, the way `lastUser` gives tuigreet its first guess.
        Required when `greeter = "cp-eras-ui"`.
      '';
    };

    session = lib.mkOption {
      type = lib.types.str;
      default = "${lib.getExe pkgs.uwsm} start hyprland-uwsm.desktop";
      defaultText = lib.literalExpression ''"''${lib.getExe pkgs.uwsm} start hyprland-uwsm.desktop"'';
      description = ''
        What the greeter starts once the password is accepted: the
        session command greetd runs as the signed-in user. Both
        greeters hand it over verbatim. One option rather than a string
        in each branch so the two cannot drift, and so a test can point
        it at something that leaves a mark (`tests/greeter.nix`).
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

    # greetd, with whichever greeter `custom.greetd.greeter` names.
    systemd.services.greetd.serviceConfig = {
      Type = "idle";
      StandardInput = "tty";
      StandardOutput = "tty";
      StandardError = "journal";
      TTYReset = true;
      TTYHangup = true;
      TTYVDisallocate = true;
    };

    assertions = [
      {
        assertion = cfg.greeter != "cp-eras-ui" || cfg.user != null;
        message = "custom.greetd.greeter = \"cp-eras-ui\" needs custom.greetd.user (or lastUser): the login screen has no username field.";
      }
    ];

    services.greetd = {
      enable = true;
      settings = {
        default_session = {
          command =
            if cfg.greeter == "cp-eras-ui" then
              # `cage -s` allows VT switching so a broken greeter still
              # leaves a tty reachable; `-d` asks the client not to draw
              # its own decorations. cage has no "exit with the app"
              # flag because that is what it does: it runs one
              # application and terminates when it exits, and
              # `cp-eras-ui-login --greet` exits 0 once greetd has
              # accepted `start_session`, which is greetd's cue to
              # tear the greeter down and start the user's session.
              "${pkgs.cage}/bin/cage -s -d -- ${pkgs.cp-eras-ui}/bin/cp-eras-ui-login --greet --era ${cfg.era} --user ${cfg.user} --cmd '${cfg.session}'"
            else
              "${pkgs.tuigreet}/bin/tuigreet --time --remember --remember-session --cmd '${cfg.session}'";
          user = "greeter";
        };
      };
    };

    # cage under greetd gets its seat from logind: greetd opens a PAM
    # session for `greeter` on the VT, and libseat's logind backend
    # hands wlroots the DRM and input devices of the active session.
    # That is the documented greetd + cage arrangement and needs no
    # groups; the render node wgpu opens for Vulkan is world-readable
    # under udev's default rules. `video` is added anyway for libseat's
    # direct fallback, because a greeter that cannot open the GPU is a
    # black screen with no message. Exercised on a virtio-gpu seat by
    # tests/greeter.nix (2026-09-06); a real card is not.
    users.users.greeter.extraGroups = lib.mkIf (cfg.greeter == "cp-eras-ui") [ "video" ];

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
