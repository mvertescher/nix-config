# common home manager configuration

{ inputs, pkgs, lib, config, ... }:

let
  imports = [
    ./cli
    ./desktop
  ];
in
{
  inherit imports;

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
    file = lib.mkOption {
      type = lib.types.package;
      description = "Internal read-only reference to the downloaded wallpaper package.";
    };
  };

  config = {
    programs.home-manager.enable = true;

    # notifications about home-manager news
    news.display = "silent";

    gtk.gtk4.theme = null;

    stylix.fonts = {
      monospace = {
        package = pkgs.nerd-fonts.geist-mono;
        name = "GeistMono Nerd Font";
      };
    };

    # Enable declarative vivid LS_COLORS generation!
    programs.vivid.enable = true;

    home.packages = with pkgs; [
      # Other
      stdenv
      # xournal
      zathura
      meld
    ] ++ lib.optionals (stdenv.isLinux) [
      nixgl.nixGLIntel
      zenith
    ] ++ lib.optionals (stdenv.isDarwin) [
      m-cli
    ];

    # Enable dynamic base16 file-type coloring for eza listings!
    stylix.targets.vivid.enable = true;
  };
}
