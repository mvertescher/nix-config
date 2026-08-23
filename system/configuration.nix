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
