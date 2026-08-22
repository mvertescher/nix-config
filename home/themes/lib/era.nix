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
  # Per-era styling knobs. The defaults are the shared "hard edges"
  # house style; an ornamental era overrides them.
  knobs ? { },
}:

let
  c = roles;

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
    ];
    fonts.fontconfig.enable = true;

    stylix = {
      enable = true;
      image = lib.mkDefault wallpaper;
      fonts.sansSerif = lib.mkDefault { inherit (font) package name; };
    };

    # --- window manager ------------------------------------------------
    wayland.windowManager.hyprland.settings = {
      exec-once = [
        "waybar"
        "swaync"
        "hyprpaper"
      ];

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

    services.hyprpaper = {
      enable = true;
      settings = {
        preload = [ "${config.stylix.image}" ];
        wallpaper = [ ",${config.stylix.image}" ];
        ipc = false;
        splash = false;
      };
    };

    # --- bar -----------------------------------------------------------
    programs.waybar.enable = true;

    xdg.configFile."waybar/config.jsonc".text = builtins.toJSON {
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

    # stylix writes its own waybar stylesheet; an era owns the look
    # entirely, so replace it rather than layering over it.
    xdg.configFile."waybar/style.css".source = lib.mkForce (
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
    stylix.targets.swaync.enable = false;
    systemd.user.services.swaync.Install.WantedBy = lib.mkForce [ ];

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

  (lib.mkIf browserRestart {
    home.activation.eraBrowserRestart = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
      eraStampDir="${config.xdg.stateHome}/themes"
      eraStamp="$eraStampDir/browser-theme"
      eraWant="${themeStamp}"
      eraHave="$(${coreutil "cat"} "$eraStamp" 2>/dev/null || true)"

      if [ "$eraHave" = "$eraWant" ]; then
        verboseEcho "${name}: browser theme unchanged, leaving Firefox alone"
      else
        # The kernel truncates comm to 15 characters, so the wrapped
        # binary is ".firefox-wrappe" and pkill -x misses it; match the
        # launcher's command line instead.
        eraPids="$(${bin pkgs.procps "pgrep"} -f 'bin/firefox$' 2>/dev/null || true)"

        if [ -n "$eraPids" ]; then
          verboseEcho "${name}: browser theme changed, restarting Firefox"

          # SIGTERM, so Firefox writes its session store and restores
          # tabs on the way back up.
          $DRY_RUN_CMD kill -TERM $eraPids 2>/dev/null || true

          eraWaited=0
          while [ "$eraWaited" -lt 20 ] \
            && ${bin pkgs.procps "pgrep"} -f 'bin/firefox$' >/dev/null 2>&1; do
            ${coreutil "sleep"} 0.5
            eraWaited=$((eraWaited + 1))
          done

          # home-manager's unit runs with an empty Environment=, so the
          # compositor signature is read off the runtime directory
          # rather than inherited.
          eraSig="$(
            ${coreutil "ls"} "/run/user/$(${coreutil "id"} -u)/hypr" 2>/dev/null \
              | ${coreutil "head"} -1 || true
          )"

          if [ -n "$eraSig" ]; then
            $DRY_RUN_CMD ${coreutil "env"} "HYPRLAND_INSTANCE_SIGNATURE=$eraSig" \
              ${hyprctl} dispatch exec firefox >/dev/null 2>&1 || true
          else
            verboseEcho "${name}: no hyprland session, not relaunching Firefox"
          fi
        else
          verboseEcho "${name}: Firefox not running, nothing to restart"
        fi

        $DRY_RUN_CMD ${coreutil "mkdir"} -p "$eraStampDir"
        $DRY_RUN_CMD sh -c "printf '%s' '$eraWant' > '$eraStamp'"
      fi
    '';
  })
]
