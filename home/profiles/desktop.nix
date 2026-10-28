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
# Selected by import, not by an option; ./laptop.nix is the laptop half.
# See PROFILE-DESIGN.md in the consuming wrapper for the full argument.
_:

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
      };

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
