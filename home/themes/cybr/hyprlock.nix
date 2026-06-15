{
  config,
  lib,
  pkgs,
  ...
}:

let
  # Lucid Color Palette dynamically derived from Stylix!
  no0 = config.lib.stylix.colors.base00; # Black
  re0 = config.lib.stylix.colors.base08; # Red
  gr0 = config.lib.stylix.colors.base0B; # Green
  ye0 = config.lib.stylix.colors.base0A; # Yellow

  # Secondary dark accent shades
  re2 = "331215";
  trF = "FF"; # Full opacity
  tr2 = "20"; # Shadow opacity

  # Metrics & Typography
  border = 1;
  font = "GeistMono Nerd Font";
  fontXL = 64;
  fontM = 12;

  # Dynamic Wallpaper Integration
  wallpaperPath = if config.custom.wallpaper.enable then "${config.custom.wallpaper.file}" else "";

  # Blur & Noise settings matching cybr-hyprland
  blurSize = 6;
  blurPass = 4;
  noise = 0.05;
in
{
  stylix.targets.hyprlock.enable = lib.mkForce false;

  programs.hyprlock = {
    enable = true;
    settings = {
      general = {
        hide_cursor = true;
      };

      background = {
        monitor = "";
        path = lib.mkForce wallpaperPath;
        color = lib.mkForce "rgba(${re0}${trF})";
        blur_size = blurSize;
        blur_passes = blurPass;
        noise = noise;
        contrast = 1.3;
        brightness = 0.8;
        vibrancy = 0.2;
        vibrancy_darkness = 0.0;
        ignore_opacity = true;
        new_optimizations = true;
      };

      input-field = {
        monitor = "";
        rounding = 0;
        shadow_passes = 0;
        size = "300, 50";
        outline_thickness = border;
        dots_size = 0.2;
        dots_spacing = 1.0;
        dots_center = true;
        font_color = lib.mkForce "rgba(${re0}${trF})";
        inner_color = lib.mkForce "rgba(${re2}${trF})";
        check_color = lib.mkForce "rgba(${gr0}${trF})";
        fail_color = lib.mkForce "rgba(${ye0}${trF})";
        fail_text = "<i>$FAIL <b>($ATTEMPTS)</b></i>";
        fail_transition = 300;
        fade_on_empty = true;
        placeholder_text = "<i>Password</i>";
        hide_input = false;
        position = "0, 50";
        halign = "center";
        valign = "bottom";
      };

      label = [
        # Clock
        {
          monitor = "";
          shadow_passes = 0;
          text = "cmd[update:1000] echo \"<b><big> $(date +\"%H:%M\") </big></b>\"";
          color = lib.mkForce "rgba(${re0}${trF})";
          font_size = fontXL;
          font_family = font;
          position = "0, -35";
          halign = "center";
          valign = "center";
        }
        # Uptime
        {
          monitor = "";
          text = ''cmd[update:1000] echo "󰔚 $(${pkgs.gawk}/bin/awk '{s=$1; d=int(s/86400); h=int((s%86400)/3600); m=int((s%3600)/60); sec=int(s%60); printf "%02d:%02d:%02d:%02d\n", d, h, m, sec}' /proc/uptime)"'';
          shadow_passes = 0;
          color = lib.mkForce "rgba(${re0}${trF})";
          font_size = fontM;
          font_family = font;
          position = "0, -105";
          halign = "center";
          valign = "center";
        }
        # Hostname HUD (Top-Left)
        {
          monitor = "";
          text = "cmd[update:3600000] echo \"<span font_family='${font}' font_weight='bold'>$(uname -n)</span>\"";
          color = lib.mkForce "rgba(${re0}${trF})";
          font_size = fontM;
          position = "20, -20";
          halign = "left";
          valign = "top";
          shadow_passes = 0;
        }
      ];

      # Avatar image
      image = {
        monitor = "";
        path = "$HOME/.config/hypr/face.png";
        outline_thickness = 2;
        border_size = border;
        border_color = lib.mkForce "rgba(${re0}${trF})";
        rounding = 0;
        position = "0, 150";
        halign = "center";
        valign = "center";
      };
    };
  };

  wayland.windowManager.hyprland.settings = {
    bind = [
      "SUPER, backspace, exec, hyprlock"
    ];
  };
}
