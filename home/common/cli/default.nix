{ pkgs, ... }:

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
      # Note: repo-rs was removed because upstream repository https://github.com/sunbeamdotpt/repo-rs was deleted.
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
