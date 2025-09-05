{ pkgs, ... }:

let
  cargo-index = pkgs.callPackage ../pkgs/cargo-index.nix {};
  cargo-local-registry = pkgs.callPackage ../pkgs/cargo-local-registry.nix {};
  form-rs = pkgs.callPackage ../pkgs/form-rs.nix {};
  puncover = pkgs.python311Packages.callPackage ../pkgs/puncover {};
in
{
  imports = [
    ./gdb.nix
    ./git.nix
    ./shell.nix
    ./tmux.nix
    ./vim.nix
  ];

  home.packages = with pkgs; [
    # binutils
    # gcc
    # gcc-arm-embedded
    # puncover
    android-tools
    asciinema
    bat # better cat
    bazel # google build tool
    ccache
    clang
    cmatrix
    curl
    direnv
    docker
    du-dust
    eza # better ls
    fd # better find
    ffmpeg # media converter
    file
    gdb
    gitRepo
    gnumake
    google-cloud-sdk
    graphviz
    htop
    jq # json tool
    libimobiledevice
    libusb1
    minicom # serial console
    neofetch
    nushell
    openconnect # vpn client
    openocd
    openssl
    qemu
    ripgrep # better grep
    socat
    sqlite
    tokei
    tree
    unzip
    whois
    writedisk
  ] ++ lib.optionals (stdenv.isLinux) [
    conda
    flameshot
    linux-router
    lxi-tools
    woeusb
  ];

  # top replacement
  programs.bottom.enable = true;
}
