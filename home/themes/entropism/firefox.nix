# entropism browser chrome: square, flat, 1px.
#
# cybr ships a 24k userChrome plus a force-installed sidebar extension.
# This is deliberately the opposite: a short stylesheet that removes
# rounding and gradient from Firefox's own chrome and repaints it in the
# resolved roles. No extensions, no vendored CSS, nothing to keep in
# sync with upstream.
#
# Page content is not touched -- websites are not ours to restyle.
{
  config,
  lib,
  ...
}:

let
  cfg = config.themes.entropism;
  c = cfg.resolvedColors;
in
{
  config = lib.mkIf cfg.enable {
    programs.firefox = {
      enable = true;

      profiles.default = {
        id = 0;

        settings = {
          "toolkit.legacyUserProfileCustomizations.stylesheets" = true;
          "browser.startup.homepage" = "https://github.com";
          "signon.rememberSignons" = false;

          # Square everything Firefox rounds on its own, and stop it
          # animating chrome it does not need to.
          "browser.uidensity" = 1;
          "toolkit.cosmeticAnimations.enabled" = false;
        };

        userChrome = ''
          /* Generated from themes/entropism roles. No literals here. */

          :root {
            --entropism-bg: ${c.bg};
            --entropism-panel: ${c.panel};
            --entropism-border: ${c.border};
            --entropism-dim: ${c.dim};
            --entropism-fg: ${c.fg};
            --entropism-alert: ${c.alert};

            --toolbar-bgcolor: var(--entropism-panel) !important;
            --toolbar-color: var(--entropism-fg) !important;
            --tab-border-radius: 0 !important;
            --toolbarbutton-border-radius: 0 !important;
            --urlbar-min-height: 26px !important;
          }

          /* Chrome surfaces: flat panel, one hairline at the bottom. */
          #navigator-toolbox {
            background: var(--entropism-panel) !important;
            border-bottom: 1px solid var(--entropism-border) !important;
          }

          #TabsToolbar,
          #nav-bar,
          #PersonalToolbar {
            background: var(--entropism-panel) !important;
            border: none !important;
            box-shadow: none !important;
          }

          /* Tabs are labels, not buttons: square, flat, and the active
             one is inverted rather than lit. */
          .tabbrowser-tab .tab-background {
            border-radius: 0 !important;
            border: none !important;
            box-shadow: none !important;
            background: transparent !important;
          }

          .tabbrowser-tab[selected] .tab-background {
            background: var(--entropism-fg) !important;
          }

          .tabbrowser-tab[selected] .tab-label {
            color: var(--entropism-bg) !important;
          }

          .tabbrowser-tab:not([selected]) .tab-label {
            color: var(--entropism-dim) !important;
          }

          /* Address bar: a field with a border, nothing more. */
          #urlbar,
          #urlbar-background,
          #searchbar {
            border-radius: 0 !important;
            box-shadow: none !important;
            background: var(--entropism-bg) !important;
            border: 1px solid var(--entropism-border) !important;
          }

          #urlbar[focused] > #urlbar-background {
            border-color: var(--entropism-fg) !important;
          }

          #urlbar-input,
          #searchbar .searchbar-textbox {
            color: var(--entropism-fg) !important;
          }

          /* The expanded results list is the same list the launcher
             draws: flat rows, a border-toned selection band. */
          .urlbarView {
            background: var(--entropism-panel) !important;
            border: 1px solid var(--entropism-border) !important;
          }

          .urlbarView-row[selected],
          .urlbarView-row:hover {
            background: var(--entropism-border) !important;
            border-radius: 0 !important;
          }

          /* Menus and popups. */
          menupopup,
          panel {
            --panel-background: var(--entropism-panel) !important;
            --panel-color: var(--entropism-fg) !important;
            --panel-border-color: var(--entropism-border) !important;
            --panel-border-radius: 0 !important;
          }

          /* No glow on focus, no rounded buttons. */
          toolbarbutton .toolbarbutton-icon {
            border-radius: 0 !important;
            box-shadow: none !important;
          }

          /* Failure states are the only colour escalation. */
          #identity-box.notSecure #identity-icon,
          .urlbarView-row[type="switchtab"] .urlbarView-secondary {
            color: var(--entropism-alert) !important;
          }
        '';
      };
    };
  };
}
