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

    bind = let
      workspaces = [
        "0"
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

        "${mainMod}, c, killactive"
        "${mainMod}, m, exit"
        "${mainMod}, v, togglefloating"

        # bind = $mainMod, Q, exec, $safeTerminal
        # bind = $mainMod, W, exec, $code
        # bind = $mainMod, Return, exec, $terminal
        # bind = $mainMod, N, exec, $nixConfigUpdate
        # bind = $mainMod, C, killactive,
        # bind = $mainMod, M, exit,
        # bind = $mainMod, E, exec, $fileManager
        # bind = $mainMod, V, togglefloating,
        # bind = $mainMod, R, exec, $menu
        # bind = $mainMod, P, pseudo, # dwindle
        # bind = $mainMod, U, togglesplit, # dwindle
        # bind = $mainMod, B, exec, $browser
      ]
      ++
      # Change workspace
      (map (n: "SUPER,${n},workspace,${n}") workspaces)
      ++
      # Move window to workspace
      (map (n: "SUPERSHIFT,${n},movetoworkspacesilent,${n}") workspaces)
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