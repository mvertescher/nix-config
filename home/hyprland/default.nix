{ pkgs, lib, ... }:

let

in {
  imports = [
    ./binds.nix
    ./hyprlock.nix
    # ./hyprpaper.nix
  ];

  # home.pointerCursor.hyprcursor.enable = true;

  wayland.windowManager.hyprland = {
    enable = true;
    plugins = [ ];
  };

  wayland.windowManager.hyprland.settings = {
    general = {
      gaps_in = 5;
      gaps_out = 20;
      border_size = 2;
      # "col.active_border" = "rgba(33ccffee) rgba(00ff99ee) 45deg";
      # "col.inactive_border" = "rgba(595959aa)";
      resize_on_border = false;
      allow_tearing = false;
      layout = "dwindle";
    };

    input = {
      kb_layout = "us";
      # kb_variant =
      # kb_model =
      # kb_options =
      # kb_rules =
      # follow_mouse = 1
      touchpad = {
        natural_scroll = true;
      };
      # sensitivity = 0 # -1.0 - 1.0, 0 means no modification.
    };

    misc = {
      disable_autoreload = false; # Need autoreload
      disable_hyprland_logo = true;
      disable_splash_rendering = true;
      force_default_wallpaper = 0;
      initial_workspace_tracking = 1;
    };

    decoration = {
      rounding = 4;
      # rounding_power = 2

      blur = {
        enabled = true;
        size = 3;
        passes = 1;
        vibrancy = 0.1696;
      };

      fullscreen_opacity = 1.0;
      active_opacity = 0.99;
      inactive_opacity = 0.95;

      shadow = {
        enabled = true;
        range = 4;
        render_power = 3;
        # color = "rgba(1a1a1aee)";
      };
    };

    # TODO: Add these
    # env = [
    #   "XCURSOR_SIZE,24"
    #   "HYPRCURSOR_SIZE,24"
    # ];

    animations = {
      enabled = 1;

      # Default curves
      bezier = [
        # NAME,          X0,   Y0,   X1,   Y1
        "easeOutQuint,   0.23, 1,    0.32, 1"
        "easeInOutCubic, 0.65, 0.05, 0.36, 1"
        "linear,         0,    0,    1,    1"
        "almostLinear,   0.5,  0.5,  0.75, 1"
        "quick,          0.15, 0,    0.1,  1"
      ];

      # Default animations
      animation = [
        # NAME,         ONOFF, SPEED, CURVE,        [STYLE]
        "global,        1,     10,    default"
        "border,        1,     5.39,  easeOutQuint"
        "windows,       1,     4.79,  easeOutQuint"
        "windowsIn,     1,     4.1,   easeOutQuint, popin 87%"
        "windowsOut,    1,     1.49,  linear,       popin 87%"
        "fadeIn,        1,     1.73,  almostLinear"
        "fadeOut,       1,     1.46,  almostLinear"
        "fade,          1,     3.03,  quick"
        "layers,        1,     3.81,  easeOutQuint"
        "layersIn,      1,     4,     easeOutQuint, fade"
        "layersOut,     1,     1.5,   linear,       fade"
        "fadeLayersIn,  1,     1.79,  almostLinear"
        "fadeLayersOut, 1,     1.39,  almostLinear"
        "workspaces,    1,     1.94,  almostLinear, fade"
        "workspacesIn,  1,     1.21,  almostLinear, fade"
        "workspacesOut, 1,     1.94,  almostLinear, fade"
        "zoomFactor,    1,     7,     quick"
      ];
    };

    # Ignore maximize requests from apps
    windowrule = "suppressevent maximize, class:.*";

    debug = {
      disable_logs = false;
    };
  };
}

