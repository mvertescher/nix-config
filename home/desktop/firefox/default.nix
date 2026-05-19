{ pkgs, config, ... }:

{
  stylix.targets.firefox.profileNames = [ "default" ];

  programs.firefox = {
    enable = true;
    configPath = "${config.xdg.configHome}/mozilla/firefox";

    profiles = {
      default = {
        id = 0;
        settings = {
          "browser.startup.homepage" = "https://github.com";
          "signon.rememberSignons" = false;

          # Required for cybr-firefox (lucid theme)
          "toolkit.legacyUserProfileCustomizations.stylesheets" = true;
          "browser.tabs.allow_transparent_browser" = true;
          "widget.transparent-windows" = true;
        };

        userChrome = builtins.readFile ./userChrome.css;
      };
    };
  };

  # Deploy sideberry.css to config folder so you can easily copy it to Sidebery Style Editor
  xdg.configFile."firefox/sideberry.css".source = ./sideberry.css;
}
