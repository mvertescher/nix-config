{ pkgs, lib, config, ... }:

let
  # Lucid Color Palette dynamically derived from Stylix!
  no0 = config.lib.stylix.colors.base00; # Black
  no1 = config.lib.stylix.colors.base01;
  re0 = config.lib.stylix.colors.base08; # Red
  gr0 = config.lib.stylix.colors.base0B; # Green
  vi0 = config.lib.stylix.colors.base0E; # Violet
  cy0 = config.lib.stylix.colors.base0C; # Cyan

  # Secondary dark accent shades
  re1 = "631F21";
  vi1 = "421666";

  # Transparency Levels (Alpha channel)
  tr0 = "00";
  tr2 = "20";
  tr7 = "70";
  trF = "FF"; # Full

  # Layout & Decoration Metrics
  border = 1;
  gapS = 4;   # Inner gap
  gapM = 8;   # Outer gap
  radius = 28; # Rounding bevel radius
  power = 1.0; # Bevel power
  activeTr = 1.0;
  inActiveTr = 1.0;

  # Blur & Noise
  blurSize = 6;
  blurPass = 4;
  noise = 0.05;

  # Fonts
  fontM = 12;
in {
  imports = [
    ./binds.nix
  ];

  custom.wallpaper = {
    enable = lib.mkDefault true;
    name = lib.mkDefault "roppongi";
  };

  home.packages = with pkgs; [
    brightnessctl
    grimblast # hyprland screenshot tool
    hyprland
    nemo # file manager
    playerctl
    wofi # app launcher
  ];

  home.pointerCursor = {
    name = "Bibata-Modern-Ice";
    package = pkgs.bibata-cursors;
    size = 24;
    gtk.enable = true;
    x11.enable = true;
    hyprcursor.enable = true;
  };

  wayland.windowManager.hyprland = {
    enable = true;
    systemd.enable = false;
    plugins = [ ];
    configType = "hyprlang";
    extraConfig = ''
      # Pass-through mode for virtualization (QEMU/VMs)
      bind = SUPER, Escape, exec, hyprctl notify 0 2500 "rgb(ff9900)" "Passthrough Mode Enabled"
      bind = SUPER, Escape, submap, passthru
      submap = passthru
      bind = SUPER, Escape, exec, hyprctl notify 0 2500 "rgb(00ff00)" "Passthrough Mode Disabled"
      bind = SUPER, Escape, submap, reset
      submap = reset
    '';
  };

  wayland.windowManager.hyprland.settings = {
    exec-once = [ "hyprctl dispatch workspace 3" ];
    general = {
      gaps_in = gapS;
      gaps_out = gapM;
      border_size = border;
      "col.inactive_border" = lib.mkForce "rgba(${cy0}${tr0})";
      "col.active_border" = lib.mkForce "rgba(${re0}${trF})";
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
      rounding = lib.mkForce radius;
      rounding_power = power;
      active_opacity = activeTr;
      inactive_opacity = inActiveTr;

      blur = {
        enabled = true;
        size = blurSize;
        passes = blurPass;
        noise = noise;
        ignore_opacity = true;
        new_optimizations = true;
        xray = false;
        popups = true;
      };

      shadow = {
        enabled = true;
        range = 30;
        scale = 2;
        render_power = 5;
        color = lib.mkForce "rgba(${re0}${tr2})";
        color_inactive = lib.mkForce "rgba(${no0}${tr2})";
      };

      dim_inactive = false;
      dim_strength = 1.0;
      dim_special = 0.0;
    };

    env = [
      "LIBVA_DRIVER_NAME,nvidia"
      "XDG_SESSION_TYPE,wayland"
      "GBM_BACKEND,nvidia-drm"
      "__GLX_VENDOR_LIBRARY_NAME,nvidia"
    ];

    animations = {
      enabled = false;
    };


    group = {
      "col.border_active" = lib.mkForce "rgba(${re0}${trF})";
      "col.border_inactive" = lib.mkForce "rgba(${re1}${trF})";
      "col.border_locked_active" = lib.mkForce "rgba(${vi0}${trF})";
      "col.border_locked_inactive" = lib.mkForce "rgba(${vi0}${tr7})";

      groupbar = {
        "col.active" = lib.mkForce "rgba(${re0}${trF})";
        "col.inactive" = lib.mkForce "rgba(${re1}${trF})";
        "col.locked_active" = lib.mkForce "rgba(${vi0}${trF})";
        "col.locked_inactive" = lib.mkForce "rgba(${vi0}${tr7})";
        font_size = fontM;
        text_color = lib.mkForce "rgba(${no0}${trF})";
        height = 1;
        indicator_height = 24;
        text_offset = -12;
      };
    };

    debug = {
      disable_logs = false;
    };
  };
}
