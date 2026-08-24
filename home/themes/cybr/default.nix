{ pkgs, config, lib, ... }:

let
  theme = import ./colors/cybrcolors.nix;
  staticPixel = pkgs.runCommand "base0C-pixel.png" {
    color = "#${theme.base0C}";
  } "${lib.getExe' pkgs.imagemagick "convert"} xc:$color png32:$out";
in
{
  imports = [
    # Same Firefox restart the generated eras get. cybr was the one
    # theme still leaving stale chrome behind on a switch, purely
    # because it does not go through lib/era.nix.
    #
    # The stamp hashes the palette and the two stylesheets the chrome is
    # actually built from, so it moves when the browser's appearance
    # does rather than on every rebuild.
    (import ../lib/browser-restart.nix {
      inherit lib pkgs config;
      name = "Cybr";
      stamp = builtins.hashString "sha256" (builtins.toJSON {
        colors = theme;
        userChrome = builtins.hashString "sha256" (builtins.readFile ./firefox/userChrome.css);
        sidebery = builtins.hashString "sha256" (builtins.readFile ./firefox/sideberry.css);
      });
    })

    ./starship.nix
    ./firefox
    ./hyprlock.nix
    ./hyprpaper.nix
    ./rofi
    ./swaync.nix
    ./waybar
  ];

  stylix = {
    enable = true;
    base16Scheme = theme;
    image = lib.mkDefault staticPixel;
  };

  # Nothing else on this desktop named an icon theme. `stylix.icons` is
  # off, `gtk.iconTheme` was unset, and the only mention of Papirus lived
  # inside rofi's config.rasi -- which meant rofi found it and nothing
  # else did. Anything doing a freedesktop lookup fell through to
  # `hicolor` plus whatever an application ships for itself, and a
  # tray-style consumer had to be told the theme by hand on its command
  # line to see anything at all.
  #
  # `gtk.iconTheme` is the durable form: home-manager writes
  # `gtk-icon-theme-name` into both gtk-3.0/settings.ini and
  # gtk-4.0/settings.ini, which is where GTK and every non-GTK client
  # that bothers to look -- ours included -- go for the answer, and it
  # carries the package into home.packages so the name resolves.
  #
  # Named rather than derived from `config.stylix.polarity`, which is
  # `either` here because cybr never sets it: this palette is a fixed
  # dark one with no light variant, and the literal keeps it visibly in
  # step with the `icon-theme` line in rofi/cybr-rofi/config.rasi. The
  # generated eras do derive it -- see ../lib/era.nix -- because they
  # have light variants and a bar that draws tray icons.
  gtk.iconTheme = lib.mkDefault {
    name = "Papirus-Dark";
    package = pkgs.papirus-icon-theme;
  };

  programs.alacritty.settings.window.opacity = lib.mkForce 0.10;

  programs.alacritty.settings.colors.selection = {
    background = lib.mkForce "#${config.lib.stylix.colors.base0F}";
    text = lib.mkForce "#${config.lib.stylix.colors.base00}";
  };
}
