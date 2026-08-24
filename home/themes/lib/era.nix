# The shared desktop for a generated Cyberpunk 2077 style era.
#
# entropism, neomil, kitsch and neokitsch differ in palette, typeface
# and a handful of knobs -- not in plumbing. Every one of them wants the
# same set of surfaces styled from the same seven roles: hyprland's
# borders, a bar, a launcher, notifications, the prompt, the wallpaper
# and the browser chrome. That work lives here once.
#
# An era module resolves its own roles (see ./roles.nix), then calls
# this with them. It gets back a home-manager configuration; the era is
# left holding only its palettes and its options.
#
# cybr deliberately does not use this: it vendors upstream assets rather
# than generating anything.
{
  lib,
  pkgs,
  config,
  # Display name, for generated headers and the wallpaper's store name.
  name,
  variant,
  # Resolved seven-role palette.
  roles,
  # { package, name } for bar/launcher/notification chrome. Terminal
  # content keeps stylix.fonts.monospace so code stays legible.
  font,
  # "none" | "scanlines" | "noise"
  texture ? "none",
  # Restart a running Firefox when the theme changes. userChrome is only
  # read at startup, so an open window otherwise keeps the old look.
  browserRestart ? true,
  # Which status bar to run. "waybar" is the long-standing default;
  # "cyberpunk-ui" is our own layer-shell bar, which is the only one
  # that can draw the era's actual corner treatment -- waybar styles
  # with CSS, where a chamfer or a clipped corner cannot be expressed.
  bar ? "waybar",
  # Per-era styling knobs. The defaults are the shared "hard edges"
  # house style; an ornamental era overrides them.
  knobs ? { },
  # Lock screen. Everything it generates is mkDefault, so a host
  # overrides any individual setting with a plain definition; set
  # enable = false to opt out of the theme's lock entirely and drive
  # programs.hyprlock yourself.
  lock ? { },
}:

let
  c = roles;

  rolesLib = import ./roles.nix;

  # Built here rather than taken from home.packages so the era owns its
  # bar the way it owns every other component skin.
  cyberpunk-ui = pkgs.callPackage ../../common/pkgs/cyberpunk-ui {
    orbitron = pkgs.callPackage ../../common/pkgs/orbitron { };
    rajdhani-fontshare = pkgs.callPackage ../../common/pkgs/rajdhani-fontshare { };
  };

  useOwnBar = bar == "cyberpunk-ui";

  k = {
    # Corner radius across hyprland, bar, launcher and browser chrome.
    radius = 0;
    # Bar height in pixels.
    barHeight = 22;
    # Drawn between bar modules; "" for none.
    separator = "1px solid ${c.border}";
    # Active elements swap fg/bg rather than glowing.
    invertActive = true;
    # Show the hostname as a tape-coloured label at the far left.
    hostTape = true;
  }
  // knobs;

  lk = {
    enable = true;
    # Set to "" for no binding at all.
    bind = "SUPER, backspace";
  }
  // lock;

  magick = lib.getExe' pkgs.imagemagick "magick";

  header = "Generated from home/themes/${lib.toLower name} roles. No literals here.";

  # Activation runs with an empty environment -- no ambient PATH -- so
  # every tool it calls is named by store path.
  bin = p: n: "${p}/bin/${n}";
  hyprctl = bin config.wayland.windowManager.hyprland.package "hyprctl";
  coreutil = bin pkgs.coreutils;

  # Generated rather than shipped, so the wallpaper follows a colour
  # override instead of going stale as an asset with a baked-in tint.
  wallpaper =
    pkgs.runCommand "${lib.toLower name}-${variant}-${texture}.png"
      {
        bg = c.bg;
        line = c.panel;
      }
      (
        {
          none = ''
            ${magick} -size 3840x2160 xc:"$bg" png32:$out
          '';

          scanlines = ''
            ${magick} -size 1x3 xc:"$bg" -size 1x1 xc:"$line" -append tile.png
            ${magick} -size 3840x2160 tile:tile.png png32:$out
          '';

          noise = ''
            ${magick} -size 3840x2160 xc:gray50 -attenuate 1.2 +noise Gaussian \
              -colorspace Gray grain.png
            ${magick} -size 3840x2160 xc:"$bg" grain.png \
              -compose blend -define compose:args=7 -composite png32:$out
          '';
        }
        .${texture}
      );

  # home/common/hyprland pins its colours and rounding with mkForce so
  # they beat stylix. A second mkForce here would collide at equal
  # priority rather than override, so an era sits one step stronger.
  eraOverride = lib.mkOverride 40;
  rgba = role: "rgba(${lib.removePrefix "#" c.${role}}ff)";

  # The resolved theme, published for programs that are not configured
  # through home-manager -- the in-tree Iced toolkits, chiefly.
  #
  # Written to a fixed, era-agnostic path so a reader does not have to
  # know which era is active, and carrying every role rather than a
  # curated subset so the toolkit can decide what it needs. This is the
  # authoritative copy: a Rust crate keeping its own hardcoded palette
  # is a second source of truth that drifts silently (neomil-ui's
  # colors.rs was transcribed by hand and is exactly that risk).
  #
  # The base seven are always present; the optional roles a maximalist
  # era declares follow them in the same block, in `extraNames` order.
  # An era that declares none emits the file it emitted before the
  # vocabulary was extended, byte for byte -- `extrasOf` filters on what
  # is actually in the resolved palette, not on what could be.
  themeToml = ''
    # ${header}
    # Read this rather than compiling a palette in; fall back to your own
    # defaults if the file is absent, so the crate still runs standalone.
    era = "${lib.toLower name}"
    variant = "${variant}"
    polarity = "${rolesLib.polarityOf roles}"

    [font]
    ui = "${font.name}"

    [colors]
    ${lib.concatStringsSep "\n" (
      map (role: ''${role} = "${c.${role}}"'') (rolesLib.names ++ rolesLib.extrasOf c)
    )}
  '';

  # Everything the browser chrome is generated from. Hashing the inputs
  # rather than the stylesheet means the stamp moves exactly when the
  # theme does.
  themeStamp = builtins.hashString "sha256" (
    builtins.toJSON {
      inherit variant name;
      colors = c;
      font = font.name;
    }
  );
