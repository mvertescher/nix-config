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

          # Both prefs still exist in Firefox 152 (checked against the
          # store copy of the browser, not just docs).
          #
          # allow_transparent_browser defaults to false and is read by the
          # front end (tabbrowser.js, browser-sidebar.js, webext-panels.js)
          # to put transparent="true" on the <browser>. Without it the
          # content area paints an opaque background over the chrome, so
          # userChrome's transparency stops at the viewport. Platform
          # independent -- this is the one that matters here.
          #
          # widget.transparent-windows is a StaticPref that already
          # defaults to true, and is mirror: once, so it is only read at
          # window creation. widget/gtk/nsWindow.cpp passes it to
          # gtk_widget_set_app_paintable() on both the shell and the
          # container, which is the GTK/Wayland side of the effect. Setting
          # it here is therefore a no-op against the default (Firefox does
          # not even persist it to prefs.js) -- kept as documentation, and
          # as a guard in case the default flips.
          #
          # Neither pref produces a see-through *window* on its own: that
          # needs the compositor to composite it (hyprland's decoration
          # active_opacity/inactive_opacity, see home/common/hyprland) plus a
          # transparent chrome background from userChrome.css.
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
