{ pkgs, config, ... }:

{
  stylix.targets.firefox.profileNames = [ "default" ];

  programs.firefox = {
    enable = true;
    configPath = "${config.xdg.configHome}/mozilla/firefox";

    policies = {
      ExtensionSettings = {
        "{3c078156-979c-498b-8990-85f7987dd929}" = {
          installation_mode = "normal_installed";
          install_url = "https://addons.mozilla.org/firefox/downloads/latest/sidebery/latest.xpi";
        };
      };
    };

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