in
lib.mkMerge [
  {
    home.packages = [
      font.package
      pkgs.rofi
    ]
    ++ lib.optional useOwnBar cyberpunk-ui;
    fonts.fontconfig.enable = true;

    stylix = {
      enable = true;
      image = lib.mkDefault wallpaper;
      fonts.sansSerif = lib.mkDefault { inherit (font) package name; };
    };

    xdg.configFile."theme/current.toml".text = themeToml;

    # --- daemons -------------------------------------------------------
    # The bar, the notification daemon and the wallpaper daemon are started
    # by systemd user units bound to graphical-session.target, not by
    # hyprland's `exec-once`.
    #
    # exec-once fires exactly once, when the compositor starts. That is
    # fine at login and wrong for every other way a theme changes: a
    # `./switch` from one theme to another stops the outgoing theme's units
    # and starts nothing, so the desktop is left with no bar, no
    # notifications and no wallpaper until the next logout. Terra sat in
    # that state from 2026-08-23 22:45 until it was noticed the next day,
    # and it also means a switch cannot verify anything about these three.
    #
    # Units fix both halves. sd-switch starts them when they appear and
    # restarts them when their definition changes, and home-manager already
    # wires X-Restart-Triggers on swaync's and hyprpaper's generated config,
    # so a palette change reaches a running daemon. waybar needs its
    # triggers named by hand, below.
    #
    # This works even though `wayland.windowManager.hyprland.systemd.enable`
    # is false in the shared hyprland module: the session comes up under
    # uwsm, which activates graphical-session.target itself. hypridle has
    # been proving that all along.
    # --- window manager ------------------------------------------------
    wayland.windowManager.hyprland.settings = {
      general = {
        border_size = eraOverride 1;
        "col.active_border" = eraOverride (rgba "fg");
        "col.inactive_border" = eraOverride (rgba "border");
      };

      decoration = {
        rounding = eraOverride k.radius;
        blur.enabled = eraOverride false;
        shadow.enabled = eraOverride false;
      };

      group = {
        "col.border_active" = eraOverride (rgba "fg");
        "col.border_inactive" = eraOverride (rgba "border");
        "col.border_locked_active" = eraOverride (rgba "alert");
        "col.border_locked_inactive" = eraOverride (rgba "border");

        groupbar = {
          "col.active" = eraOverride (rgba "fg");
          "col.inactive" = eraOverride (rgba "border");
          "col.locked_active" = eraOverride (rgba "alert");
          "col.locked_inactive" = eraOverride (rgba "border");
          text_color = eraOverride (rgba "bg");
        };
      };
    };

    # --- wallpaper -----------------------------------------------------
    stylix.targets.hyprpaper.enable = lib.mkForce false;

    # Two corrections here, both found on 2026-08-24 by watching the daemon
    # start rather than by reading the config.
    #
    # No `preload` key: hyprpaper 0.8 removed the keyword with the rest of
    # the preload machinery, and a `wallpaper` entry loads its own path.
    # hyprlang ignores the stale key rather than erroring, so the one that
    # used to sit here changed nothing either way.
    #
    # And the wallpaper is an attrset, which home-manager renders as a
    # `wallpaper { }` block. The string form that used to be here rendered
    # a flat `wallpaper=,path` line, which 0.8's config parser rejects with
    # "Monitor <name> has no target: no wp will be created" -- so the eras
    # have been starting a wallpaper daemon that painted nothing. The flat
    # form still works over IPC, which is how it stayed hidden.
    services.hyprpaper = {
      enable = true;
      settings = {
        wallpaper = [ { monitor = "*"; path = "${config.stylix.image}"; } ];
        ipc = false;
        splash = false;
      };
    };

    # services.hyprpaper installs no package of its own; the unit runs a
    # store path. Without this the binary is absent from PATH.
    home.packages = [ pkgs.hyprpaper ];

    # --- bar -----------------------------------------------------------
    programs.waybar = lib.mkIf (!useOwnBar) {
      enable = true;
      systemd.enable = true;
    };

    # home-manager drops its `pkill -USR2 waybar` onChange hooks the moment
    # systemd.enable is set, so without this an era change would repaint the
    # stylesheet under a bar that never rereads it. style.css needs nothing
    # from us -- home-manager already gives it a reload trigger against
    # waybar's SIGUSR2 ExecReload -- but the module list is read only at
    # startup, so config.jsonc has to restart.
    systemd.user.services.waybar.Unit.X-Restart-Triggers = lib.mkIf (!useOwnBar) [
      "${config.xdg.configFile."waybar/config.jsonc".source}"
    ];

    # The native bar has no home-manager module, so its unit is written
    # here. It re-reads theme/current.toml at startup, which is what makes
    # a restart the whole of "wear the new era".
    systemd.user.services.cyberpunk-ui-bar = lib.mkIf useOwnBar {
      Unit = {
        Description = "cyberpunk-ui status bar (${name})";
        PartOf = [ "graphical-session.target" ];
        After = [ "graphical-session.target" ];
        X-Restart-Triggers = [ "${config.xdg.configFile."theme/current.toml".source}" ];
      };
      Service = {
        Type = "simple";
        ExecStart = lib.getExe' cyberpunk-ui "cyberpunk-ui-bar";
        Restart = "on-failure";
        RestartSec = 3;
      };
      Install.WantedBy = [ "graphical-session.target" ];
    };

    # Only written when waybar is actually the bar; otherwise these are
    # two generated files nobody reads, which is the sort of thing that
    # later gets mistaken for the live configuration.
    xdg.configFile."waybar/config.jsonc" = lib.mkIf (!useOwnBar) {
      text = builtins.toJSON {
      layer = "top";
      position = "top";
      height = k.barHeight;
      margin-top = 0;
      margin-bottom = 0;
      margin-left = 0;
      margin-right = 0;
      spacing = 0;

      modules-left = (lib.optional k.hostTape "custom/host") ++ [ "hyprland/workspaces" ];
      modules-center = [ "hyprland/window" ];
      modules-right = [
        "pulseaudio"
        "network"
        "memory"
        "cpu"
        "clock"
      ];

      "custom/host" = {
        exec = "hostname";
        interval = "once";
        format = " {} ";
        tooltip = false;
      };

      "hyprland/workspaces" = {
        format = "{id}";
        on-click = "activate";
        all-outputs = true;
      };

      "hyprland/window" = {
        format = "{title}";
        max-length = 90;
        separate-outputs = true;
      };

      cpu = {
        format = "CPU {usage}%";
        interval = 5;
        states.critical = 90;
      };

      memory = {
        format = "MEM {percentage}%";
        interval = 5;
        states.critical = 90;
      };

      network = {
        format-wifi = "NET {essid}";
        format-ethernet = "NET eth";
        format-disconnected = "NET --";
        tooltip = false;
      };

      pulseaudio = {
        format = "VOL {volume}%";
        format-muted = "VOL --";
        tooltip = false;
      };

      clock = {
        format = "{:%Y-%m-%d %H:%M}";
        tooltip = false;
      };
    };
    };

    # An era owns the bar's look entirely, so stylix's waybar target is
    # turned off rather than overwritten. Same reasoning as the hyprlock
    # target below, and it buys the same thing: with stylix no longer
    # writing a competing stylesheet, this can be a plain definition
    # instead of mkForce, which leaves a host free to override the bar
    # with an ordinary one of its own. mkForce here made that need
    # mkOverride with a number in it.
    stylix.targets.waybar.enable = lib.mkForce false;

    xdg.configFile."waybar/style.css" = lib.mkIf (!useOwnBar) {
      source = (
      builtins.toFile "${lib.toLower name}-waybar.css" ''
        /* ${header} */
        * {
          border: none;
          border-radius: ${toString k.radius}px;
          box-shadow: none;
          text-shadow: none;
          min-height: 0;
          font-family: "${font.name}";
          font-size: 12px;
        }

        window#waybar {
          background: ${c.panel};
          color: ${c.fg};
          border-bottom: 1px solid ${c.border};
        }

        #workspaces,
        #window,
        #cpu,
        #memory,
        #network,
        #pulseaudio,
        #clock {
          padding: 0 10px;
          ${lib.optionalString (k.separator != "") "border-left: ${k.separator};"}
        }

        #custom-host {
          padding: 0 10px;
          color: ${c.bg};
          background: ${c.tape};
        }

        #workspaces button {
          padding: 0 8px;
          border-radius: ${toString k.radius}px;
          color: ${c.dim};
          background: transparent;
        }

        #workspaces button.active {
          ${
            if k.invertActive then
              "color: ${c.bg}; background: ${c.fg};"
            else
              "color: ${c.fg}; background: transparent;"
          }
        }

        #workspaces button.urgent {
          color: ${c.bg};
          background: ${c.alert};
        }

        #window {
          color: ${c.dim};
        }

        #cpu.critical,
        #memory.critical {
          color: ${c.alert};
        }
      ''
    );
    };

    # --- launcher ------------------------------------------------------
    xdg.configFile."rofi/config.rasi".text = ''
      /* ${header} */
      configuration {
        modi: "drun,run";
        show-icons: false;
        display-drun: "run";
        drun-display-format: "{name}";
        me-select-entry: "";
        me-accept-entry: [ MousePrimary ];
      }
      @theme "era"
    '';

    xdg.configFile."rofi/era.rasi".text = ''
      /* ${header} */
      * {
        background-color: transparent;
        text-color:       ${c.fg};
        font:             "${font.name} 12";
      }

      window {
        background-color: ${c.panel};
        border:           1px;
        border-color:     ${c.border};
        border-radius:    ${toString k.radius}px;
        width:            40%;
        padding:          0;
      }

      mainbox {
        children: [ inputbar, listview ];
        padding:  0;
      }

      inputbar {
        background-color: ${c.bg};
        border:           0 0 1px 0;
        border-color:     ${c.border};
        padding:          6px 8px;
        children:         [ prompt, entry ];
      }

      prompt {
        text-color: ${c.dim};
        padding:    0 6px 0 0;
      }

      entry {
        placeholder:       "";
        placeholder-color: ${c.dim};
      }

      listview {
        lines:        12;
        columns:      1;
        scrollbar:    false;
        fixed-height: false;
        padding:      4px 0;
      }

      element {
        padding:       4px 10px;
        border-radius: ${toString k.radius}px;
      }

      element normal.normal {
        text-color: ${c.fg};
      }

      element selected.normal {
        background-color: ${c.border};
        text-color:       ${c.fg};
      }

      element urgent.normal,
      element selected.urgent {
        text-color: ${c.alert};
      }
    '';

    # --- notifications -------------------------------------------------
    # The WantedBy that used to be mkForce'd empty here is restored: the
    # unit is now how swaync starts, rather than a dormant duplicate of an
    # exec-once line.
    stylix.targets.swaync.enable = false;

    services.swaync = {
      enable = true;

      settings = {
        positionX = "right";
        positionY = "top";
        layer = "overlay";
        control-center-layer = "overlay";
        cssPriority = "user";
        control-center-width = 420;
        notification-window-width = 420;
        timeout = 8;
        timeout-low = 4;
        timeout-critical = 0;
        fit-to-screen = true;
        relative-timestamps = true;
      };

      style = ''
        /* ${header} */
        * {
          border-radius: ${toString k.radius}px;
          box-shadow: none;
          text-shadow: none;
          font-family: "${font.name}";
          font-size: 12px;
        }

        .notification-row { background: transparent; }

        .notification {
          background: ${c.panel};
          border: 1px solid ${c.border};
          margin: 4px;
          padding: 0;
        }

        /* The only chromatic escalation. */
        .notification.critical { border: 1px solid ${c.alert}; }

        .notification-content { padding: 8px 10px; }

        .summary { color: ${c.fg}; }
        .body, .time { color: ${c.dim}; }

        .close-button {
          background: transparent;
          color: ${c.dim};
          border: none;
          padding: 0 6px;
        }
        .close-button:hover { color: ${c.alert}; background: transparent; }

        .control-center {
          background: ${c.bg};
          border: 1px solid ${c.border};
        }
        .control-center-list { background: transparent; }

        .widget-title { color: ${c.fg}; padding: 8px 10px; }
        .widget-title > button {
          background: transparent;
          border: 1px solid ${c.border};
          color: ${c.dim};
          padding: 2px 8px;
        }
        .widget-title > button:hover { color: ${c.fg}; }

        .notification-group-headers { color: ${c.dim}; }
      '';
    };

    # --- prompt --------------------------------------------------------
    programs.starship = {
      enable = true;

      settings = {
        scan_timeout = 100;
        add_newline = false;

        format = lib.concatStrings [
          "$username"
          "$hostname"
          "$directory"
          "$git_branch"
          "$git_status"
          "$cmd_duration"
          "$line_break"
          "$character"
        ];

        username = {
          show_always = true;
          style_user = "fg:${c.dim}";
          style_root = "fg:${c.alert}";
          format = "[$user]($style)";
        };

        hostname = {
          ssh_only = false;
          style = "fg:${c.tape}";
          format = "[@$hostname]($style) ";
        };

        directory = {
          style = "fg:${c.fg}";
          format = "[$path]($style)[$read_only]($read_only_style) ";
          read_only = " ro";
          read_only_style = "fg:${c.alert}";
          truncation_length = 4;
          truncate_to_repo = false;
        };

        git_branch = {
          style = "fg:${c.dim}";
          format = "[$branch]($style) ";
          symbol = "";
        };

        git_status = {
          style = "fg:${c.alert}";
          format = "[$all_status$ahead_behind]($style) ";
          conflicted = "!";
          ahead = ">";
          behind = "<";
          diverged = "<>";
          untracked = "?";
          stashed = "$";
          modified = "*";
          staged = "+";
          renamed = "»";
          deleted = "x";
        };

        cmd_duration = {
          min_time = 2000;
          style = "fg:${c.dim}";
          format = "[$duration]($style) ";
        };

        character = {
          success_symbol = "[>](fg:${c.fg})";
          error_symbol = "[>](fg:${c.alert})";
          vimcmd_symbol = "[<](fg:${c.dim})";
        };

        package.disabled = true;
        nodejs.disabled = true;
        rust.disabled = true;
        python.disabled = true;
        golang.disabled = true;
        java.disabled = true;
        docker_context.disabled = true;
        nix_shell = {
          disabled = false;
          style = "fg:${c.dim}";
          format = "[nix]($style) ";
          symbol = "";
        };
      };
    };

    # --- multiplexer ---------------------------------------------------
    #
    # home/common/cli/tmux.nix ships a powerline status bar by default:
    # chevron glyphs between segments, centred window list, a green
    # arrow on the active window. That is precisely the ornament these
    # eras reject, so replace it rather than recolour it.
    #
    # Its default also shells out to `curl icanhazip.com` on every
    # status refresh; this one does not, because a status bar should not
    # depend on a third party being up.
    programs.tmux.statusBarExtraConfig = ''
      # ${header}
      set -g status-justify left
      set -g status-style "bg=${c.panel},fg=${c.fg}"

      # Window list: plain labels, active inverted rather than lit.
      set-window-option -g window-status-separator ""
      set-window-option -g window-status-format "#[fg=${c.dim},bg=${c.panel}] #I:#W "
      set-window-option -g window-status-current-format \
        "#[fg=${if k.invertActive then c.bg else c.fg},bg=${
          if k.invertActive then c.fg else c.panel
        }] #I:#W "

      # The host, in the same tape colour the bar uses for it.
      set -g status-left-length 40
      set -g status-left "#[bg=${c.tape},fg=${c.bg}] #h "

      set -g status-right-length 40
      set -g status-right "#[fg=${c.dim}] #S  %Y-%m-%d %H:%M "

      # Borders are the same 1px hairline as everywhere else.
      set -g pane-border-style "fg=${c.border}"
      set -g pane-active-border-style "fg=${c.fg}"
      set -g message-style "bg=${c.panel},fg=${c.fg}"
      set -g mode-style "bg=${c.border},fg=${c.fg}"
    '';

    # --- browser -------------------------------------------------------
    programs.firefox = {
      enable = true;

      profiles.default = {
        id = 0;

        settings = {
          "toolkit.legacyUserProfileCustomizations.stylesheets" = true;
          "browser.startup.homepage" = "https://github.com";
          "signon.rememberSignons" = false;
          "browser.uidensity" = 1;
          "toolkit.cosmeticAnimations.enabled" = false;
        };

        userChrome = ''
          /* ${header} */
          :root {
            --era-bg: ${c.bg};
            --era-panel: ${c.panel};
            --era-border: ${c.border};
            --era-dim: ${c.dim};
            --era-fg: ${c.fg};
            --era-alert: ${c.alert};

            --toolbar-bgcolor: var(--era-panel) !important;
            --toolbar-color: var(--era-fg) !important;
            --tab-border-radius: ${toString k.radius}px !important;
            --toolbarbutton-border-radius: ${toString k.radius}px !important;
            --urlbar-min-height: 26px !important;
          }

          #navigator-toolbox {
            background: var(--era-panel) !important;
            border-bottom: 1px solid var(--era-border) !important;
          }

          #TabsToolbar, #nav-bar, #PersonalToolbar {
            background: var(--era-panel) !important;
            border: none !important;
            box-shadow: none !important;
          }

          .tabbrowser-tab .tab-background {
            border-radius: ${toString k.radius}px !important;
            border: none !important;
            box-shadow: none !important;
            background: transparent !important;
          }

          .tabbrowser-tab[selected] .tab-background {
            background: var(--era-${if k.invertActive then "fg" else "panel"}) !important;
          }

          .tabbrowser-tab[selected] .tab-label {
            color: var(--era-${if k.invertActive then "bg" else "fg"}) !important;
          }

          .tabbrowser-tab:not([selected]) .tab-label {
            color: var(--era-dim) !important;
          }

          #urlbar, #urlbar-background, #searchbar {
            border-radius: ${toString k.radius}px !important;
            box-shadow: none !important;
            background: var(--era-bg) !important;
            border: 1px solid var(--era-border) !important;
          }

          #urlbar[focused] > #urlbar-background {
            border-color: var(--era-fg) !important;
          }

          #urlbar-input, #searchbar .searchbar-textbox {
            color: var(--era-fg) !important;
          }

          .urlbarView {
            background: var(--era-panel) !important;
            border: 1px solid var(--era-border) !important;
          }

          .urlbarView-row[selected], .urlbarView-row:hover {
            background: var(--era-border) !important;
            border-radius: ${toString k.radius}px !important;
          }

          menupopup, panel {
            --panel-background: var(--era-panel) !important;
            --panel-color: var(--era-fg) !important;
            --panel-border-color: var(--era-border) !important;
            --panel-border-radius: ${toString k.radius}px !important;
          }

          toolbarbutton .toolbarbutton-icon {
            border-radius: ${toString k.radius}px !important;
            box-shadow: none !important;
          }

          #identity-box.notSecure #identity-icon {
            color: var(--era-alert) !important;
          }
        '';
      };
    };
  }

  # --- lock screen -----------------------------------------------------
  #
  # Generated from the same roles, in the same house style: square,
  # 1px, unlit, no blur. Every value is mkDefault, so a host overrides
  # any single setting with a plain definition and does not need to
  # restate the rest - which matters because a work machine may have
  # lock-screen requirements the theme knows nothing about.
  #
  # stylix's hyprlock target is force-disabled, which is what frees
  # these to be mkDefault rather than mkForce.
  #
  # NOTE: this generates hyprlock's *configuration* only. hyprlock also
  # needs a PAM service, which is a system-level option a home-manager
  # module cannot set -- see security.pam.services.hyprlock in
  # system/wm/hyprland.nix. Without it the lock takes the session and
  # then exits, which looks exactly like a working lock screen right up
  # until you try to unlock it.
  (lib.mkIf lk.enable {
    stylix.targets.hyprlock.enable = lib.mkForce false;

    programs.hyprlock = {
      enable = lib.mkDefault true;

      settings = {
        general.hide_cursor = lib.mkDefault true;

        background = {
          monitor = lib.mkDefault "";
          path = lib.mkDefault "${config.stylix.image}";
          color = lib.mkDefault (rgba "bg");
          # No blur, no vibrancy: the wallpaper is already a flat field
          # and softening it would be ornament.
          blur_passes = lib.mkDefault 0;
          noise = lib.mkDefault 0;
        };

        input-field = {
          monitor = lib.mkDefault "";
          size = lib.mkDefault "280, 44";
          rounding = lib.mkDefault k.radius;
          outline_thickness = lib.mkDefault 1;
          shadow_passes = lib.mkDefault 0;
          dots_center = lib.mkDefault true;
          dots_size = lib.mkDefault 0.2;
          fade_on_empty = lib.mkDefault false;
          placeholder_text = lib.mkDefault "";
          outer_color = lib.mkDefault (rgba "border");
          inner_color = lib.mkDefault (rgba "panel");
          font_color = lib.mkDefault (rgba "fg");
          check_color = lib.mkDefault (rgba "dim");
          fail_color = lib.mkDefault (rgba "alert");
          fail_text = lib.mkDefault "$FAIL ($ATTEMPTS)";
          position = lib.mkDefault "0, -80";
          halign = lib.mkDefault "center";
          valign = lib.mkDefault "center";
        };

        label = lib.mkDefault [
          {
            monitor = "";
            text = "cmd[update:1000] date +%H:%M";
            color = (rgba "fg");
            font_size = 64;
            font_family = font.name;
            position = "0, 40";
            halign = "center";
            valign = "center";
            shadow_passes = 0;
          }
          {
            monitor = "";
            text = "cmd[update:60000] date +%Y-%m-%d";
            color = (rgba "dim");
            font_size = 12;
            font_family = font.name;
            position = "0, -8";
            halign = "center";
            valign = "center";
            shadow_passes = 0;
          }
          # The host, in the tape accent the bar and prompt also use
          # for it.
          {
            monitor = "";
            text = "cmd[update:3600000] uname -n";
            color = (rgba "tape");
            font_size = 12;
            font_family = font.name;
            position = "20, -20";
            halign = "left";
            valign = "top";
            shadow_passes = 0;
          }
        ];
      };
    };
  })

  (lib.mkIf (lk.enable && lk.bind != "") {
    wayland.windowManager.hyprland.settings.bind = [
      "${lk.bind}, exec, hyprlock"
    ];
  })

  (lib.mkIf browserRestart (import ./browser-restart.nix {
    inherit lib pkgs config name;
    stamp = themeStamp;
  }))
]
