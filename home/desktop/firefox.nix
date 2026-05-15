{ pkgs, config, ... }:

{
  stylix.targets.firefox.profileNames = [ "default" ];

  programs.firefox = {
    enable = true;
    configPath = "${config.xdg.configHome}/mozilla/firefox";
    # extensions = with pkgs.nur.repos.rycee.firefox-addons; [
    #   lastpass-password-manager
    #   ublock-origin
    # ];

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
