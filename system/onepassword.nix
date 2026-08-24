# 1Password: the desktop app, the `op` CLI, and the one piece of glue
# NixOS needs that no other distribution does.
#
# Both nixpkgs modules are used rather than plain packages, because both
# install setuid/setgid helpers that a bare `environment.systemPackages`
# entry cannot: the GUI needs `1Password-BrowserSupport` owned by the
# `onepassword` group for browser integration, and the CLI needs
# `op` in the `onepassword-cli` group for the desktop app to authorise it
# biometrically. Installing the packages by hand gets you binaries that
# start and then refuse to talk to each other.
{
  config,
  lib,
  ...
}:

let
  cfg = config.custom.onePassword;
in
{
  options.custom.onePassword = {
    enable = lib.mkEnableOption "the 1Password desktop app and CLI";

    users = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "alice" ];
      description = ''
        Accounts allowed to authorise 1Password's polkit actions --
        which is what "unlock with your system password" and browser
        integration go through. Empty means nobody can, which makes the
        app installed and useless, so this asserts rather than defaults.
      '';
    };

    browsers = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ".firefox-wrapped" ];
      description = ''
        Process names 1Password will accept browser-integration requests
        from, written to /etc/1password/custom_allowed_browsers.

        This exists because of a NixOS-specific trap. The app verifies
        the *code signature and name* of the process asking to integrate,
        against a built-in list of browsers it trusts. On NixOS
        `programs.firefox` installs a wrapper script that execs the real
        binary as `.firefox-wrapped`, so the process asking is not called
        `firefox` and is refused -- silently, from the browser's point of
        view: the extension simply never finds the app and offers to set
        up a standalone vault instead, which looks like a first-run
        screen rather than an error.

        Check with `readlink -f $(command -v firefox)` and look at what
        the wrapper execs if browser unlock stops working after a Firefox
        update changes the wrapper's shape.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.users != [ ];
        message = "custom.onePassword.users must list at least one account; "
          + "with none, polkit refuses every unlock and the app cannot be used.";
      }
    ];

    programs._1password.enable = true;

    programs._1password-gui = {
      enable = true;
      polkitPolicyOwners = cfg.users;
    };

    environment.etc."1password/custom_allowed_browsers" = {
      text = lib.concatStringsSep "\n" cfg.browsers + "\n";
      mode = "0755";
    };
  };
}
