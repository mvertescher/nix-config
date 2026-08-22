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

  # Shared with wrappers that need the same scheme at the NixOS level;
  # see scheme.nix.
  scheme = import ./scheme.nix;

  roles = [
    "bg"
    "panel"
    "border"
    "dim"
    "fg"
    "alert"
    "tape"
  ];

  # A null override means "keep the variant's value", so a host can set
  # one role without restating six.
  overrides = lib.filterAttrs (_: v: v != null) cfg.colors;

  resolved = scheme.resolve {
    inherit (cfg) variant;
    inherit overrides;
  };

  # Bare six-digit hex, so callers can append an alpha suffix.
  hex = role: lib.removePrefix "#" resolved.${role};

  # home/common/hyprland pins its colours and rounding with mkForce so
  # they beat stylix. A second mkForce here would be a same-priority
  # collision, not an override, so the theme sits one step stronger.
  entropismOverride = lib.mkOverride 40;

  magick = lib.getExe' pkgs.imagemagick "magick";

  # Generated rather than shipped: the wallpaper is derived from the same
  # `bg` role as everything else, so it follows an override instead of
  # becoming a stale asset with a colour baked into it.
  wallpaper =
    pkgs.runCommand "entropism-${cfg.variant}-${cfg.texture}.png"
      {
        bg = resolved.bg;
        line = resolved.panel;
      }
      (
        {
          none = ''
            ${magick} -size 3840x2160 xc:"$bg" png32:$out
          '';

          # Horizontal lines every 4px, one pixel tall: a CRT that has
          # been left on too long. Built as a tile then repeated, rather
          # than with an mpr: register, which does not survive the
          # write/delete round trip reliably.
          scanlines = ''
            ${magick} -size 1x3 xc:"$bg" -size 1x1 xc:"$line" -append tile.png
            ${magick} -size 3840x2160 tile:tile.png png32:$out
          '';

          # Fine grain, as though the panel is amplifying its own noise.
          # Grey noise blended over the background keeps the hue.
          noise = ''
            ${magick} -size 3840x2160 xc:gray50 -attenuate 1.2 +noise Gaussian \
              -colorspace Gray grain.png
            ${magick} -size 3840x2160 xc:"$bg" grain.png \
              -compose blend -define compose:args=7 -composite png32:$out
          '';
        }
        .${cfg.texture}
      );
in
{
  imports = [
    ./firefox.nix
    ./hyprpaper.nix
    ./rofi.nix
    ./starship.nix
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
            default = pkgs.callPackage ../../common/pkgs/rajdhani-fontshare { };
          };
          name = lib.mkOption {
            type = lib.types.str;
            default = "Rajdhani";
          };
        };
      };
      default = { };
      description = ''
        Face for the bar, launcher and notifications. Defaults to
        Rajdhani, which is the typeface Cyberpunk 2077 actually sets its
        in-game interface in (Orbitron is its secondary), and which this
        repo already vendors.

        For a more literal salvaged-terminal read, `pkgs.departure-mono`
        ("Departure Mono") is bitmap-adjacent and also packaged.

        Terminal content keeps whatever stylix.fonts.monospace is, so
        code stays legible.
      '';
    };

    texture = lib.mkOption {
      type = lib.types.enum [
        "none"
        "scanlines"
        "noise"
      ];
      default = "none";
      description = ''
        Wallpaper treatment. "none" is a flat field of `bg`; the others
        add a degraded-display artefact generated from the same colour.
        Off by default -- an entropism display earns its texture from
        age, not from decoration.
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
        base16Scheme = scheme.toBase16 cfg.variant resolved;

        # mkDefault so a host can still supply a real wallpaper, and so
        # it loses to a system image propagated by
        # stylix.homeManagerIntegration.
        image = lib.mkDefault wallpaper;

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
