# neokitsch -- "substance and style", the luxury-hardware era.
#
# Gold line-work on true black under a violet haze. Kitsch's later and
# quieter descendant: the ornament is still there, but it has learned
# restraint. Corners are softly rounded -- the reference hardware is --
# with the one clipped corner left to the native bar; the radius knob
# is set from the trace, below.
#
# The `reference` palette is transcribed from the pixel reads in
# home/common/pkgs/cp-eras-ui/docs/neokitsch/README.md, not eyeballed.
#
# This is the era the word "kitsch" makes people picture -- gilded, with
# wood veneer filling every selected element. See ../kitsch for the
# other half of that mix-up.
#
#   imports = [ nix-config/home/themes/neokitsch ];
#   themes.neokitsch = {
#     enable = true;
#     variant = "reference";
#     colors.fg = "#e7c686";
#   };
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.themes.neokitsch;

  scheme = import ./scheme.nix;
  roleLib = import ../lib/roles.nix;

  overrides = lib.filterAttrs (_: v: v != null) cfg.colors;

  resolved = scheme.resolve {
    inherit (cfg) variant;
    inherit overrides;
  };
in
{
  options.themes.neokitsch = {
    enable = lib.mkEnableOption "neokitsch theme";

    variant = lib.mkOption {
      type = lib.types.enum (builtins.attrNames scheme.palettes);
      default = "reference";
      description = ''
        reference -- sampled gold on black under the violet haze.
        bleach    -- light mode: warm paper, gold darkened to a
                     legible bronze, amber kept for escalation.
        ash       -- dark but neutral, gold reserved for what matters.
      '';
    };

    colors = lib.mkOption {
      type = lib.types.submodule {
        options = lib.genAttrs roleLib.names (
          role:
          lib.mkOption {
            type = lib.types.nullOr lib.types.str;
            default = null;
            example = "#e7c686";
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
            # The overlay's build (`lib/overlays.nix`), which is also what
            # `cp-eras-ui` embeds -- one derivation, not a second
            # `callPackage` of the same path.
            default = pkgs.rajdhani-fontshare;
          };
          name = lib.mkOption {
            type = lib.types.str;
            default = "Rajdhani";
          };
          weight = lib.mkOption {
            type = lib.types.ints.between 300 700;
            default = 600;
            description = ''
              The bar sets its labels at 600 (`src/eras/neokitsch.rs`,
              Face::SemiBold); the launcher, notifications and browser
              chrome follow it.
            '';
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

    monoFont = lib.mkOption {
      type = lib.types.nullOr (
        lib.types.submodule {
          options = {
            package = lib.mkOption { type = lib.types.package; };
            name = lib.mkOption { type = lib.types.str; };
          };
        }
      );
      default = null;
      description = ''
        Face for terminal content, or null to keep stylix.fonts.monospace
        (GeistMono, home/common/home.nix). Reserved for an era whose
        material shows a mono face of its own; none sets one by default,
        so code reads the same on every era.
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
        "trace"
      ];
      default = "none";
      description = ''
        Wallpaper treatment. "none" is a flat field of `bg`; "scanlines"
        and "noise" degrade it; "trace" is the reference dashboard's own
        ground -- its gradients, from the design trace -- at monitor
        size. Off by default; the references are clean panels rather
        than degraded ones.
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
    { themes.neokitsch.resolvedColors = resolved; }

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
        name = "Neokitsch";
        # This sat at the house 0 until 2026-09-07, on a reading of the
        # corners as square but for the clipped one; at the desk that
        # was wrong, the reference hardware is rounded. The trace's
        # own numbers (`src/eras/neokitsch.rs`): panels round at 6, rows
        # and the tape at 4, cards and buttons at 3, and the 30px
        # top-right clip is the native bar's to draw. 6 is the panel
        # value, and windows, rofi and swaync are panels. The violet haze
        # in the references is a blur, so the terminal gets one.
        knobs = {
          radius = 6;
          blur = true;
        };
        inherit (cfg) variant texture;
        inherit (cfg) bar;
        roles = resolved;
        font = cfg.uiFont;
        inherit (cfg) monoFont;
        browserRestart = cfg.firefox.restartOnActivation;
        inherit (cfg) lock;
      })
    ]))
  ];
}
