{ pkgs, ... }:

let
  # Pinned precompiled 0.47.2. This let-binding wins over the `with pkgs`
  # below, so it is this one and not nixpkgs' dprint that is installed;
  # it cannot move into `lib/overlays.nix` without shadowing nixpkgs'
  # (0.56.1) for every consumer. repo-rs and mpris-status did move there.
  dprint = pkgs.callPackage ../pkgs/dprint.nix { };
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
      antigravity-cli
      asciinema
      bazel # google build tool
      ccache
      clang
      # claude-code is deliberately not here. Wrapper flakes install it
      # per host instead, because hosts want different things from it: a
      # headless host runs it as a plain CLI, while a workstation may
      # also run always-on Remote Control sessions and want the
      # programs.claude-code module, whose wrapper derivation ships its
      # own bin/claude and collides with a package listed here.
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
      gh # github cli
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
      rclone # cloud storage sync (Google Drive, S3, etc.)
      ripgrep # better grep
      socat
      sops
      ssh-to-age
      sqlite
      tokei
      tree
      uhubctl
      unzip
      vultr-cli
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
