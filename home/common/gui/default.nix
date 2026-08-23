{ pkgs, ... }:

let
  orbitron = pkgs.callPackage ../pkgs/orbitron {};
  rajdhani-fontshare = pkgs.callPackage ../pkgs/rajdhani-fontshare {};
  entropism-ui-demo = pkgs.callPackage ../pkgs/entropism-ui {
    inherit rajdhani-fontshare;
  };
  cyberpunk-ui = pkgs.callPackage ../pkgs/cyberpunk-ui {
    inherit orbitron rajdhani-fontshare;
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
    cyberpunk-ui
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}
