{ pkgs, ... }:

let
  orbitron = pkgs.callPackage ../pkgs/orbitron {};
  rajdhani-fontshare = pkgs.callPackage ../pkgs/rajdhani-fontshare {};
  entropism-ui-demo = pkgs.callPackage ../pkgs/entropism-ui {
    inherit rajdhani-fontshare;
  };
in
{
  imports = [
    ./alacritty.nix
    # ./neovim-ide.nix
    ./vscode.nix
  ];

  home.packages = with pkgs; [
    # etcher
    orbitron
    rajdhani-fontshare
    wireshark

    # Chat
    # slack
    # zoom-us
  ] ++ lib.optionals (stdenv.isLinux) [
    entropism-ui-demo
    # google-chrome
    libreoffice
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}
