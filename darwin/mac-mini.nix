# Mac mini: an always-on, mostly-headless Mac reached over the network.
#
# Opt-in per host (import it from the host's modules); lib/mkDarwin.nix
# does not apply it. Nothing here is specific to one machine — identity
# (hostname, user, keys beyond the shared ones) belongs to the wrapper.

{ config, lib, ... }:

let
  user = config.system.primaryUser;
in
{
  # --- Stay up ---------------------------------------------------------
  # A desktop with no battery has no reason to sleep, and a sleeping
  # machine is an unreachable one. The display may still blank.
  power.sleep = {
    computer = "never";
    harddisk = "never";
    display = lib.mkDefault 10;
  };

  # Come back on its own after an outage or a hang instead of waiting
  # for someone to press the button on the back.
  power.restartAfterPowerFailure = true;
  power.restartAfterFreeze = true;

  networking.wakeOnLan.enable = true;

  # --- Remote access ---------------------------------------------------
  # Apple's sshd ("Remote Login"). Key-only: Remote Login lets any local
  # account in by password otherwise.
  services.openssh = {
    enable = true;
    extraConfig = ''
      PasswordAuthentication no
      KbdInteractiveAuthentication no
    '';
  };

  users.users.${user}.openssh.authorizedKeys.keys = import ../lib/ssh-keys.nix;

  # A machine nobody sits at has no use for a guest account, and the
  # login window is exposed to anyone on the LAN via Screen Sharing.
  system.defaults.loginwindow.GuestEnabled = false;
}
