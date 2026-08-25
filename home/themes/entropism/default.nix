# entropism -- the salvaged-hardware era.
#
# Where neomil is the issued equipment, entropism is what came before
# it: necessity over style. Degraded monochrome displays, zero ornament,
# 1px lines, square corners, no glow.
#
# Only the palette, typeface and knobs live here; the desktop itself is
# built by ../lib/era.nix, which the other generated eras share. Colours
# are semantic *roles* rather than base16 slots, so a wrapper retints
# everything by naming one:
#
#   imports = [ nix-config/home/themes/entropism ];
#   themes.entropism = {
#     enable = true;
#     variant = "dead-pixel";
#     colors.fg = "#c8d0c4";
#   };
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.themes.entropism;

  scheme = import ./scheme.nix;
  roleLib = import ../lib/roles.nix;

  # A null override means "keep the variant's value", so a host can set
  # one role without restating six.
  overrides = lib.filterAttrs (_: v: v != null) cfg.colors;

  resolved = scheme.resolve {
    inherit (cfg) variant;
    inherit overrides;
  };
in
{
  options.themes.entropism = {
    enable = lib.mkEnableOption "entropism theme";

    variant = lib.mkOption {
      type = lib.types.enum (builtins.attrNames scheme.palettes);
      default = "nexus";
      description = ''
        Which preset display to emulate.

        nexus            -- the sampled one: sage on warm dark, the era
                            as published. Default since 2026-08-23; the
                            other three predate the sampling pass and
                            were designed to the era's description.
        burn-in          -- amber phosphor with a menu burned into it.
        dead-pixel       -- salvaged grey LCD, green-shifted and uneven.
        salvage-phosphor -- desaturated green CRT.
      '';
    };

    colors = lib.mkOption {
      type = lib.types.submodule {
        options = lib.genAttrs roleLib.names (
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
        Face for the bar, launcher and notifications. Departure Mono is
        bitmap-adjacent, which suits a salvaged terminal; neomil uses
        Rajdhani, the face Cyberpunk 2077 sets its own interface in.

        Terminal content keeps stylix.fonts.monospace either way, so
        code stays legible.
      '';
    };

    bar = lib.mkOption {
      type = lib.types.enum [
        "waybar"
        "cp-eras-ui"
      ];
      default = "waybar";
      description = ''
        Which status bar to run. cp-eras-ui is our own layer-shell
        bar; it is the only one that can draw this era's corner
        treatment, since waybar styles with CSS and a chamfer or a
        clipped corner cannot be expressed there.
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

    lock = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = ''
          Generate a hyprlock configuration from the theme roles and
          bind it. Every generated value is mkDefault, so a host can
          override any single setting with a plain definition; turn
          this off only if you want to drive programs.hyprlock
          entirely yourself.
        '';
      };

      bind = lib.mkOption {
        type = lib.types.str;
        default = "SUPER, backspace";
        example = "SUPER SHIFT, L";
        description = ''
          Key that locks the session. Set to "" to add no binding,
          for example if an idle daemon is the only thing that should
          lock.
        '';
      };
    };

    firefox.restartOnActivation = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = ''
        Restart a running Firefox when the theme changes, since
        userChrome is only read at startup. Fires only on a real change
        and only if Firefox is already running; shutdown is SIGTERM plus
        a wait, so tabs are restored.
      '';
    };

    # Read by nothing outside this theme; exposed for debugging.
    resolvedColors = lib.mkOption {
      internal = true;
      readOnly = true;
      type = lib.types.attrsOf lib.types.str;
      description = "Variant palette with overrides and fallbacks applied.";
    };
  };

  config = lib.mkMerge [
    { themes.entropism.resolvedColors = resolved; }

    (lib.mkIf cfg.enable (lib.mkMerge [
      {
        stylix.base16Scheme = scheme.toBase16 cfg.variant resolved;
        stylix.polarity = lib.mkDefault (roleLib.polarityOf resolved);
      }

      (import ../lib/era.nix {
        inherit lib pkgs config;
        name = "Entropism";
        inherit (cfg) variant texture;
        inherit (cfg) bar;
        roles = resolved;
        font = cfg.uiFont;
        browserRestart = cfg.firefox.restartOnActivation;
        inherit (cfg) lock;
      })
    ]))
  ];
}
