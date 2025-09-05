{ pkgs, ... }:

{
  imports = [
    # ./firefox.nix
    ./alacritty.nix
    ./vscode.nix
  ];

  home.packages = with pkgs; [
    # etcher
    wireshark

    # Chat
    # slack
    # zoom-us
  ] ++ lib.optionals (stdenv.isLinux) [
    google-chrome
    libreoffice
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}