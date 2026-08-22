# entropism notifications: square, flat, and quiet.
#
# Panel-coloured card, 1px border, alert border only when something has
# actually failed. Matches the cybr module's shape (stylix target off,
# service not auto-wanted, launched from hyprland) so hosts behave the
# same whichever theme they pick.
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
    stylix.targets.swaync.enable = false;
    systemd.user.services.swaync.Install.WantedBy = lib.mkForce [ ];

    wayland.windowManager.hyprland.settings.exec-once = [ "swaync" ];

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
        /* Generated from themes/entropism roles. No literals here. */
        * {
          border-radius: 0;
          box-shadow: none;
          text-shadow: none;
          font-family: "${cfg.uiFont.name}";
          font-size: 12px;
        }

        .notification-row {
          background: transparent;
        }

        .notification {
          background: ${c.panel};
          border: 1px solid ${c.border};
          margin: 4px;
          padding: 0;
        }

        /* The only chromatic escalation in the whole theme. */
        .notification.critical {
          border: 1px solid ${c.alert};
        }

        .notification-content {
          padding: 8px 10px;
        }

        .summary {
          color: ${c.fg};
        }

        .body,
        .time {
          color: ${c.dim};
        }

        .close-button {
          background: transparent;
          color: ${c.dim};
          border: none;
          padding: 0 6px;
        }

        .close-button:hover {
          color: ${c.alert};
          background: transparent;
        }

        .control-center {
          background: ${c.bg};
          border: 1px solid ${c.border};
        }

        .control-center-list {
          background: transparent;
        }

        .widget-title {
          color: ${c.fg};
          padding: 8px 10px;
        }

        .widget-title > button {
          background: transparent;
          border: 1px solid ${c.border};
          color: ${c.dim};
          padding: 2px 8px;
        }

        .widget-title > button:hover {
          color: ${c.fg};
        }

        .notification-group-headers {
          color: ${c.dim};
        }
      '';
    };
  };
}
