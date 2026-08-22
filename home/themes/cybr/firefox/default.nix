{ pkgs, config, ... }:

let
  # Sidebery's gecko id (src/manifest.json upstream). It names both the
  # policy entry that installs the extension and the browser-extension-data
  # directory its storage.js lives in, so keep the two in sync.
  sideberyId = "{3c078156-979c-498b-8990-85f7987dd929}";

  # Upstream's CSS hardcodes the cybr red/black. Follow the active stylix
  # palette instead, the same way waybar/colors.css and the rofi theme do.
  sideberyCSS =
    builtins.replaceStrings
      [ "#F75049" "#030408" ]
      [ "#${config.lib.stylix.colors.base08}" "#${config.lib.stylix.colors.base00}" ]
      (builtins.readFile ./sideberry.css);
in
{
  stylix.targets.firefox.profileNames = [ "default" ];

  programs.firefox = {
    enable = true;

    policies = {
      ExtensionSettings = {
        ${sideberyId} = {
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

        # Sidebery keeps its custom styles in storage.local under the
        # top-level sidebarCSS key (src/types/storage.ts upstream), so the
        # theme can be applied declaratively instead of being pasted into
        # the Style Editor by hand. Home Manager writes this to
        # browser-extension-data/<id>/storage.js and flips
        # extensions.webextensions.ExtensionStorageIDB.enabled to false so
        # Firefox reads the JSON backend rather than IndexedDB.
        #
        # Trade-off: that file is a read-only store symlink, and it also
        # backs Sidebery's runtime state (tabsDataCache, snapshots,
        # favicons, expandedBookmarkFolders, UI settings). Those can no
        # longer persist across restarts — see home-manager issue #9211.
        extensions.settings.${sideberyId} = {
          force = true;
          settings = {
            sidebarCSS = sideberyCSS;
          };
        };
      };
    };
  };
}
