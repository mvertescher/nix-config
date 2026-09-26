# general nix-darwin configuration, applied to every Mac by
# lib/mkDarwin.nix — the macOS counterpart of system/configuration.nix.
#
# Assumes Nix was installed with the upstream installer (nixos.org), so
# nix-darwin manages the daemon. Determinate Nix runs its own daemon and
# needs `nix.enable = false` instead, which a host can set.

{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    curl
    git
    vim
  ];

  nix = {
    # Same policy as system/configuration.nix; see the reasoning there.
    # nix-darwin schedules through launchd, so these take a calendar
    # interval where NixOS takes a systemd `dates`.
    gc = {
      automatic = true;
      interval = { Weekday = 0; Hour = 3; Minute = 15; };
      options = "--delete-older-than 7d";
    };

    optimise = {
      automatic = true;
      interval = { Weekday = 0; Hour = 4; Minute = 15; };
    };

    package = pkgs.nixVersions.latest;

    settings = {
      experimental-features = [ "nix-command" "flakes" ];
      warn-dirty = true;

      # Lets the admin group use the extra substituters below and copy
      # unsigned paths in (`nix copy --to ssh-ng://<mac>`). On NixOS the
      # wheel group gets this by default; macOS's equivalent is admin.
      # root is already in nix-darwin's default and merges in.
      trusted-users = [ "@admin" ];

      # numtide's cache for the llm-agents packages — the same cache and
      # key as system/configuration.nix, which explains why.
      extra-substituters = [ "https://cache.numtide.com" ];
      extra-trusted-public-keys = [
        "niks3.numtide.com-1:DTx8wZduET09hRmMtKdQDxNNthLQETkc/yaX7M4qK0g="
      ];
    };
  };

  # Writes /etc/zshrc so nix is on PATH for login shells. zsh is the
  # macOS default shell; the installer's own hook is what nix-darwin
  # replaces.
  programs.zsh.enable = true;

  # A wrapper's bootstrap installs these two files before the first
  # activation (see darwin/bootstrap/). nix-darwin refuses to replace
  # /etc files whose contents it does not recognise, so declare them.
  # Hashed from the files themselves, so the two cannot drift.
  environment.etc."nix/nix.conf".knownSha256Hashes = [
    (builtins.hashFile "sha256" ./bootstrap/nix.conf)
  ];
  environment.etc."zshenv".knownSha256Hashes = [
    (builtins.hashFile "sha256" ./bootstrap/zshenv)
  ];

  # nix-darwin's schema version, not macOS's. Fixed at install time like
  # NixOS's; a host overrides it only when adopting a new one on purpose.
  system.stateVersion = 6;
}
