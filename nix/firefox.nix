{ pkgs, ... }:

{
  nixpkgs.config.packageOverrides = pkgs: {
    nur = import (builtins.fetchTarball
      "https://github.com/nix-community/NUR/archive/master.tar.gz") {
        inherit pkgs;
      };
  };

  programs.firefox = {
    enable = true;
    extensions = with pkgs.nur.repos.rycee.firefox-addons; [
      lastpass-password-manager
      ublock-origin
    ];
    
    profiles = {
      default = {
        id = 0;
        settings = {
          "browser.startup.homepage" = "https://github.com";
          "signon.rememberSignons" = false;
        };
      };
    };
  };
}
