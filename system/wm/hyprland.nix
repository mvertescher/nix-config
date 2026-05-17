# nixos hyprland configuration

{ pkgs, lib, ... }:

{
  programs.hyprland = {
    enable = true;
    withUWSM = true;
  };

  # Enable greetd display manager with tuigreet
  systemd.services.greetd.serviceConfig = {
    Type = "idle";
    StandardInput = "tty";
    StandardOutput = "tty";
    StandardError = "journal";
    TTYReset = true;
    TTYHangup = true;
    TTYVDisallocate = true;
  };

  services.greetd = {
    enable = true;
    settings = {
      default_session = {
        command = "${pkgs.tuigreet}/bin/tuigreet --time --remember --remember-session --cmd '${pkgs.uwsm}/bin/uwsm start hyprland-uwsm.desktop'";
        user = "greeter";
      };
    };
  };

  # Pre-populate tuigreet cache so username is automatically pre-filled on first boot
  systemd.tmpfiles.rules = [
    "d /var/cache/tuigreet 0755 greeter greeter - -"
    "f /var/cache/tuigreet/lastuser 0644 greeter greeter - mverte"
  ];

}
