{ pkgs, lib, config, ... }:

let
  c = config.lib.stylix.colors;
  inherit ((import ../../lib/shades.nix { inherit lib; }).forColors c) mid dark;
in

{
  stylix.targets.swaync.enable = false;

  # Launched by its systemd user unit rather than hyprland's exec-once.
  # See ../lib/era.nix for the reasoning: exec-once only fires when the
  # compositor starts, so a theme switch on a live session left the
  # desktop with no notification daemon until the next logout. The unit's
  # X-Restart-Triggers, which home-manager wires to swaync's config and
  # style, restart it when this theme's colours change.

  services.swaync = {
    enable = true;
    settings = {
      "$schema" = "/etc/xdg/swaync/configSchema.json";
      ignore-gtk-theme = true;
      positionX = "right";
      positionY = "bottom";
      layer = "overlay";
      control-center-layer = "overlay";
      layer-shell = true;
      cssPriority = "user";
      layer-shell-cover-screen = true;
      control-center-margin-top = 0;
      control-center-margin-bottom = 0;
      control-center-margin-right = 0;
      control-center-margin-left = 0;
      timeout = 0;
      timeout-low = 0;
      timeout-critical = 0;
      fit-to-screen = true;
      relative-timestamps = true;
      control-center-width = 500;
      control-center-height = 600;
      notification-window-width = 500;
      keyboard-shortcuts = true;
      image-visibility = "when-available";
      transition-time = 200;
      hide-on-clear = true;
      hide-on-action = true;
      text-empty = "No Notifications";
      script-fail-notify = true;
      widgets = [
        "title"
        "dnd"
        "notifications"
        "mpris"
      ];
      widget-config = {
        title = {
          text = "Notifications";
          clear-all-button = true;
          button-text = "Clear All";
        };
        dnd = {
          text = "Mute";
        };
        label = {
          max-lines = 1;
          text = "Notification Center";
        };
        mpris = {
          show-album-art = "always";
          loop-carousel = true;
        };
      };
    };
    style = ''
      /*
      # ---------------------------------------
      # cybr-swaync    lucid theme for swaync
      # Project:       https://github.com/cybrcore/cybr-swaync
      # Author:        scherrer-txt   |   License:     GPL-3.0
      # Source:        ~/.config/swaync/style.css
      # ---------------------------------------
      */

      /* === VARIABLES === */
      /* VALUES */
      :root {
              --radius: 0px;
              --gap: 8px;
              --gap2: calc(var(--gap)*2);
              --border-size: 1px;
      }

      /* Colors - generated from the active Stylix palette. These used to
         be literals, and some of them (gr0, ye0, cy0) did not even match
         cybr's own palette, so the notification centre drifted from the
         rest of the desktop. */
      @define-color no0 #${c.base00};

      @define-color re0 #${c.base08};
      @define-color re1 #${mid "base08"};
      @define-color re2 #${dark "base08"};

      @define-color gr0 #${c.base0B};
      @define-color gr2 #${dark "base0B"};
      @define-color gr0tr #${c.base0B}40;

      @define-color ye0 #${c.base0A};
      @define-color ye2 #${dark "base0A"};

      @define-color bl0 #${c.base0D};
      @define-color bl2 #${dark "base0D"};

      @define-color cy0 #${c.base0C};
      @define-color cy2 #${dark "base0C"};
      @define-color wh0 #${c.base05};

      @define-color tr0 rgba(${c."base00-rgb-r"}, ${c."base00-rgb-g"}, ${c."base00-rgb-b"}, 0.05);


      /* === GLOBAL === */
      * {
              all: unset;
              font-size: 12px;
              font-family: "${config.stylix.fonts.monospace.name}";
              transition: 0;
      }

      /* === DND ===  */
      .widget-dnd label,
      .widget-label>label {
              color: @re0;
      }

      .widget-dnd switch {
              margin: var(--gap);
              border-radius: var(--radius);
              min-width: 35px;
              min-height: 15px;
              background: @re2;
              color: #${c.base0B}FF;
      }

      .widget-dnd switch slider {
              min-width: 13px;
              min-height: 13px;
              margin: 2px;
              background: @re0;
      }

      .widget-dnd switch:checked {
              background: @gr0;
      }

      .widget-dnd switch:checked slider {
              background: @gr2;
      }

      /* === MEDIA WIDGET === */
      .widget-mpris {
              color: @cy0;
              background-color: @tr0;
              border: 1px solid @re0;
              margin-top: var(--gap2);
      }

      .widget-mpris-title {
              font-weight: normal;
              font-size: 14px;
      }

      .widget-mpris-subtitle {
              font-size: 12px;
      }

      .widget-mpris .widget-mpris-player .mpris-background {
              filter: blur(7px) grayscale(1) brightness(0.3);
              opacity: 0.3;
      }

      .widget-mpris .widget-mpris-player .mpris-overlay {
              padding: 40px;
      }

      .widget-mpris .widget-mpris-player .mpris-overlay .widget-mpris-album-art {
              border-radius: var(--radius);
              -gtk-icon-size: 100px;
      }

      .widget.widget-mpris>carouselindicatordots {
              --dots-padding: var(--gap);
              padding: var(--gap);
              margin: 0;
              color: @re0;
      }

      .widget-mpris-player button {
              margin: var(--gap);
              -gtk-icon-size: 20px;
      }

      /* ----------------- */

      /*  === CONTROL CENTER === */
      /* Main panel */
      .control-center {
              margin: var(--gap);
              padding: var(--gap2);
              background-color: @tr0;
              border: 1px solid @re0;
              color: @re0;
      }

      /* Titlebar */
      .control-center .widget-title {
              color: @re0;
      }

      /* Gap between notifications */
      .control-center .notification-row .notification-background {
              margin-top: var(--gap2);
      }

      .control-center .notification-row .notification-background {
              background-color: @no0;
      }

      /* Clear all button */
      .control-center .widget-title button {
              padding: var(--gap);
              border-radius: var(--radius);
              color: @cy0;
              background-color: @cy2;
      }

      .control-center .widget-title button:hover {
              background-color: @bl2;
              color: @bl0;
      }

      /* Spacing */
      .control-center .notification-row .notification-background .notification .notification-content {
              margin: var(--gap2);
      }

      /* === FLOATING NOTIFICATIONS === */

      /* Basic */
      .floating-notifications.background .notification-row .notification-background {
              margin: var(--gap);
              border-radius: var(--radius);
              background-color: @tr0;
              color: @re0;
      }

      .floating-notifications.background .notification-row .notification-background .notification {
              padding: var(--gap2);
      }

      /* Images */
      .image {
              margin: var(--gap);
              margin-right: var(--gap2);
              border-radius: var(--radius);
              /* background-color: @no0; */
              -gtk-icon-size: 30px;
      }

      /* Normal */

      .notification-row .notification-background .notification.normal .notification-content .summary {
              color: @re0;
              font-weight: bold;
              text-transform: uppercase;
      }

      .notification-row .notification-background .notification.normal .notification-content .time {
              color: @wh0;
      }

      .floating-notifications.background .notification-row .notification-background .notification.normal {
              background-color: @tr0;
      }

      .notification-row .notification-background .notification.normal .notification-content .body {
              color: @re0;
      }

      .notification.normal {
              border: 1px solid @re0;
      }

      /* Low */

      .notification-row .notification-background .notification.low .notification-content .summary {
              color: @gr0;
              font-size: 14px;
      }

      .notification-row .notification-background .notification.low .notification-content .time {
              color: @gr0;
      }

      .floating-notifications.background .notification-row .notification-background .notification.low {
              background-color: @gr0tr;
      }

      .notification-row .notification-background .notification.low .notification-content .body {
              color: @gr0;
      }

      .notification.low {
              border: 1px solid @gr0;
      }

      /* Critical */

      .notification-row .notification-background .notification.critical .notification-content .summary {
              color: @no0;
              font-size: 14px;
      }

      .notification-row .notification-background .notification.critical .notification-content .time {
              color: @no0;
      }

      .notification-row .notification-background .notification.critical {
              background-color: @re0;
      }

      .notification-row .notification-background .notification.critical .notification-content .body {
              color: @no0;
      }

      .notification.critical {
              border: 1px solid @re0;
      }

      /*  === BUTTONS === */

      /* Basic */

      button {
              margin: var(--gap);
              padding: 2px;
              border-radius: var(--radius);
              background-color: @cy2;
              color: @cy0;
              border: none;
      }

      button:hover {
              background-color: @bl2;
              color: @bl0;
      }

      button:active {
              background-color: @re2;
              color: @no0;
      }

      /* Actions */
      .notification-row .notification-background .notification>*:last-child>* .notification-action {
              margin: var(--gap);
              border-radius: var(--radius);
              background-color: @no0;
              color: @re0;
      }

      .notification-row .notification-background .notification>*:last-child>* .notification-action:active {
              box-shadow: inset 0 0 0 1px @gr1;
      }

      /* Close button */
      .close-button {
              background-color: @cy2;
              color: @cy0;
      }

      .close-button:hover {
              background-color: @bl2;
              color: @bl0;
      }
    '';
  };

  wayland.windowManager.hyprland.settings = {
    bind = [
      "SUPER, N, exec, swaync-client -t"
    ];
  };
}
