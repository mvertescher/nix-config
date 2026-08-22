# entropism -- the salvaged-hardware theme.
#
# Where cybr is an aggressive Neomilitarism look, entropism is the era
# before it: necessity over style. Degraded monochrome displays, zero
# ornament, 1px lines, square corners, no glow.
#
# Unlike cybr, whose palette is a fixed base16 file, this theme is
# configurable. It exposes semantic *roles* rather than base16 slots, so
# a wrapper can retint the whole desktop by naming one colour:
#
#   imports = [ nix-config/home/themes/entropism ];
#   themes.entropism = {
#     enable = true;
#     variant = "dead-pixel";
#     colors.fg = "#c8d0c4";
#   };
#
# Every downstream module reads the resolved roles, never a literal, so
# that single override reaches stylix, waybar, rofi, swaync and the
# hyprland borders together.
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.themes.entropism;

  palettes = import ./palettes.nix;

  roles = [
    "bg"
    "panel"
    "border"
    "dim"
    "fg"
    "alert"
    "tape"
  ];

  # variant palette < per-role overrides. A null override means "keep the
  # variant's value", so a host can set one role without restating six.
  overrides = lib.filterAttrs (_: v: v != null) cfg.colors;
  base = palettes.${cfg.variant} // overrides;

  # `tape` is the only optional role: presets that want it to track the
  # foreground leave it out entirely, so an override of `fg` carries the
  # label accent with it instead of stranding it on the old colour.
  resolved = base // { tape = base.tape or base.fg; };

  # Bare six-digit hex, so callers can append an alpha suffix.
  hex = role: lib.removePrefix "#" resolved.${role};

  # home/common/hyprland pins its colours and rounding with mkForce so
  # they beat stylix. A second mkForce here would be a same-priority
  # collision, not an override, so the theme sits one step stronger.
  entropismOverride = lib.mkOverride 40;
in
{
  imports = [
    ./rofi.nix
    ./swaync.nix
    ./waybar.nix
  ];

  options.themes.entropism = {
    enable = lib.mkEnableOption "entropism theme";

    variant = lib.mkOption {
      type = lib.types.enum [
        "burn-in"
        "dead-pixel"
        "salvage-phosphor"
      ];
      default = "burn-in";
      description = "Which preset display to emulate.";
    };

    colors = lib.mkOption {
      type = lib.types.submodule {
        options = lib.genAttrs roles (
          role:
          lib.mkOption {
            type = lib.types.nullOr lib.types.str;
            default = null;
            example = "#d9a24a";
            description = "Override the ${role} role (\"#rrggbb\").";
          }
        );
      };
      default = { };
      description = ''
        Override any semantic role; null falls back to the variant
        palette. `tape` additionally falls back to `fg`.
      '';
    };

    uiFont = lib.mkOption {
      type = lib.types.submodule {
        options = {
          package = lib.mkOption {
            type = lib.types.package;
            default = pkgs.departure-mono;
          };
          name = lib.mkOption {
            type = lib.types.str;
            default = "Departure Mono";
          };
        };
      };
      default = { };
      description = ''
        Bitmap-adjacent face for the bar, launcher and notifications.
        Terminal content keeps whatever stylix.fonts.monospace is, so
        code stays legible.
      '';
    };

    # Read by this theme's own submodules; not part of the wrapper API.
    resolvedColors = lib.mkOption {
      internal = true;
      readOnly = true;
      type = lib.types.attrsOf lib.types.str;
      description = "Variant palette with overrides and fallbacks applied.";
    };
  };

  config = lib.mkMerge [
    # Defined unconditionally so the submodules can read it without
    # ordering games; they gate their own config on `enable`.
    { themes.entropism.resolvedColors = resolved; }

    (lib.mkIf cfg.enable {
      home.packages = [ cfg.uiFont.package ];
      fonts.fontconfig.enable = true;

      stylix = {
        enable = true;

        # Collapsing syntax highlighting into fg/dim/alert is the point,
        # not an oversight: an entropism display has one working colour.
        # base08/09/0F are the alert red, base0A carries the tape accent
        # so a marker-written label still reads as a label, and every
        # other slot is fg or dim. No rainbow.
        base16Scheme = {
          scheme = "Entropism ${cfg.variant}";
          author = "generated from themes/entropism";

          base00 = hex "bg";
          base01 = hex "panel";
          base02 = hex "border";
          base03 = hex "dim";
          base04 = hex "dim";
          # No lighten helper exists in this repo and the spec is not
          # worth a colour-maths dependency, so the light end of the ramp
          # is simply fg.
          base05 = hex "fg";
          base06 = hex "fg";
          base07 = hex "fg";

          base08 = hex "alert";
          base09 = hex "alert";
          base0A = hex "tape";
          base0B = hex "fg";
          base0C = hex "dim";
          base0D = hex "fg";
          base0E = hex "dim";
          base0F = hex "alert";
        };

        # A flat field of the background colour. mkDefault so a host can
        # still supply a real wallpaper, and so it loses to a system
        # image propagated by stylix.homeManagerIntegration.
        image = lib.mkDefault (
          pkgs.runCommand "entropism-${cfg.variant}-bg.png" { color = resolved.bg; }
            "${lib.getExe' pkgs.imagemagick "convert"} xc:$color png32:$out"
        );

        # UI face for GTK and friends. Terminal content deliberately
        # keeps home/common's monospace.
        fonts.sansSerif = lib.mkDefault {
          inherit (cfg.uiFont) package name;
        };
      };

      # Square, unlit, 1px. home/common/hyprland sets these at mkForce
      # (to beat stylix), so overriding them needs a stronger priority
      # than mkForce rather than another mkForce, which would only
      # collide.
      wayland.windowManager.hyprland.settings = {
        general = {
          border_size = entropismOverride 1;
          "col.active_border" = entropismOverride "rgba(${hex "fg"}ff)";
          "col.inactive_border" = entropismOverride "rgba(${hex "border"}ff)";
        };

        decoration = {
          rounding = entropismOverride 0;
          blur.enabled = entropismOverride false;
          shadow.enabled = entropismOverride false;
        };

        group = {
          "col.border_active" = entropismOverride "rgba(${hex "fg"}ff)";
          "col.border_inactive" = entropismOverride "rgba(${hex "border"}ff)";
          "col.border_locked_active" = entropismOverride "rgba(${hex "alert"}ff)";
          "col.border_locked_inactive" = entropismOverride "rgba(${hex "border"}ff)";

          groupbar = {
            "col.active" = entropismOverride "rgba(${hex "fg"}ff)";
            "col.inactive" = entropismOverride "rgba(${hex "border"}ff)";
            "col.locked_active" = entropismOverride "rgba(${hex "alert"}ff)";
            "col.locked_inactive" = entropismOverride "rgba(${hex "border"}ff)";
            text_color = entropismOverride "rgba(${hex "bg"}ff)";
          };
        };
      };
    })
  ];
}
