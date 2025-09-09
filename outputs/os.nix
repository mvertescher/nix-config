{ extraSystemConfig, inputs, system, pkgs, ... }:

let
  inherit (inputs.nixpkgs.lib) nixosSystem;
  inherit (pkgs) lib;

  hosts = [ "terra" ];

  modules' = [
    ../system/configuration.nix
    extraSystemConfig
  ];

  make = host: {
    ${host} = nixosSystem {
      inherit lib pkgs system;
      specialArgs = { inherit inputs; };
      modules = modules' ++ [
        ../system/host/${host}
        inputs.home-manager.nixosModules.home-manager {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.users.mverte = ../home/host/${host}.nix;
        }
      ];
    };
  };
in
lib.mergeAttrsList (map make hosts)
