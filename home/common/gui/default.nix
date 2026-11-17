{ pkgs, ... }:

{
  imports = [
    ./alacritty.nix
    ./firefox.nix
    # ./neovim-ide.nix
    ./vscode.nix
  ];

  # cp-eras-ui and its fonts (orbitron-vf, rajdhani-fontshare) were
  # installed from here until 2026-11-16. They come from the `ui`
  # repository now, and a host that wants them installs them itself.
  home.packages = with pkgs; [
    # etcher
    wireshark

    # Chat
    # slack
    # zoom-us
  ] ++ lib.optionals stdenv.isLinux [
    # google-chrome
    libreoffice
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}
