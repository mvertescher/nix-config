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

      # Query active wallpaper path via hyprctl IPC
      ACTIVE_PATH=$(hyprctl hyprpaper "listactive" | head -n 1 | awk -F ': ' '{print $2}')

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

      # Instantly reload the wallpaper for each active monitor using the quoted direct wallpaper command
      for mon in $MONITORS; do
        hyprctl hyprpaper "wallpaper $mon,$WALLPAPERS_DIR/$NEXT_WALLPAPER.jpg"
      done
    '';
  };
in
{


  config = lib.mkIf cfg.enable {
    custom.wallpaper.file = wallpaperFile;

    # Install both hyprpaper and our rotation script helper
    home.packages = [
      pkgs.hyprpaper
      rotateWallpaperScript
    ];

    stylix.targets.hyprpaper.enable = lib.mkForce false;
    services.hyprpaper.enable = lib.mkForce false;

    xdg.configFile."hypr/hyprpaper.conf".text = let
      wallpaperBlocks = if cfg.monitors == [ ]
                        then ''
                          wallpaper {
                              monitor = *
                              path = ${wallpaperFile}
                          }
                        ''
                        else lib.concatMapStringsSep "\n" (mon: ''
                          wallpaper {
                              monitor = ${mon}
                              path = ${wallpaperFile}
                          }
                        '') cfg.monitors;
    in ''
      preload = ${wallpaperFile}
      ${wallpaperBlocks}
      ipc = true
      splash = false
    '';

    wayland.windowManager.hyprland.settings = {
      exec-once = [
        "hyprpaper"
      ];
    };
  };
}