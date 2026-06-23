{ pkgs, ... }:

let
  cargo-index = pkgs.callPackage ../pkgs/cargo-index.nix { };
  cargo-local-registry = pkgs.callPackage ../pkgs/cargo-local-registry.nix { };
  dprint = pkgs.callPackage ../pkgs/dprint.nix { };
  form-rs = pkgs.callPackage ../pkgs/form-rs.nix { };
  puncover = pkgs.python311Packages.callPackage ../pkgs/puncover { };
  repo-rs = pkgs.callPackage ../pkgs/repo-rs.nix { };
in
{
  imports = [
    ./bat.nix
    ./gdb.nix
    ./git.nix
    #./helix.nix
    ./shell.nix
    ./tmux.nix
    ./vim.nix
  ];

  home.packages =
    with pkgs;
    [
      # binutils
      # gcc
      # gcc-arm-embedded
      # puncover
      android-tools
      asciinema
      bazel # google build tool
      ccache
      clang
      cmatrix
      curl
      direnv
      dprint
      docker
      dust
      eza # better ls
      fd # better find
      ffmpeg # media converter
      file
      gemini-cli
      gdb
      gitRepo
      # repo-rs is a Rust implementation of the Android repo tool.
      # It is significantly faster (10x+) for everyday query operations like:
      #   repo-rs status
      #   repo-rs diff
      #   repo-rs list
      # However, it contains upstream bugs (such as 'repo-rs info' crashing on path resolution),
      # so we co-install both standard Python 'gitRepo' (repo) and 'repo-rs'.
      repo-rs
      gnumake
      google-cloud-sdk
      graphviz
      htop
      jq # json tool
      libimobiledevice
      libusb1
      minicom # serial console
      # neofetch
      nixfmt-rfc-style
      nufmt
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
    ]
    ++ lib.optionals (stdenv.isLinux) [
      conda
      flameshot
      linux-router
      lxi-tools
      woeusb
    ];

  # top replacement
  programs.bottom.enable = true;
}
