# cybr's own window-manager colours.
#
# The shared Hyprland module (home/common/hyprland) sets a baseline
# look as defaults; these two were the only values in it that needed
# cybr's shade arithmetic, so they live with the theme. The mid shade of
# the scheme's red marks the quiet end of a pair: an inactive group
# border and an inactive tab.
{ lib, config, ... }:

let
  shades = (import ./lib/shades.nix { inherit lib; }).forColors config.lib.stylix.colors;
  quiet = "rgba(${shades.mid "base08"}FF)";
in
{
  wayland.windowManager.hyprland.settings.group = {
    "col.border_inactive" = quiet;
    groupbar."col.inactive" = quiet;
  };
}
