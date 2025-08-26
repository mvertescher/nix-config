# Rebuild with `home-manager switch`

{ pkgs, ... }:

let
  # work = builtins.getEnv "PWD" + "/work/home.nix";

  imports = [
    # ./firefox.nix
    ./alacritty.nix
    ./gdb.nix
    ./git.nix
    ./shell.nix
    ./tmux.nix
    ./vim.nix
    ./vscode.nix

    # Add work configuration
    # work
  ];

  # TODO: Properly autodetect nvidia version
  # nixGL = pkgs.callPackage "${builtins.fetchTarball {
  #  url = https://github.com/guibou/nixGL/archive/7d6bc1b21316bab6cf4a6520c2639a11c25a220e.tar.gz;
  #  sha256 = "02y38zmdplk7a9ihsxvnrzhhv7324mmf5g8hmxqizaid5k5ydpr3";
  # }}/nixGL.nix" { nvidiaVersion = "440.82"; };

  cargo-index = pkgs.callPackage ./pkgs/cargo-index.nix {};
  cargo-local-registry = pkgs.callPackage ./pkgs/cargo-local-registry.nix {};
  form-rs = pkgs.callPackage ./pkgs/form-rs.nix {};
  puncover = pkgs.python311Packages.callPackage ./pkgs/puncover {};

  # rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));

in {
  inherit imports;

  home.stateVersion = "22.05";
  home.username = "mvertescher";
  home.homeDirectory = if pkgs.stdenv.isDarwin then "/Users/mvertescher" else "/home/mvertescher";

  programs.home-manager.enable = true;

  nixpkgs.overlays = [
  #  rust-overlay
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
    # exa
    fd
    file
    graphviz
    htop
    neofetch
    # puncover
    ripgrep
    socat
    tokei
    tree
    unzip
    whois

    # Development
    # binutils
    bazel
    ccache
    clang
    docker
    # etcher
    # gcc
    # gcc-arm-embedded
    gdb
    gitRepo
    gnumake
    google-cloud-sdk
    jq
    libimobiledevice
    libusb1
    minicom
    openocd
    openssl
    qemu
    sqlite
    wireshark

    # Go
    delve
    go
    go-outline
    go-tools
    gopls

    # Nix
    niv
    nix-tree
    nixfmt
    # nixops

    # Node.js
    nodePackages.node-pre-gyp
    nodePackages.node2nix

    # Chat
    # slack
    # zoom-us

    # Media
    ffmpeg

    # Rust
    # cargo-index
    # cargo-local-registry
    cargo-audit
    cargo-bloat
    cargo-expand
    cargo-fuzz
    # cargo-geiger
    cargo-make
    cargo-raze
    cargo-tarpaulin
    # form-rs
    rust-bindgen
    rustup
    twiggy
    # rust-bin.stable.latest.default

    # cargo
    # rustc
    # rustfmt
    # svd2rust
    # rustup

    # Other
    openconnect
    stdenv
    # xournal
    zathura
    meld
  ] ++ lib.optionals (stdenv.isLinux) [
    conda
    # dfeet
    flameshot
    google-chrome
    libreoffice
    linux-router
    lxi-tools
    # nixGL.nixGLIntel
    obs-studio
    qutebrowser
    spotify
    vlc
    woeusb
    zenith
  ] ++ lib.optionals (stdenv.isDarwin) [
    m-cli
  ];
}
