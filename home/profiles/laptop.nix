# Host profile: laptop.
#
# A battery-powered machine that suspends when its lid closes.
#
# Idle works as on the desktop (./desktop.nix): nothing else in the stack
# blanks the displays, so hypridle turns them off after 10 minutes. There
# is no lock and no idle suspend; suspending on lid close is logind's job.
#
# The laptop-specific part is after_sleep_cmd. Displays that were blanked
# when the machine suspended can stay dark after resume until a second key
# press, so DPMS is forced back on after every resume.
#
# hyprctl is called by store path, so the listeners do not depend on the
# PATH the session exported to the systemd user manager. The unit itself is
# bound to graphical-session.target, which uwsm activates, as does
# Hyprland's own systemd integration
# (`wayland.windowManager.hyprland.systemd.enable`).
{ config, ... }:

let
  hyprctl = "${config.wayland.windowManager.hyprland.package}/bin/hyprctl";
in
{
  services.hypridle = {
    enable = true;
    settings = {
      general = {
        # Honour dbus idle inhibitors, so video playback keeps the screen on.
        ignore_dbus_inhibit = false;
        # Ignore Wayland zwp_idle_inhibit_manager_v1 inhibitors: Chromium on
        # Wayland leaks zwp_idle_inhibitor_v1 objects after video/RTC wake
        # locks (while properly releasing its D-Bus ScreenSaver lock), and
        # Hyprland treats windows on inactive workspaces as visible when
        # animations are disabled, permanently wedging ext_idle_notifier_v1.
        ignore_wayland_inhibit = true;
        after_sleep_cmd = "${hyprctl} dispatch dpms on";
      };

      listener = [
        {
          timeout = 600;
          on-timeout = "${hyprctl} dispatch dpms off";
          on-resume = "${hyprctl} dispatch dpms on";
        }
      ];
    };
  };
}
