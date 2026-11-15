# The Neomil desktop, as designed in ./design (approved 2026-11-14 with
# the reference header, dark windows and corners cut on all four).
#
# Every size below is written in the design's units and multiplied once
# by `s`, so the TV fitting is this file at another scale plus the three
# adjustments named in `fit` -- not a second set of numbers. Inks are the
# measured ones (home/common/pkgs/cp-eras-ui/docs/neomil/line-weights.md,
# and INK in design/neomil-design.js); they are literals here because the
# seven shared roles do not carry them.
#
# What this cannot draw, and so differs from the design:
#   - Hyprland's border is one width for focused and unfocused windows,
#     so the TV's heavier focus outline is not available; focus is ink
#     only.
#   - The printed echo is a hard offset shadow: a faint band below and
#     right of the focused window, not a second thin line.
#   - Hyprland's animations are curves, not steps; they are linear and
#     short instead.
#   - rofi, swaync and hyprlock have no cut corners. Their plates are
#     square until our own programs replace them.
{
  lib,
  config,
  target,
  unit,
  uiFont,
}:
let
  fit =
    {
      desktop = { scale = 1.0; line = 1.0; margin = 40; gapOut = 10; gapIn = 6; };
      tv = { scale = 1.5; line = 1.5; margin = 80; gapOut = 16; gapIn = 10; };
    }
    .${target};

  s = unit * fit.scale;
  # Design units to whole pixels. Gaps and margins do not take the 1.5.
  px = dp: builtins.floor (dp * s + 0.5);
  flat = dp: builtins.floor (dp * unit + 0.5);
  # The era's one outline: 0.8333 units, 2 px on the desktop at 4K.
  line = lib.max 1 (builtins.floor (0.8333 * unit * fit.line + 0.5));
  p = dp: "${toString (px dp)}px";

  ink = {
    bg = "#050304";
    fg = "#ef3333";
    bright = "#fb3535";
    border = "#5e1112";
    dim = "#a32226";
    rest = "#2e1012";
    ghost = "#1a0607";
    field = "#760e0f";
    panel = "#12080a";
    card = "#3a1218";
    tape = "#dedede";
    onRed = "#19090a";
  };
  hex = c: lib.removePrefix "#" c;
  rgba = c: a: "rgba(${hex c}${a})";

  ui = uiFont.name;
  wide = "Orbitron";
  o = lib.mkOverride 30; # this file refines era.nix within the one theme, so it sits above era.nix's plain definitions
