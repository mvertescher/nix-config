# common home manager configuration

{ inputs, pkgs, ... }:

let
  imports = [
    ./cli
    ./desktop
    ./starship.nix
  ];

in {
  inherit imports;

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
}
