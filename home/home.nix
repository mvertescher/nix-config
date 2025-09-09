# common home manager configuration

{ inputs, system, pkgs, ... }:

let
  imports = [
    ./cli
    ./desktop
  ];

  # rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
in {
  inherit imports;

  programs.home-manager.enable = true;

  # home.homeDirectory = if pkgs.stdenv.isDarwin then "/Users/mvertescher" else "/home/mvertescher";

  # notifications about home-manager news
  news.display = "silent";

  # nixpkgs.overlays = [
  #  rust-overlay
  #  (self: super: {
  #    rust-bin = rust-overlay.rust-bin.stable.latest.default;
  #  })
  # ];

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
