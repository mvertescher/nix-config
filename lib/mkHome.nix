# Standalone home-manager configurations, exposed as
# `nix-config.lib.mkHome` — same host-set shape as ./mkNixos.nix:
#
#   mkHome {
#     hosts = {
#       myhost = {
#         system = "x86_64-linux";       # optional, per host
#         user = "mvertescher";          # optional, per host
#         modules = [ ./hosts/myhost/home.nix ];
#       };
#     };
#     extraOverlays = [ ];               # optional, applied to all hosts
#   }
#
# This repo defines no hosts: the wrapper owns machine identity
# (username, home directory, monitors, ...) in its own modules, which
# import this repo's shared home/ modules by path. `home.username` and
# `home.homeDirectory` are defaulted from `user` at mkDefault priority
# — home directories vary per host, so a host module's plain definition
# overrides without mkForce.
{ inputs, overlays }:

{ hosts, extraOverlays ? [ ] }:

let
  hostPkgs = import ./host-pkgs.nix { inherit inputs overlays extraOverlays; };

  make = name: host:
    let
      pkgs = hostPkgs host;
      lib = pkgs.lib;
      # `mkNixos` defaults this to "mverte" instead. Not an oversight and
      # not to be unified: both are frozen by consumers whose call sites
      # cannot be updated from here.
      user = host.user or "mvertescher";
    in
    inputs.home-manager.lib.homeManagerConfiguration {
      inherit pkgs;
      modules = (host.modules or [ ]) ++ [
        inputs.stylix.homeModules.stylix
        {
          home.username = lib.mkDefault user;
          home.homeDirectory = lib.mkDefault "/home/${user}";
        }
      ];
    };
in
inputs.nixpkgs.lib.mapAttrs make hosts
