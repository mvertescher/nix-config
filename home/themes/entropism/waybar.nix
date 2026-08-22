# entropism bar: a status line, not a dashboard.
#
# Flat panel, square corners, 1px top border, modules separated by a
# literal "|". No icons, no pills, no arrows -- the cybr bar's slanted
# SVG separators are exactly the ornament this theme rejects.
{
  config,
  lib,
  ...
}:

let
  cfg = config.themes.entropism;
  c = cfg.resolvedColors;
in
{
  config = lib.mkIf cfg.enable {
    wayland.windowManager.hyprland.settings.exec-once = [ "waybar" ];

    programs.waybar.enable = true;

    xdg.configFile."waybar/config.jsonc".text = builtins.toJSON {
      layer = "top";
      position = "top";
      height = 22;
      margin-top = 0;
      margin-bottom = 0;
      margin-left = 0;
      margin-right = 0;
      spacing = 0;

      modules-left = [
        "custom/host"
        "hyprland/workspaces"
      ];
      modules-center = [ "hyprland/window" ];
      modules-right = [
        "pulseaudio"
        "network"
        "memory"
        "cpu"
        "clock"
      ];

      "custom/host" = {
        # The one place the tape accent is allowed: a functional label,
        # the equivalent of a name written on a machine in marker.
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

    # stylix writes its own waybar stylesheet; this theme owns the look
    # entirely, so replace it rather than layering on top.
    xdg.configFile."waybar/style.css".source = lib.mkForce (
      builtins.toFile "entropism-waybar.css" ''
        /* Generated from themes/entropism roles. No literals here. */
        * {
          border: none;
          border-radius: 0;
          box-shadow: none;
          text-shadow: none;
          min-height: 0;
          font-family: "${cfg.uiFont.name}";
          font-size: 12px;
        }

        window#waybar {
          background: ${c.panel};
          color: ${c.fg};
          border-bottom: 1px solid ${c.border};
        }

        /* One separator style, applied uniformly: a dim pipe. */
        #workspaces,
        #window,
        #cpu,
        #memory,
        #network,
        #pulseaudio,
        #clock {
          padding: 0 10px;
          border-left: 1px solid ${c.border};
        }

        #custom-host {
          padding: 0 10px;
          color: ${c.bg};
          background: ${c.tape};
        }

        #workspaces button {
          padding: 0 8px;
          border-radius: 0;
          color: ${c.dim};
          background: transparent;
        }

        /* Active workspace is inverted rather than highlighted -- the
           cheapest possible emphasis. */
        #workspaces button.active {
          color: ${c.bg};
          background: ${c.fg};
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
}
