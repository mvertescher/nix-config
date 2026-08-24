{ pkgs, ... }:

let
  orbitron = pkgs.callPackage ../pkgs/orbitron {};
  rajdhani-fontshare = pkgs.callPackage ../pkgs/rajdhani-fontshare {};
  # `entropism-ui` was the one-era predecessor and is gone: cyberpunk-ui
  # wears entropism as one of four, and its login, mailbox, store and
  # dashboard screens replace that crate's.
  cyberpunk-ui = pkgs.callPackage ../pkgs/cyberpunk-ui {
    inherit orbitron rajdhani-fontshare;
  };
in
{
  imports = [
    ./alacritty.nix
    ./firefox.nix
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
    # google-chrome
    libreoffice
    cyberpunk-ui
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}
