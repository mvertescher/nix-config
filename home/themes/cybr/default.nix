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

  programs.alacritty.settings.window.opacity = lib.mkForce 0.10;

  programs.alacritty.settings.colors.selection = {
    background = lib.mkForce "#${config.lib.stylix.colors.base0F}";
    text = lib.mkForce "#${config.lib.stylix.colors.base00}";
  };
}
