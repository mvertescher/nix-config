{ lib, ... }:

let
  browser = "google-chrome";
  fileManager = "nemo";
  mainMod = "SUPER";
  menu = "wofi --show drun";
  nixConfigUpdate = "alacritty nu -c hms";
  terminal = "alacritty";
in {
  wayland.windowManager.hyprland.settings = {
    # Move/resize windows with mainMod + LMB/RMB and dragging
    bindm = [
      "SUPER,mouse:272,movewindow"
      "SUPER,mouse:273,resizewindow"
    ];

    bindel = [
      ", XF86AudioRaiseVolume, exec, wpctl set-volume -l 1.0 @DEFAULT_AUDIO_SINK@ 5%+"
      ", XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-"
      ", XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle"
      ", XF86AudioMicMute, exec, wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle"
      ", XF86MonBrightnessUp, exec, brightnessctl -e4 -n2 set 5%+"
      ", XF86MonBrightnessDown, exec, brightnessctl -e4 -n2 set 5%-"
    ];

    bind = let
      # Hyprland's regular workspaces are 1-indexed: workspace 0 does not
      # exist, and dispatching to it silently succeeds while doing
      # nothing (`hyprctl dispatch workspace 0` returns ok and no
      # workspace is created). Binding the 0 key to workspace 0 therefore
      # made it a dead key. Keyboards run 1-9 then 0, so the 0 key is the
      # tenth workspace.
      workspaces = [
        "1"
        "2"
        "3"
        "4"
        "5"
        "6"
        "7"
        "8"
        "9"
      ];
      tenthKey = "0";
      tenth = "10";
      # Map keys (arrows and hjkl) to hyprland directions (l, r, u, d)
      directions = rec {
        left = "l";
        right = "r";
        up = "u";
        down = "d";
        h = left;
        l = right;
        k = up;
        j = down;
      };
    in
      [
        "${mainMod}, Return, exec, ${terminal}"
        "${mainMod}, b, exec, ${browser}"
        "${mainMod}, r, exec, ${menu}"
        "${mainMod}, f, exec, firefox"
        "${mainMod}, w, exec, hyprland-rotate-wallpaper"

        "${mainMod}, c, killactive"
        "${mainMod}, m, exit"
        "${mainMod} shift, r, exec, hyprctl reload"
        "${mainMod}, o, layoutmsg, togglesplit" # dwindle
        "${mainMod}, p, pseudo" # dwindle
        "${mainMod}, v, togglefloating"

        # special scratchpad workspace
        "${mainMod}, s, togglespecialworkspace, magic"
        "${mainMod} shift, s, movetoworkspace, special:magic"


        # bind = $mainMod, W, exec, $code
        # bind = $mainMod, N, exec, $nixConfigUpdate
        # bind = $mainMod, E, exec, $fileManager
      ]
      ++
      # Change workspace
      (map (n: "SUPER,${n},workspace,${n}") workspaces)
      ++ [ "SUPER,${tenthKey},workspace,${tenth}" ]
      ++
      # Move window to workspace
      (map (n: "SUPERSHIFT,${n},movetoworkspacesilent,${n}") workspaces)
      ++ [ "SUPERSHIFT,${tenthKey},movetoworkspacesilent,${tenth}" ]
      ++
      # Move focus
      (lib.mapAttrsToList (key: direction: "SUPER,${key},movefocus,${direction}") directions)
      ++
      # Swap windows
      (lib.mapAttrsToList (key: direction: "SUPERSHIFT,${key},swapwindow,${direction}") directions)
      ++
      # Move windows
      (lib.mapAttrsToList (
          key: direction: "SUPERCONTROL,${key},movewindow,${direction}"
        )
        directions)
      ;
  };
}