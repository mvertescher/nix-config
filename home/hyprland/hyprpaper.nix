{ config, lib, pkgs, ... }:

let
  wellKnownWallpapers = {
    shibuya = "0r9d7bx74wy6hml4rjsbxadph183fj3r2sn39h9hi54wmdaxhx9p";
    akihabara = "1d93p8lq5n0gzr4bswg5vn5yby1mjgn9iannr4jj7lg5am698dd0";
    roppongi = "0papw2l73mzma2528fcyfvy52dm4mzsg3s30yw1x4f71dfixpb56";
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
in
{
  options.custom.wallpaper = {
    enable = lib.mkEnableOption "Custom cybrpapers wallpaper";
    name = lib.mkOption {
      type = lib.types.str;
      default = "roppongi";
      description = "Name of the wallpaper from cybrpapers repo (e.g. shibuya, akihabara, roppongi)";
    };
    sha256 = lib.mkOption {
      type = lib.types.str;
      default = "";
      description = "SHA256 hash of the wallpaper. Only required if using a custom wallpaper not pre-configured.";
    };
    monitors = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      description = "List of monitor names to apply the wallpaper to. Leave empty to apply to all monitors.";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ pkgs.hyprpaper ];

    stylix.targets.hyprpaper.enable = lib.mkForce false;
    services.hyprpaper.enable = lib.mkForce false;

    xdg.configFile."hypr/hyprpaper.conf".text = let
      wallpaperLines = if cfg.monitors == [ ]
                       then "wallpaper = ,${wallpaperFile}"
                       else lib.concatMapStringsSep "\n" (mon: "wallpaper = ${mon},${wallpaperFile}") cfg.monitors;
    in ''
      preload = ${wallpaperFile}
      ${wallpaperLines}
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