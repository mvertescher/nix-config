{ config, lib, pkgs, ... }:

let
  wellKnownWallpapers = {
    shibuya = "0r9d7bx74wy6hml4rjsbxadph183fj3r2sn39h9hi54wmdaxhx9p";
    akihabara = "1d93p8lq5n0gzr4bswg5vn5yby1mjgn9iannr4jj7lg5am698dd0";
    roppongi = "0papw2l73mzma2528fcyfvy52dm4mzsg3s30yw1x4f71dfixpb56";
    shinjuku = "1917mxf5r0kxnbkpwv6dl9gxjxhgay7nv6wjry0c7rv06k622112";

    # Verified SHA256 hashes for the rest of the cybrpapers catalog desktop files
    asakusa = "1pjg0mdg54bijqhg7r4dlnnhy82nblrmz27r2l5dk6hiy6039s1p";
    chiyoda = "0k2p3cgc26yifmyrjawmjklpfrjn3wycsc4lhd3i8jqnhpvbnqgw";
    harajuku = "08kc119rkwvcrq9j13lvvrxi77mgp45j9fx5fj1lwcs6bnfdg1ka";
    ikebukuro = "13cnl471rsfcqsa3hzq2p0pi93dwvf3jymrqsqiy1zzi151vfg8k";
    minato = "0z5k99mgnnlkk112j5a3yg2ap2chssfsp73fqh15jqzq1drszd17";
    samurai = "1jkcfgjzm04lyw4cq71zm752kwk4s5hiri2y2sshp8xxj39wlfi9";
    taito = "053mh2rha9h45lrfnpmyyp6p3f4f86il3h1cs6jsyjzixv1nma8i";
    yoyogi = "016ljpjxjar0xb83nlw9fa7yripnfwa890lfhchhlxyxm54b0k9n";
  };

  cfg = config.custom.wallpaper;
  resolution = "3840x2160";

  selectedHash = if wellKnownWallpapers ? ${cfg.name}
                 then wellKnownWallpapers.${cfg.name}
                 else cfg.sha256;

  wallpaperUrl = "https://raw.githubusercontent.com/cybrcore/cybrpapers/main/wallpapers/${cfg.name}/${cfg.name}-${resolution}.jpg";

  wallpaperFile = pkgs.fetchurl {
    url = wallpaperUrl;
    sha256 = selectedHash;
  };

  # A declarative linkFarm containing all 12 wallpapers pre-cached in the Nix store
  wallpapersDir = pkgs.linkFarm "cybrpapers" (
    lib.mapAttrsToList (name: hash: {
      name = "${name}.jpg";
      path = pkgs.fetchurl {
        url = "https://raw.githubusercontent.com/cybrcore/cybrpapers/main/wallpapers/${name}/${name}-${resolution}.jpg";
        sha256 = hash;
      };
    }) wellKnownWallpapers
  );

  # Reusable, syntax-verified bash wallpaper rotation helper
  rotateWallpaperScript = pkgs.writeShellApplication {
    name = "hyprland-rotate-wallpaper";
    runtimeInputs = [
      pkgs.hyprland
      pkgs.coreutils
      pkgs.gnugrep
      pkgs.gawk
    ];
    text = ''
      WALLPAPERS_DIR="${wallpapersDir}"
      WALLPAPERS=(
        "shibuya" "akihabara" "roppongi" "shinjuku"
        "asakusa" "chiyoda" "harajuku" "ikebukuro"
        "minato" "samurai" "taito" "yoyogi"
      )

      # Query active wallpaper path via hyprctl IPC.
      #
      # `|| true` because writeShellApplication turns on errexit and
      # pipefail: with hyprpaper not running, hyprctl exits 1 and the whole
      # script died here, which made the "nothing active yet, start at
      # shibuya" branch below unreachable.
      ACTIVE_PATH=$(hyprctl hyprpaper "listactive" 2>/dev/null | head -n 1 | awk -F ': ' '{print $2}' || true)

      if [[ -z "$ACTIVE_PATH" ]]; then
        NEXT_WALLPAPER="shibuya"
      else
        ACTIVE_FILE=$(basename "$ACTIVE_PATH")
        ACTIVE_NAME="''${ACTIVE_FILE%.jpg}"
        # Strip the front hash part (e.g. yfadwr7iz6wi4ax6ps2223aw5v9ic080-)
        ACTIVE_NAME="''${ACTIVE_NAME#*-}"
        # Strip the back resolution part (e.g. -3840x2160)
        ACTIVE_NAME="''${ACTIVE_NAME%-*}"

        ACTIVE_INDEX=-1
        for i in "''${!WALLPAPERS[@]}"; do
          if [[ "''${WALLPAPERS[i]}" == "$ACTIVE_NAME" ]]; then
            ACTIVE_INDEX=$i
            break
          fi
        done

        NEXT_INDEX=$(( (ACTIVE_INDEX + 1) % ''${#WALLPAPERS[@]} ))
        NEXT_WALLPAPER="''${WALLPAPERS[NEXT_INDEX]}"
      fi

      echo "Rotating to wallpaper: $NEXT_WALLPAPER"

      # Query all active monitors dynamically
      MONITORS=$(hyprctl monitors | grep "Monitor" | awk '{print $2}')

      # Do NOT add a `hyprctl hyprpaper preload` in front of this, however
      # much the pre-0.8 hyprpaper documentation suggests it. hyprpaper
      # 0.8.0 (2025-12-29) was rewritten onto hyprtoolkit/hyprwire and the
      # whole preload/unload/listloaded vocabulary went with it: hyprctl
      # 0.55 accepts exactly two hyprpaper requests, `wallpaper mon,path`
      # and `listactive`, and answers anything else with "invalid
      # hyprpaper request" and exit 1. Under errexit that would abort the
      # script before it ever set a wallpaper. `wallpaper` loads the image
      # itself now, so there is nothing to preload and nothing to unload.
      #
      # hyprctl reports errors on stdout, not stderr, so this stays
      # unredirected: from a keybind it lands in the hyprland log, which is
      # the only place anyone would go looking.
      for mon in $MONITORS; do
        hyprctl hyprpaper "wallpaper $mon,$WALLPAPERS_DIR/$NEXT_WALLPAPER.jpg"
      done
    '';
  };
in
{


  config = lib.mkIf cfg.enable {
    custom.wallpaper.file = wallpaperFile;

    # home-manager's services.hyprpaper adds no package of its own -- the
    # unit runs a store path directly -- so the binary still has to be
    # installed here or `hyprpaper -c` is unavailable for debugging.
    home.packages = [
      pkgs.hyprpaper
      rotateWallpaperScript
    ];

    stylix.targets.hyprpaper.enable = lib.mkForce false;

    # Expose the whole cybrpapers set under the path a wallpaper picker can
    # discover at runtime. hyprpaper.conf only names the single image the
    # session boots with, and the rotation helper has the linkFarm baked in
    # at build time, so rofi/scripts/wallpaper had nothing to enumerate.
    xdg.configFile."hypr/walls".source = wallpapersDir;

    # Launched by its systemd user unit rather than hyprland's exec-once,
    # and configured through home-manager's `settings` rather than a
    # hand-written hyprpaper.conf. See ../lib/era.nix for the reasoning.
    # The practical gain beyond surviving a live theme switch is that
    # home-manager puts the rendered config in the unit's
    # X-Restart-Triggers, so changing the wallpaper restarts the daemon
    # instead of leaving it showing the old image.
    #
    # No `preload` key: hyprpaper 0.8 dropped the keyword along with the
    # rest of the preload machinery, and a `wallpaper` entry loads its own
    # path. hyprlang ignores the stale key rather than erroring, which is
    # what made the rotation helper look like it was missing a step.
    #
    # `ipc = true`, unlike the generated eras: the rotation helper drives
    # `hyprctl hyprpaper wallpaper`, and that needs the socket.
    # Each wallpaper is an attrset, which home-manager renders as a
    # `wallpaper { }` block. The string form -- `wallpaper = ",path"` --
    # renders as a flat `wallpaper=,path` line, which hyprpaper 0.8's
    # config parser does not accept: it logs "Monitor DP-3 has no target:
    # no wp will be created" and paints nothing. Confirmed by running
    # `hyprpaper -c` against both shapes; the flat form still works over
    # IPC, which is why the rotation helper is unaffected and why this is
    # easy to miss.
    services.hyprpaper = {
      enable = true;
      settings = {
        wallpaper =
          if cfg.monitors == [ ] then
            [ { monitor = "*"; path = "${wallpaperFile}"; } ]
          else
            map (mon: { monitor = mon; path = "${wallpaperFile}"; }) cfg.monitors;
        ipc = true;
        splash = false;
      };
    };
  };
}