{
  description = "Home Manager configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Needed on non-NixOS to solve GL application errors
    nixgl.url = "github:nix-community/nixGL";

    # Fast nix search client
    # nix-search = {
    #   url = github:diamondburned/nix-search;
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };

    # Another nix search client
    nix-search-cli = {
      url = "github:peterldowns/nix-search-cli";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    stylix = {
      url = "github:nix-community/stylix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    llm-agents = {
      url = "github:numtide/llm-agents.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    disko = {
      url = "github:nix-community/disko";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };


  outputs = inputs @ { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      # TODO: Support MacOS, maybe others
      # system = "x86_64-darwin";

      overlays = import ./lib/overlays.nix { inherit inputs; };

      pkgs = import inputs.nixpkgs {
        inherit overlays system;
        config.allowUnfree = true;
        config.allowUnfreePredicate = (_: true);
      };
    in
    {
      # `out.pkgs` is a frozen reach-through: the home-manager-only
      # wrapper gets its overlaid pkgs (and `pkgs.builders.mkHome`)
      # here. Keep it stable.
      out = { inherit pkgs overlays; };

      # Library entry points for wrapper flakes. This repo defines no
      # configurations of its own: wrappers pass their host sets to
      # mkNixos, and home-manager wrappers call mkHome (either here or
      # via the frozen `pkgs.builders.mkHome` shim).
      lib = {
        mkNixos = import ./lib/mkNixos.nix { inherit inputs overlays; };
        mkHome = { extraHomeConfig ? { } }:
          import ./lib/mkHome.nix { inherit extraHomeConfig inputs pkgs; };
      };

      # Installer ISO with SSH keys baked in, for unattended provisioning
      # via scripts/provision-server.sh. Plain nixpkgs, no overlays needed.
      packages.${system}.installer-iso =
        (nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [ ./system/installer.nix ];
        }).config.system.build.isoImage;
    };
}
