# general nixos configuration

{ config, pkgs, ... }:

let

in
{
  networking = {
    networkmanager = {
      enable = false;
    };
  };

  environment.systemPackages = with pkgs; [
    curl
    git
    vim
  ];

  # nix daemon config
  nix = {
    gc = {
      automatic = true;
      dates = "weekly";
      # --delete-older-than rather than --max-freed on purpose: it keeps
      # a predictable window of generations to roll back to, where a
      # size-based policy can leave you with none on a busy week.
      options = "--delete-older-than 7d";
    };

    # Hardlink identical files in the store. GC controls how much the
    # store holds; this controls how much of it is duplicated, and they
    # are independent -- terra was collecting weekly and still carrying
    # every copy of every shared file.
    #
    # The scheduled job rather than settings.auto-optimise-store: the
    # latter dedups on every path write, which taxes each build to save
    # space nobody is short of. Once a week off-peak is the right trade
    # on both a workstation and a small VPS.
    #
    # Safe alongside server's keep-outputs: this only replaces identical
    # files with hardlinks, it never deletes a store path.
    optimise = {
      automatic = true;
      dates = [ "weekly" ];
    };

    # Flakes settings
    package = pkgs.nixVersions.latest;

    settings = {
      experimental-features = [ "nix-command" "flakes" ];
      warn-dirty = true;

      # numtide's cache, for the llm-agents packages: claude-code,
      # antigravity-cli and codex (lib/overlays.nix). They track
      # upstream closely, which is why they come from that input
      # rather than the nixpkgs pin -- and codex is a Rust workspace
      # that takes tens of minutes to build, so without this every
      # move of the input pays a full compile on every host.
      #
      # `extra-` rather than plain: this adds to cache.nixos.org, it
      # does not replace it. The key is the one llm-agents.nix
      # declares in its own nixConfig (and its README), which is the
      # same trust the flake would ask for interactively with
      # --accept-flake-config; putting it here means it is a decision
      # recorded in the config rather than a prompt answered per
      # invocation.
      extra-substituters = [ "https://cache.numtide.com" ];
      extra-trusted-public-keys = [
        "niks3.numtide.com-1:DTx8wZduET09hRmMtKdQDxNNthLQETkc/yaX7M4qK0g="
      ];
    };
  };

  services = {
    sshd.enable = true;
  };

  users.users.mverte = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    initialPassword = "mverte";
  };
}
