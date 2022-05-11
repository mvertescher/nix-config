# Rebuild with `home-manager switch`

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
  cargo-local-registry = pkgs.callPackage ./pkgs/cargo-local-registry.nix {};
  form-rs = pkgs.callPackage ./pkgs/form-rs.nix {};

  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));

in {
  inherit imports;

  programs.home-manager.enable = true;
  nixpkgs.config.allowUnfree = true;

  nixpkgs.overlays = [
    rust-overlay
  #  (self: super: {
  #    rust-bin = rust-overlay.rust-bin.stable.latest.default;
  #  })
  ];

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
    # binutils
    ccache
    clang
    conda
    dfeet
    docker
    # gcc
    gcc-arm-embedded
    gdb
    gitRepo
    gnumake
    jq
    libimobiledevice
    libusb1
    lxi-tools
    openocd
    openssl
    qemu
    sqlite
    wireshark
    etcher
    woeusb

    # Nix
    niv
    nix-tree
    nixGL.nixGLIntel
    nixfmt
    # nixops

    # Node.js
    nodePackages.node-pre-gyp
    nodePackages.node2nix

    # Chat
    slack
    # zoom-us

    # Media
    ffmpeg
    obs-studio
    spotify
    vlc

    # Rust
    cargo-index
    cargo-local-registry
    cargo-make
    form-rs
    rustup
    # rust-bin.stable.latest.default


    # cargo
    # rustc
    # rustfmt
    # svd2rust
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
