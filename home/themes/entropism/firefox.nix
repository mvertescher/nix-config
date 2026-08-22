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
  pkgs,
  ...
}:

let
  cfg = config.themes.entropism;
  c = cfg.resolvedColors;

  # Everything the chrome is generated from. Hashing the inputs rather
  # than the stylesheet means the stamp changes exactly when the theme
  # does -- a reworded comment in the CSS will not trigger a restart.
  themeStamp = builtins.hashString "sha256" (
    builtins.toJSON {
      inherit (cfg) variant;
      colors = c;
      font = cfg.uiFont.name;
    }
  );

  # Activation runs with an empty environment - no ambient PATH - so every
  # tool it calls has to be named by store path.
  hyprctl = "${config.wayland.windowManager.hyprland.package}/bin/hyprctl";
  pgrep = "${pkgs.procps}/bin/pgrep";
  sleep = "${pkgs.coreutils}/bin/sleep";
  mkdir = "${pkgs.coreutils}/bin/mkdir";
  cat = "${pkgs.coreutils}/bin/cat";
  ls = "${pkgs.coreutils}/bin/ls";
  id = "${pkgs.coreutils}/bin/id";
  head = "${pkgs.coreutils}/bin/head";
  env = "${pkgs.coreutils}/bin/env";
in
{
  options.themes.entropism.firefox.restartOnActivation = lib.mkOption {
    type = lib.types.bool;
    default = true;
    description = ''
      Restart a running Firefox when the theme changes, since userChrome
      is only read at startup and an already-open window keeps rendering
      the previous theme.

      The restart is deliberately conservative: it fires only when the
      palette, variant or UI font actually changed, only when Firefox is
      already running, and it sends SIGTERM and waits so Firefox writes
      its session store and restores tabs on the way back up.
    '';
  };

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

    # userChrome is read once at startup, so switching themes leaves any
    # open window rendering the old one. Restart it -- but only on a real
    # change, and only if it is actually running, so an ordinary rebuild
    # never costs anyone their tabs.
    home.activation.entropismFirefoxRestart =
      lib.mkIf cfg.firefox.restartOnActivation
        (
          lib.hm.dag.entryAfter [ "writeBoundary" ] ''
            entropismStampDir="${config.xdg.stateHome}/entropism"
            entropismStamp="$entropismStampDir/firefox-theme"
            entropismWant="${themeStamp}"
            entropismHave="$(${cat} "$entropismStamp" 2>/dev/null || true)"

            if [ "$entropismHave" = "$entropismWant" ]; then
              verboseEcho "entropism: browser theme unchanged, leaving Firefox alone"
            else
              # Match on the launcher's full command line: the kernel
              # truncates comm to 15 characters, so the wrapped binary
              # shows up as ".firefox-wrappe" and pkill -x misses it.
              entropismPids="$(${pgrep} -f 'bin/firefox$' 2>/dev/null || true)"

              if [ -n "$entropismPids" ]; then
                verboseEcho "entropism: browser theme changed, restarting Firefox"

                # SIGTERM, not SIGKILL: Firefox flushes its session store
                # on the way out and restores the tabs when it comes back.
                $DRY_RUN_CMD kill -TERM $entropismPids 2>/dev/null || true

                entropismWaited=0
                while [ "$entropismWaited" -lt 20 ] \
                  && ${pgrep} -f 'bin/firefox$' >/dev/null 2>&1; do
                  ${sleep} 0.5
                  entropismWaited=$((entropismWaited + 1))
                done

                # Relaunch through the compositor, so the new process gets
                # a real Wayland environment rather than activation's.
                #
                # The signature is read off the runtime directory instead
                # of $HYPRLAND_INSTANCE_SIGNATURE: home-manager's unit runs
                # with an empty Environment=, so the variable is not there
                # to inherit even though the session is running.
                entropismSig="$(
                  ${ls} "/run/user/$(${id} -u)/hypr" 2>/dev/null | ${head} -1 || true
                )"

                if [ -n "$entropismSig" ]; then
                  $DRY_RUN_CMD ${env} "HYPRLAND_INSTANCE_SIGNATURE=$entropismSig" \
                    ${hyprctl} dispatch exec firefox >/dev/null 2>&1 || true
                else
                  verboseEcho "entropism: no hyprland session, not relaunching Firefox"
                fi
              else
                verboseEcho "entropism: Firefox not running, nothing to restart"
              fi

              $DRY_RUN_CMD ${mkdir} -p "$entropismStampDir"
              $DRY_RUN_CMD sh -c "printf '%s' '$entropismWant' > '$entropismStamp'"
            fi
          ''
        );
  };
}
