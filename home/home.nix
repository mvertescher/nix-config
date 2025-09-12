# common home manager configuration

{ inputs, system, pkgs, ... }:

let
  imports = [
    ./cli
    ./desktop
  ];

in {
  inherit imports;

  programs.home-manager.enable = true;

  # notifications about home-manager news
  news.display = "silent";

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
