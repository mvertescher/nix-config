{ extraSystemConfig, extraHomeConfig ? { }, inputs, pkgs, ... }:

let
  inherit (inputs.nixpkgs.lib) nixosSystem;
  inherit (pkgs) lib;

  privateHome = import ../lib/private-config.nix { inherit lib; } extraHomeConfig;

  hosts = [ "terra" "server" ];

  modules' = [
    ../system/configuration.nix
    extraSystemConfig
  ];

  make = host: {
    ${host} = nixosSystem {
      inherit lib pkgs;
      specialArgs = { inherit inputs; };
      modules = modules' ++ [
        ../system/host/${host}
        inputs.home-manager.nixosModules.home-manager {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.backupFileExtension = "backup";
          home-manager.users.mverte.imports =
            [ ../home/host/${host}.nix ] ++ privateHome host;
        }
        inputs.stylix.nixosModules.stylix
      ];
    };
  };
in
lib.mergeAttrsList (map make hosts)
