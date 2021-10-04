# Rebuild with `home-manager` switch

{ pkgs, ... }:

let
  imports = [
    # ./firefox.nix
    ./alacritty.nix
    ./gdb.nix
    ./git.nix
    ./shell.nix
    ./tmux.nix
    ./vim.nix
    ./vscode.nix
  ];

  # TODO: Properly autodetect nvidia version
  nixGL = pkgs.callPackage "${builtins.fetchTarball {
    url = https://github.com/guibou/nixGL/archive/7d6bc1b21316bab6cf4a6520c2639a11c25a220e.tar.gz;
    sha256 = "02y38zmdplk7a9ihsxvnrzhhv7324mmf5g8hmxqizaid5k5ydpr3";
  }}/nixGL.nix" { nvidiaVersion = "440.82"; };

  cargo-index = pkgs.callPackage ./pkgs/cargo-index.nix {};
  form-rs = pkgs.callPackage ./pkgs/form-rs.nix {};

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
    zenith

    # Development
    binutils
    ccache
    conda
    dfeet
    docker
    gcc
    gcc-arm-embedded
    gdb
    gnumake
    jq
    libimobiledevice
    openocd
    openssl
    qemu
    sqlite
    wireshark

    # Nix
    niv
    nix-tree
    nixGL.nixGLIntel
    nixfmt
    nixops

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
    cargo
    cargo-index
    cargo-make
    form-rs
    rustc
    rustfmt
    svd2rust
    # rustup

    # Other
    flameshot
    google-chrome
    libreoffice
    openconnect
    qutebrowser
    stdenv
    xournal
    zathura
    meld
  ];
}
