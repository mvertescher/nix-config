# Restart a running Firefox when the browser theme changes.
#
# userChrome.css is read once at startup, so a theme switch leaves an
# open window wearing the previous era's chrome until the user happens
# to restart it themselves. This closes that gap.
#
# Extracted from lib/era.nix so the vendored `cybr` theme can have it
# too. cybr is not generated from roles and so does not go through the
# era builder, which is exactly why it was the one theme still leaving
# stale chrome behind.
#
#   imports = [ (import ../lib/browser-restart.nix {
#     inherit lib pkgs config;
#     name = "Cybr";
#     stamp = builtins.hashString "sha256" (builtins.toJSON { ... });
#   }) ];
#
# `stamp` should hash whatever the chrome is generated from, so it moves
# exactly when the browser's appearance does and not on every rebuild.
{
  lib,
  pkgs,
  config,
  # Theme name, used only in activation output.
  name,
  # Opaque marker for "this is the look Firefox should currently have".
  stamp,
}:

let
  bin = p: n: "${p}/bin/${n}";
  hyprctl = bin config.wayland.windowManager.hyprland.package "hyprctl";
  coreutil = bin pkgs.coreutils;
in
{
  home.activation.themeBrowserRestart = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    themeStampDir="${config.xdg.stateHome}/themes"
    themeStamp="$themeStampDir/browser-theme"
    themeWant="${stamp}"
    themeHave="$(${coreutil "cat"} "$themeStamp" 2>/dev/null || true)"

    if [ "$themeHave" = "$themeWant" ]; then
      verboseEcho "${name}: browser theme unchanged, leaving Firefox alone"
    else
      # The kernel truncates comm to 15 characters, so the wrapped
      # binary is ".firefox-wrappe" and pkill -x misses it; match the
      # launcher's command line instead.
      themePids="$(${bin pkgs.procps "pgrep"} -f 'bin/firefox$' 2>/dev/null || true)"

      if [ -n "$themePids" ]; then
        verboseEcho "${name}: browser theme changed, restarting Firefox"

        # SIGTERM, so Firefox writes its session store and restores
        # tabs on the way back up.
        $DRY_RUN_CMD kill -TERM $themePids 2>/dev/null || true

        themeWaited=0
        while [ "$themeWaited" -lt 20 ] \
          && ${bin pkgs.procps "pgrep"} -f 'bin/firefox$' >/dev/null 2>&1; do
          ${coreutil "sleep"} 0.5
          themeWaited=$((themeWaited + 1))
        done

        # home-manager's unit runs with an empty Environment=, so the
        # compositor signature is read off the runtime directory rather
        # than inherited.
        themeSig="$(
          ${coreutil "ls"} "/run/user/$(${coreutil "id"} -u)/hypr" 2>/dev/null \
            | ${coreutil "head"} -1 || true
        )"

        if [ -n "$themeSig" ]; then
          $DRY_RUN_CMD ${coreutil "env"} "HYPRLAND_INSTANCE_SIGNATURE=$themeSig" \
            ${hyprctl} dispatch exec firefox >/dev/null 2>&1 || true
        else
          verboseEcho "${name}: no hyprland session, not relaunching Firefox"
        fi
      else
        verboseEcho "${name}: Firefox not running, nothing to restart"
      fi

      $DRY_RUN_CMD ${coreutil "mkdir"} -p "$themeStampDir"
      $DRY_RUN_CMD sh -c "printf '%s' '$themeWant' > '$themeStamp'"
    fi
  '';
}
