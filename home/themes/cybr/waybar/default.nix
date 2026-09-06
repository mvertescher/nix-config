{ pkgs, lib, config, ... }:

let
  shades = import ../../../lib/shades.nix { inherit lib; };

  # Rust replacement for upstream's mediaplayer.py, which needed python3 plus
  # PyGObject and the Playerctl GIR typelib. Talks D-Bus directly, so the
  # music module has no interpreter dependency at runtime. Built once in
  # `lib/overlays.nix`.
  inherit (pkgs) mpris-status;

  # NixOS stand-in for upstream's `waybar-updates`, an Arch/AUR pacman
  # helper that is not in nixpkgs. ./nix-updates.sh documents what it
  # reports; writeShellApplication shellchecks it at build time.
  #
  # nix stays out of runtimeInputs on purpose: the script wants the
  # client that matches the running daemon, which the session PATH
  # already provides, and pinning nixpkgs' copy would pull a second nix
  # into the home closure for one optional tooltip line.
  nix-updates = pkgs.writeShellApplication {
    name = "nix-updates";
    runtimeInputs = [ pkgs.coreutils pkgs.gnused pkgs.jq ];
    text = builtins.readFile ./nix-updates.sh;
  };

  cfg = config.custom.waybar;

  modulesTemplate = builtins.readFile ./cybr-waybar/modules.jsonc;

  templatedModules = builtins.replaceStrings
    [
      "~/.config/waybar/scripts/mediaplayer.py"
      "@nix-updates@"
    ]
    [
      (lib.getExe mpris-status)
      ("${lib.getExe nix-updates}"
        + lib.optionalString (cfg.flakeLock != null) " ${cfg.flakeLock}")
    ]
    modulesTemplate;
in
{
  options.custom.waybar = {
    flakeLock = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      example = lib.literalExpression
        ''"''${config.home.homeDirectory}/nix-config/flake.lock"'';
      description = ''
        Path to a flake.lock whose input age the `custom/updates` module
        should report alongside the pending-reboot and last-switch
        signals. `null` leaves that signal off.

        Which repo built the system is a per-host fact this shared theme
        cannot know, so there is nothing sensible to default it to: a host
        config has to supply it. Nothing breaks if the path is wrong or
        goes missing -- ./nix-updates.sh only reports the age when the
        file is readable, and fails closed to `{}` otherwise.
      '';
    };
  };

  config = {
    # Launched by waybar's own systemd user unit rather than hyprland's
    # exec-once. See ../../lib/era.nix for the reasoning; the short version
    # is that exec-once only fires when the compositor starts, so switching
    # into this theme on a live session left the desktop with no bar.
    #
    # home-manager drops its `pkill -USR2 waybar` onChange hooks as soon as
    # systemd.enable is set, so the files this theme generates have to be
    # named here or a palette change would never reach a running bar.
    #
    # Split by what waybar can absorb: it rereads its stylesheet on SIGUSR2,
    # which home-manager already wires as an ExecReload plus a reload
    # trigger on style.css, so colors.css joins that rather than forcing a
    # restart for a colour change. The module list is only read at startup,
    # so config.jsonc and modules.jsonc are restart triggers.
    programs.waybar = {
      enable = true;
      systemd.enable = true;
    };

    systemd.user.services.waybar.Unit = {
      X-Reload-Triggers = [ "${config.xdg.configFile."waybar/colors.css".source}" ];
      X-Restart-Triggers = [
        "${config.xdg.configFile."waybar/config.jsonc".source}"
        "${config.xdg.configFile."waybar/modules.jsonc".source}"
      ];
    };

    # Every family below is base16 accent + two derived shades. The mid and
    # dark variants used to be literals lifted from cybr's 33-colour
    # palette.nix, which meant they stayed cybr-red no matter which scheme
    # stylix had loaded. Deriving them keeps the bar consistent with the
    # active theme.
    xdg.configFile."waybar/colors.css".text =
      let
        inherit (shades.forColors config.lib.stylix.colors) mid dark;
        family = name: slot: ''
          @define-color ${name}0 #${config.lib.stylix.colors.${slot}};
          @define-color ${name}1 #${mid slot};
          @define-color ${name}2 #${dark slot};
        '';
      in
      ''
        /* Generated dynamically from Stylix active palette */
        @define-color no0 #${config.lib.stylix.colors.base00};
        @define-color no1 #${config.lib.stylix.colors.base01};
        @define-color no2 #${config.lib.stylix.colors.base02};

        ${family "re" "base08"}
        ${family "gr" "base0B"}
        ${family "ye" "base0A"}
        ${family "bl" "base0D"}
        ${family "vi" "base0E"}
        ${family "cy" "base0C"}
        ${family "or" "base09"}
        ${family "pi" "base0F"}
        /* wh/me are foreground ramps rather than accents: base16 already
           carries the mid step (base03/base04), so only the dark end is
           derived. */
        @define-color wh0 #${config.lib.stylix.colors.base05};
        @define-color wh1 #${config.lib.stylix.colors.base03};
        @define-color wh2 #${dark "base05"};

        @define-color me0 #${config.lib.stylix.colors.base04};
        @define-color me1 #${mid "base04"};
        @define-color me2 #${dark "base04"};
      '';

    xdg.configFile."waybar/config.jsonc".source = ./cybr-waybar/config.jsonc;
    xdg.configFile."waybar/style.css".source = lib.mkForce ./cybr-waybar/style.css;
    xdg.configFile."waybar/output-switcher.sh".source = ./cybr-waybar/output-switcher.sh;
    xdg.configFile."waybar/scripts".source = ./cybr-waybar/scripts;
    xdg.configFile."waybar/svg".source = ./cybr-waybar/svg;
    xdg.configFile."waybar/modules.jsonc".text = templatedModules;

    fonts.fontconfig.enable = true;

    home.packages = [
      mpris-status
      pkgs.nerd-fonts.geist-mono

      # The pulseaudio module's on-click. Declared here rather than in the
      # shared GUI module for the same reason rofi/default.nix declares
      # its scripts' dependencies next to the scripts: nothing else in the
      # config calls it, and if the handler goes so should the package.
      pkgs.pavucontrol
    ];
  };
}
