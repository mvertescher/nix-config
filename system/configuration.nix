# nixos configuration

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
    wget
  ];
 
  # Nix daemon config
  nix = {
    gc = {
      automatic = true;
      dates = "weekly";
      options = "--delete-older-than 7d";
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
  };
}
