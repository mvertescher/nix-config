{ pkgs, ... }:

{
  imports = [
    ./alacritty.nix
    ./firefox.nix
    # ./neovim-ide.nix
    ./vscode.nix
  ];

  # All three come from this repo's overlay (see `lib/overlays.nix`), not
  # from a `callPackage` here: `home/themes/lib/era.nix` needs the same
  # cp-eras-ui for its bar unit, and two instantiations of one package
  # are two things free to drift.
  home.packages = with pkgs; [
    # etcher
    orbitron-vf
    rajdhani-fontshare
    wireshark

    # Chat
    # slack
    # zoom-us
  ] ++ lib.optionals (stdenv.isLinux) [
    # google-chrome
    libreoffice
    # `entropism-ui` was the one-era predecessor and is gone: cp-eras-ui
    # wears entropism as one of four, and its login, mailbox, store and
    # dashboard screens replace that crate's.
    cp-eras-ui
    obs-studio
    qutebrowser
    spotify
    vlc
  ];
}
