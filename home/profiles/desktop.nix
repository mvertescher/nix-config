# Host profile: desktop.
#
# A mains-powered machine that stays reachable.
#
# Nothing in the stack handles idle: Hyprland has no built-in idle timer,
# and a theme's hyprlock is bound to a keybind only, so the displays
# otherwise stay lit forever. Blank them after 10 minutes; no lock and no
# suspend. A desktop is not saving a battery, and suspending a machine
# that is expected to answer over the network is a downgrade, not a power
# saving.
#
# The systemd user service works because the session comes up under uwsm,
# which activates graphical-session.target even though the shared hyprland
# module leaves `wayland.windowManager.hyprland.systemd.enable` false --
# the same mechanism the themes' daemons rely on (see
# ../themes/lib/era.nix).
#
# Selected by import, not by an option: an enum with one arm is an import.
# See PROFILE-DESIGN.md in the consuming wrapper for the full argument and
# for the laptop half, which is deliberately not written until laptop
# hardware exists.
{ ... }:

{
  services.hypridle = {
    enable = true;
    settings = {
      # Honour dbus idle inhibitors, so video playback keeps the screen on.
      general.ignore_dbus_inhibit = false;

      listener = [
        {
          timeout = 600;
          on-timeout = "hyprctl dispatch dpms off";
          on-resume = "hyprctl dispatch dpms on";
        }
      ];
    };
  };
}
