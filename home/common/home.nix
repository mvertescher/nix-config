# common home manager configuration

{ pkgs, lib, ... }:

let
  imports = [
    ./cli
    ./gui
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

    # Stylix's KDE target prepends its store path to XDG_CONFIG_DIRS. When
    # the variable was unset, that drops the spec default (/etc/xdg), which
    # breaks xdg-user-dirs-update ("No default user directories"), autostart
    # entries and menus. Re-append /etc/xdg when it is missing.
    home.sessionVariablesExtra = lib.optionalString pkgs.stdenv.isLinux ''
      case ":''${XDG_CONFIG_DIRS:-}:" in
        *:/etc/xdg:*) ;;
        *) export XDG_CONFIG_DIRS="''${XDG_CONFIG_DIRS:+''${XDG_CONFIG_DIRS}:}/etc/xdg" ;;
      esac
    '';

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
    ] ++ lib.optionals stdenv.isLinux [
      nixgl.nixGLIntel
      zenith
    ] ++ lib.optionals stdenv.isDarwin [
      m-cli
    ];

    # Enable dynamic base16 file-type coloring for eza listings!
    stylix.targets.vivid.enable = true;
  };
}
