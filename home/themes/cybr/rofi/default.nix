{ pkgs, lib, config, ... }:

let
  shades = import ../../../lib/shades.nix { inherit lib; };
  inherit (shades.forColors config.lib.stylix.colors) mid dark;
in

{
  home.packages = [
    # rofi-wayland was merged into rofi upstream.
    pkgs.rofi
  ];

  # The rasi chain is config.rasi -> style.rasi -> theme/cybrcore.rasi, and
  # every file resolves the next by absolute ~/.config path, so all of them
  # have to land in the same place.
  xdg.configFile."rofi/config.rasi".source = ./cybr-rofi/config.rasi;
  xdg.configFile."rofi/style.rasi".source = ./cybr-rofi/style.rasi;
  xdg.configFile."rofi/launcher.rasi".source = ./cybr-rofi/launcher.rasi;
  xdg.configFile."rofi/scripts".source = ./cybr-rofi/scripts;

  # Upstream ships this file with the palette baked in. Generate it from the
  # active stylix scheme instead, the same way waybar/colors.css is built, so
  # the launcher follows the theme rather than pinning its own red.
  xdg.configFile."rofi/theme/cybrcore.rasi".text = ''
    /* Generated dynamically from Stylix active palette */
    * {
      background:         #${config.lib.stylix.colors.base00}90;
      background-full:    #${config.lib.stylix.colors.base00}FF;
      foreground:         #${config.lib.stylix.colors.base08};
      accent:             #${config.lib.stylix.colors.base08};
      background-tb:      #${dark "base08"};
      border-tb:          #${config.lib.stylix.colors.base0B};
      background-none:    #${config.lib.stylix.colors.base0D};
      selected:           linear-gradient(to right, #${config.lib.stylix.colors.base08}30, #${config.lib.stylix.colors.base08}30);
      button:             linear-gradient(#${config.lib.stylix.colors.base08}30);
      button-selected:    linear-gradient(#${config.lib.stylix.colors.base08}30);
      active:             linear-gradient(to right, #${config.lib.stylix.colors.base0C}FF, #${config.lib.stylix.colors.base0B}FF);
      urgent:             #${config.lib.stylix.colors.base09};
      font:               "${config.stylix.fonts.monospace.name} 12";
      gap:                8px;
      radius:             0px;
      no0:                #${config.lib.stylix.colors.base00};
      re0:                #${config.lib.stylix.colors.base08};
    }
  '';
}
