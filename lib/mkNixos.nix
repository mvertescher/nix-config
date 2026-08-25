# Host-set NixOS builder, exposed as `nix-config.lib.mkNixos`.
#
# This repo defines no hosts of its own: a wrapper flake passes the
# full host set and this builder wraps each host with the shared stack
# (system/configuration.nix, home-manager as a NixOS module, stylix).
#
#   mkNixos {
#     hosts = {
#       myhost = {
#         system = "x86_64-linux";        # optional, per-host (SBCs are aarch64)
#         user = "mverte";                # optional, per-host login user
#         modules = [ ./hosts/myhost ];   # host identity: hardware, disks, ...
#         homeModules = [ ./hosts/myhost/home.nix ];  # the user's HM imports
#       };
#     };
#     extraSystemConfig = { };            # optional module shared by all hosts
#     extraOverlays = [ ];                # optional, applied to all hosts
#   }
#
# `pkgs` is constructed here, per host, from this repo's pinned nixpkgs
# and overlays — wrappers never touch pkgs plumbing. `networking.hostName`
# defaults to the attr name.
{ inputs, overlays }:

{
  hosts,
  extraSystemConfig ? { },
  extraOverlays ? [ ],
}:

let
  hostPkgs = import ./host-pkgs.nix { inherit inputs overlays extraOverlays; };

  make = name: host:
    let
      pkgs = hostPkgs host;
      lib = pkgs.lib;
      # `mkHome` defaults this to "mvertescher" instead. Not an
      # oversight and not to be unified: both are frozen by consumers
      # whose call sites cannot be updated from here.
      user = host.user or "mverte";
    in
    inputs.nixpkgs.lib.nixosSystem {
      inherit lib pkgs;
      specialArgs = { inherit inputs; };
      # Host modules sit before the home-manager/stylix wiring: list
      # options merge in module order, so moving them changes generated
      # unit text (e.g. Wants=) even when the config is equivalent.
      modules = [
        ../system/configuration.nix
        { networking.hostName = lib.mkDefault name; }
        extraSystemConfig
      ] ++ (host.modules or [ ]) ++ [
        inputs.home-manager.nixosModules.home-manager {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.backupFileExtension = "backup";
          home-manager.users.${user}.imports = host.homeModules or [ ];
        }
        inputs.stylix.nixosModules.stylix
      ];
    };
in
inputs.nixpkgs.lib.mapAttrs make hosts
