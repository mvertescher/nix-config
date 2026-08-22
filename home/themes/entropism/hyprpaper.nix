# entropism wallpaper daemon.
#
# The theme generates its own background from the `bg` role (see the
# `texture` option), but something has to put it on the screen. cybr
# hand-rolls hyprpaper and force-disables stylix's target; do the same
# here so the image shown is the one this theme generated, not one
# stylix picked.
{
  config,
  lib,
  ...
}:

let
  cfg = config.themes.entropism;
in
{
  config = lib.mkIf cfg.enable {
    stylix.targets.hyprpaper.enable = lib.mkForce false;

    wayland.windowManager.hyprland.settings.exec-once = [ "hyprpaper" ];

    services.hyprpaper = {
      enable = true;

      settings = {
        # Same file stylix was handed, so the desktop and every other
        # surface agree on the background.
        preload = [ "${config.stylix.image}" ];
        wallpaper = [ ",${config.stylix.image}" ];
        ipc = false;
        splash = false;
      };
    };
  };
}