in
{
  toml = ''
    # Native px per design unit. The TV's heavier lines come with the
    # scale (1.5 x 0.8333 units), so the extra line multiplier stays 1
    # (home/themes/neomil/desktop.nix, target "${target}").
    ui_scale = ${toString s}
    line_scale = 1.0
  '';

  swaync = ''
      /* home/themes/neomil/desktop.nix (design/launcher.html) */
      * { font-size: ${p 8.5}; }
      .notification {
        background: ${ink.panel};
        border: ${toString line}px solid ${ink.bright};
        border-left: ${p 4} solid ${ink.fg};
        margin: ${p 6};
      }
      .notification-content { padding: ${p 8} ${p 12}; }
      .summary { color: ${ink.fg}; font-size: ${p 11}; font-weight: 700; }
      .body { color: ${ink.tape}; }
      .time { color: ${ink.dim}; }
  '';

  config = {
    wayland.windowManager.hyprland.settings = {
      general = {
        border_size = o line;
        gaps_out = o (flat fit.gapOut);
        # Hyprland puts gaps_in on each side of a window.
        gaps_in = o (flat fit.gapIn / 2);
        "col.active_border" = o (rgba ink.bright "ff");
        "col.inactive_border" = o (rgba ink.border "ff");
      };
      decoration = {
        # A straight cut, the panels' 8 units, on all four corners.
        rounding = o (px 8);
        rounding_power = o 1.0;
        shadow = {
          enabled = o true;
          sharp = o true;
          range = o 1;
          scale = o 1.0;
          offset = o "${toString (px 2.5)} ${toString (px 2.5)}";
          color = o (rgba ink.bright "26");
          color_inactive = o "rgba(00000000)";
        };
      };
      animations = {
        enabled = o true;
        bezier = o [ "hard, 0, 0, 1, 1" ];
        animation = o [
          "windows, 1, 2, hard, popin 92%"
          "windowsOut, 1, 1, hard, popin 96%"
          "border, 1, 1, hard"
          "fade, 1, 1, hard"
          "layers, 1, 2, hard, slide"
          "workspaces, 1, 2, hard, slidefade 8%"
        ];
      };
    };

    # --- terminal -------------------------------------------------------
    # The design's mono text is 7 units; at the TV's scale stylix's 12 pt
    # is unreadable from the couch. The padding keeps the first glyph out
    # of the window's cut corner (seen on terra 2026-11-14: the prompt's
    # first letter was clipped).
    programs.alacritty.settings = {
      font.size = o (px 7 * 0.75);
      window.padding = {
        x = px 6;
        y = px 5;
      };
    };

    # --- launcher: the GO HOME panel, the password field, menu rows ----
    xdg.configFile."rofi/era.rasi".text = lib.mkForce ''
      /* Generated from home/themes/neomil/desktop.nix (design/launcher.html). */
      * {
        background-color: transparent;
        text-color:       ${ink.fg};
        font:             "${ui} Medium ${p 12}";
      }

      window {
        background-color: ${ink.panel}f5;
        border:           ${toString line}px;
        border-color:     ${ink.bright};
        border-radius:    0;
        width:            ${p 380};
        padding:          ${p 12} ${p 20} ${p 16} ${p 14};
      }

      mainbox {
        children: [ textbox-title, inputbar, listview ];
        spacing:  ${p 10};
      }

      textbox-title {
        expand:  false;
        content: "RUN";
        font:    "${ui} Bold ${p 17}";
      }

      inputbar {
        background-color: ${ink.ghost};
        border:           ${toString line}px;
        border-color:     ${ink.field};
        padding:          ${p 6} ${p 8};
        children:         [ entry ];
      }

      entry {
        text-color:        ${ink.tape};
        cursor-color:      ${ink.bright};
        cursor-width:      ${p 1.5};
        placeholder:       "";
      }

      listview {
        lines:        7;
        columns:      1;
        spacing:      ${p 4};
        scrollbar:    false;
        fixed-height: false;
      }

      element {
        padding:          ${p 7} ${p 14};
        background-color: ${ink.rest};
        border:           ${toString line}px;
        border-color:     ${ink.border};
        border-radius:    0;
      }

      element-text {
        background-color: transparent;
        text-color:       inherit;
        text-transform:   uppercase;
      }

      element normal.normal,
      element alternate.normal {
        text-color: ${ink.fg};
      }

      element selected.normal {
        background-color: ${ink.fg};
        border-color:     ${ink.bright};
        text-color:       ${ink.onRed};
      }

      element urgent.normal,
      element selected.urgent {
        text-color: ${ink.bright};
      }
    '';

    # --- notifications: a small bright panel with a red spine ----------
    services.swaync.settings = {
      notification-window-width = o (px 250);
      control-center-width = o (px 250);
    };

    # --- lock: the login card, field and ENTER (design/lock.html) ------
    # Positions are from the screen centre, y upwards.
    programs.hyprlock.settings = {
      shape = [
        # the user card
        {
          monitor = "";
          size = "${toString (px 190)}, ${toString (px 250)}";
          color = rgba ink.card "d9";
          rounding = 0;
          border_size = line;
          border_color = rgba ink.bright "ff";
          position = "0, ${toString (px 50)}";
          halign = "center";
          valign = "center";
          zindex = 0;
        }
        # the portrait block and its figure
        {
          monitor = "";
          size = "${toString (px 84)}, ${toString (px 84)}";
          color = rgba ink.fg "ff";
          rounding = 0;
          position = "0, ${toString (px 75)}";
          halign = "center";
          valign = "center";
          zindex = 1;
        }
        {
          monitor = "";
          size = "${toString (px 28)}, ${toString (px 28)}";
          color = rgba ink.onRed "ff";
          rounding = -1;
          position = "0, ${toString (px 84)}";
          halign = "center";
          valign = "center";
          zindex = 2;
        }
        {
          monitor = "";
          size = "${toString (px 50)}, ${toString (px 22)}";
          color = rgba ink.onRed "ff";
          rounding = px 11;
          position = "0, ${toString (px 44)}";
          halign = "center";
          valign = "center";
          zindex = 2;
        }
        # ENTER
        {
          monitor = "";
          size = "${toString (px 64)}, ${toString (px 26)}";
          color = rgba ink.fg "ff";
          rounding = 0;
          border_size = line;
          border_color = rgba ink.bright "ff";
          position = "${toString (px 93)}, ${toString (px (-110))}";
          halign = "center";
          valign = "center";
          zindex = 1;
        }
      ];

      input-field = {
        size = "${toString (px 180)}, ${toString (px 26)}";
        rounding = 0;
        outline_thickness = line;
        outer_color = rgba ink.field "ff";
        inner_color = rgba ink.ghost "ff";
        font_color = rgba ink.tape "ff";
        check_color = rgba ink.dim "ff";
        fail_color = rgba ink.bright "ff";
        dots_center = false;
        position = "${toString (px (-35))}, ${toString (px (-110))}";
        zindex = 1;
      };

      label = [
        {
          monitor = "";
          text = "cmd[update:3600000] whoami | tr a-z A-Z";
          color = rgba ink.fg "ff";
          font_size = px 13;
          font_family = "${ui} Medium";
          position = "0, ${toString (px (-42))}";
          halign = "center";
          valign = "center";
          zindex = 2;
        }
        {
          monitor = "";
          text = "ENTER";
          color = rgba ink.onRed "ff";
          font_size = px 8;
          font_family = "${ui} Bold";
          position = "${toString (px 93)}, ${toString (px (-110))}";
          halign = "center";
          valign = "center";
          zindex = 2;
        }
        {
          monitor = "";
          text = "cmd[update:1000] date +%H:%M";
          color = rgba ink.fg "ff";
          font_size = px 48;
          font_family = "${wide} SemiBold";
          position = "${toString (flat fit.margin + px 60)}, ${toString (px 10)}";
          halign = "left";
          valign = "center";
        }
        {
          monitor = "";
          text = "cmd[update:60000] date '+%A %-d %B' | tr a-z A-Z";
          color = rgba ink.tape "d9";
          font_size = px 7;
          font_family = "${ui} Medium";
          position = "${toString (flat fit.margin + px 62)}, ${toString (px (-36))}";
          halign = "left";
          valign = "center";
        }
      ];
    };
  };
}
