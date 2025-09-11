{ ... }:

{
  programs.hyprlock = {
    enable = true;
    settings = {
      general = {
        hide_cursor = true;
      };

      animations = {
        enabled = true;
        bezier = [
          "easeout,0.5, 1, 0.9, 1"
          "easeoutback,0.34,1.22,0.65,1"
        ];
        animation = [
          "fade, 1, 3, easeout"
          "inputField, 1, 1, easeoutback"
        ];
      };

      input-field = {
        placeholder_text = "";

        size = "20%, 5%";
        outline_thickness = 3;
        # inner_color = "rgba(0, 0, 0, 0.0)"; # no fill

        # outer_color = "rgba(33ccffee) rgba(00ff99ee) 45deg";
        # check_color = "rgba(00ff99ee) rgba(ff6633ee) 120deg";
        # fail_color = "rgba(ff6633ee) rgba(ff0066ee) 40deg";

        # font_color = "rgb(143, 143, 143)";
        fade_on_empty = false;
        rounding = "15";

        position = "0, -20";
        halign = "center";
        valign = "center";
      };

      label = {
        text = "$TIME";
        color = "rgba(200, 200, 200, 1.0)";
        font_size = 25;
        font_family = "Noto Sans";

        position = "0, 80";
        halign = "center";
        valign = "center";
      };
    };
  };

  wayland.windowManager.hyprland = {
    settings = {
      bind = [
        "SUPER, backspace, exec, hyprlock"
      ];
    };
  };
}