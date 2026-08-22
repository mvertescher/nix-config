# neomil -- Neo-Militarism, the issued-equipment era.
#
# The default player UI of Cyberpunk 2077: hard edges, military
# hierarchy, escalation by red brightness rather than by hue. The
# `reference` palette is transcribed from the sampled values in
# home/common/pkgs/neomil-ui, not eyeballed -- see ./palettes.nix.
#
# Distinct from `cybr`, which is also Neomilitarism-flavoured: cybr is
# the cybrcore community look with vendored assets, this is the
# reference-sampled one and generates everything from roles.
#
#   imports = [ nix-config/home/themes/neomil ];
#   themes.neomil = {
#     enable = true;
#     variant = "bleach";      # light grey/white
#     colors.alert = "#ff0033";
#   };
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.themes.neomil;

  scheme = import ./scheme.nix;
  roleLib = import ../lib/roles.nix;

  overrides = lib.filterAttrs (_: v: v != null) cfg.colors;

  resolved = scheme.resolve {
    inherit (cfg) variant;
    inherit overrides;
  };
in
{
  options.themes.neomil = {
    enable = lib.mkEnableOption "neomil theme";

    variant = lib.mkOption {
      type = lib.types.enum (builtins.attrNames scheme.palettes);
      default = "reference";
      description = ''
        reference -- sampled reds on near-black, the faithful read.
        bleach    -- light mode: grey and paper white, reds kept for
                     escalation and labels.
        ash       -- dark but neutral, red reserved for what matters.
      '';
    };

    colors = lib.mkOption {
      type = lib.types.submodule {
        options = lib.genAttrs roleLib.names (
          role:
          lib.mkOption {
            type = lib.types.nullOr lib.types.str;
            default = null;
            example = "#de2e2e";
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
        Rajdhani is the typeface Cyberpunk 2077 sets its own in-game
        interface in, with Orbitron secondary; this repo already vendors
        both. Terminal content keeps stylix.fonts.monospace.
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
        Wallpaper treatment generated from `bg`. Off by default; the
        references are clean panels rather than degraded ones.
      '';
    };

    firefox.restartOnActivation = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = ''
        Restart a running Firefox when the theme changes, since
        userChrome is only read at startup.
      '';
    };

    resolvedColors = lib.mkOption {
      internal = true;
      readOnly = true;
      type = lib.types.attrsOf lib.types.str;
      description = "Variant palette with overrides and fallbacks applied.";
    };
  };

  config = lib.mkMerge [
    { themes.neomil.resolvedColors = resolved; }

    (lib.mkIf cfg.enable (lib.mkMerge [
      {
        stylix.base16Scheme = scheme.toBase16 cfg.variant resolved;

        # Inferred from the background rather than restated per palette,
        # so `bleach` is correctly recognised as a light scheme and
        # stylix stops guessing at GTK and icon variants.
        stylix.polarity = lib.mkDefault (roleLib.polarityOf resolved);
      }

      (import ../lib/era.nix {
        inherit lib pkgs config;
        name = "Neomil";
        inherit (cfg) variant texture;
        roles = resolved;
        font = cfg.uiFont;
        browserRestart = cfg.firefox.restartOnActivation;
      })
    ]))
  ];
}
