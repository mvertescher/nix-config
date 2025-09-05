{ pkgs, lib, ... }:

let

in {
  imports = [
    ./binds.nix
  ];

  wayland.windowManager.hyprland = {
    enable = true;
    plugins = [ ];
  };

  wayland.windowManager.hyprland.settings = {
    general = {
      gaps_in = 5;
      gaps_out = 20;
      border_size = 2;
      # col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg;
      # col.inactive_border = rgba(595959aa);
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
      active_opacity = 1.0;
      inactive_opacity = 0.95;

      shadow = {
        enabled = true;
        range = 4;
        render_power = 3;
        # color = rgba(1a1a1aee);
      };
    };

    # TODO: Add these
    # env = [
    #   "XCURSOR_SIZE,24"
    #   "HYPRCURSOR_SIZE,24"
    # ];

    animations = {
      enabled = 0;
    };

    # Ignore maximize requests from apps
    windowrule = "suppressevent maximize, class:.*";

    debug = {
      disable_logs = false;
    };
  };
}

