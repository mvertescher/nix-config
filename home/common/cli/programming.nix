{
  home.packages = with pkgs; [
    # Go
    delve
    go
    go-outline
    go-tools
    gopls

    # Nix
    niv
    nix-search-cli
    nix-tree
    nixfmt
    # nixops

    # Node.js
    nodePackages.node-pre-gyp
    nodePackages.node2nix

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
    rust-cbindgen
    rustup
    twiggy
    # rust-bin.stable.latest.default

    # cargo
    # rustc
    # rustfmt
    # svd2rust
    # rustup
  ];
}