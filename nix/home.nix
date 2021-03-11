# Rebuild with `home-manager` switch

{ pkgs, ... }:

let
  imports = [
    ./alacritty.nix
    # ./firefox.nix
    ./git.nix
    ./shell.nix
    ./vim.nix
    ./vscode.nix
  ];
in {
  inherit imports;

  programs.home-manager.enable = true;
  nixpkgs.config.allowUnfree = true;

  home.packages = with pkgs; [
    # CLI
    asciinema
    bat
    cmatrix
    curl
    direnv
    du-dust
    exa
    fd
    file
    htop
    neofetch
    ripgrep
    tokei
    tree
    unzip
    whois

    # Development
    binutils
    conda
    dfeet
    docker
    gcc
    gcc-arm-embedded
    gdb
    gnumake
    libimobiledevice
    openocd
    openssl
    qemu
    sqlite
    wireshark

    # Nix
    niv
    nix-tree
    nixfmt

    # Node.js
    nodePackages.node-pre-gyp
    nodePackages.node2nix

    # Chat
    slack
    zoom-us

    # Media
    ffmpeg
    spotify
    vlc

    # Rust
    rustup

    # Other
    flameshot
    qutebrowser
    stdenv
    zathura
  ];
}
