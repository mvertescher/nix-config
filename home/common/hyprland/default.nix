{ pkgs, lib, config, ... }:

let
  # Lucid Color Palette dynamically derived from Stylix!
  no0 = config.lib.stylix.colors.base00; # Black
  re0 = config.lib.stylix.colors.base08; # Red # Green
  vi0 = config.lib.stylix.colors.base0E; # Violet
  cy0 = config.lib.stylix.colors.base0C; # Cyan

  # The quiet end of a pair (an inactive group border, an inactive tab).
  # A plain scheme colour: a theme that wants a derived shade states it
  # itself (cybr does, from its own shades.nix), so this module needs no
  # colour arithmetic and no helper from a theme.
  re1 = config.lib.stylix.colors.base03;

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
    ./workspace-app.nix
  ];

  custom.wallpaper = {
    enable = lib.mkDefault true;
    name = lib.mkDefault "roppongi";
  };

  home.packages = with pkgs; [
    brightnessctl
    grimblast # hyprland screenshot tool
    hyprland
    hyprpicker
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

  # This module owns the baseline look, so stylix's hyprland target is
  # off: it wrote the same keys at normal priority, which forced every
  # value here to mkForce and every theme one step past that -- two
  # files that each had to know the other's priority number. With the
  # target off, the look below is a set of defaults and a theme
  # overrides any of it with a plain definition. The target also set
  # `misc.background_color`, kept below, and turned hyprpaper on, which
  # each theme does for itself.
  stylix.targets.hyprland.enable = false;

  wayland.windowManager.hyprland.settings = {
    exec-once = [ "hyprctl dispatch workspace 3" ];
    general = {
      gaps_in = lib.mkDefault gapS;
      gaps_out = lib.mkDefault gapM;
      border_size = lib.mkDefault border;
      "col.inactive_border" = lib.mkDefault "rgba(${cy0}${tr0})";
      "col.active_border" = lib.mkDefault "rgba(${re0}${trF})";
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
      # Let a replacement lock client take over when the previous one
      # dies, so a crashed lock screen is recoverable in place instead
      # of stranding the session behind a surface nothing owns.
      #
      # Off by default, which is how a host running this config ended
      # up on Hyprland's "lockscreen app died" recovery page on
      # 2026-08-23 with the documented fix unavailable: the page tells
      # you to set this keyword, but by then the compositor is already
      # wedged and even a known-good lock client takes the lock and
      # immediately loses its surface. Set it *before* it is needed, or
      # it is not a recovery path.
      allow_session_lock_restore = true;
      disable_hyprland_logo = true;
      background_color = lib.mkDefault "rgb(${no0})";
      disable_splash_rendering = true;
      force_default_wallpaper = 0;
      initial_workspace_tracking = 1;
    };

    decoration = {
      rounding = lib.mkDefault radius;
      rounding_power = lib.mkDefault power;
      active_opacity = activeTr;
      inactive_opacity = inActiveTr;

      blur = {
        enabled = lib.mkDefault true;
        size = blurSize;
        passes = blurPass;
        inherit noise;
        ignore_opacity = true;
        new_optimizations = true;
        xray = false;
        popups = true;
      };

      shadow = {
        enabled = lib.mkDefault true;
        range = lib.mkDefault 30;
        scale = lib.mkDefault 2;
        render_power = 5;
        color = lib.mkDefault "rgba(${re0}${tr2})";
        color_inactive = lib.mkDefault "rgba(${no0}${tr2})";
      };

      dim_inactive = false;
      dim_strength = 1.0;
      dim_special = 0.0;
    };

    # Vendor-neutral only. The GPU vendor variables that used to sit here
    # -- LIBVA_DRIVER_NAME, GBM_BACKEND, __GLX_VENDOR_LIBRARY_NAME -- were
    # hardcoded to nvidia for every host importing this module, which is
    # not a cosmetic wrong default on an AMD or Intel machine: it points
    # the VA-API, GBM and GLX loaders at a driver that is not installed and
    # can take the session down. They now live in a GPU profile the host
    # imports alongside this one, e.g. ../../profiles/gpu/nvidia.nix.
    env = [
      "XDG_SESSION_TYPE,wayland"
    ];

    animations = {
      enabled = lib.mkDefault false;
    };


    group = {
      "col.border_active" = lib.mkDefault "rgba(${re0}${trF})";
      "col.border_inactive" = lib.mkDefault "rgba(${re1}${trF})";
      "col.border_locked_active" = lib.mkDefault "rgba(${vi0}${trF})";
      "col.border_locked_inactive" = lib.mkDefault "rgba(${vi0}${tr7})";

      groupbar = {
        "col.active" = lib.mkDefault "rgba(${re0}${trF})";
        "col.inactive" = lib.mkDefault "rgba(${re1}${trF})";
        "col.locked_active" = lib.mkDefault "rgba(${vi0}${trF})";
        "col.locked_inactive" = lib.mkDefault "rgba(${vi0}${tr7})";
        font_size = fontM;
        text_color = lib.mkDefault "rgba(${no0}${trF})";
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
