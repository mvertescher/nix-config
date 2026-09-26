# Host-set nix-darwin builder, exposed as `nix-config.lib.mkDarwin` —
# the macOS sibling of ./mkNixos.nix, same shape:
#
#   mkDarwin {
#     hosts = {
#       myhost = {
#         system = "aarch64-darwin";      # optional; Intel Macs pass x86_64-darwin
#         user = "mverte";                # optional, the macOS login user
#         modules = [ ./hosts/myhost ];   # host identity
#         homeModules = [ ./hosts/myhost/home.nix ];  # the user's HM imports
#       };
#     };
#     extraSystemConfig = { };            # optional module shared by all hosts
#     extraOverlays = [ ];                # optional, applied to all hosts
#   }
#
# The shared stack is darwin/configuration.nix, home-manager as a darwin
# module and stylix. Machine *kinds* (darwin/mac-mini.nix, ...) are not
# wired in here: a host opts into one by importing it from its modules.
#
# Unlike NixOS, macOS creates the login user itself (Setup Assistant), so
# `user` must name an account that already exists; the builder only
# tells nix-darwin and home-manager about it. `networking.hostName`,
# `computerName` and `localHostName` default to the attr name.
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
      # host-pkgs.nix defaults to x86_64-linux, which is right for its
      # other two callers and wrong for every Mac.
      pkgs = hostPkgs (host // { system = host.system or "aarch64-darwin"; });
      inherit (pkgs) lib;
      user = host.user or "mverte";
    in
    inputs.nix-darwin.lib.darwinSystem {
      inherit pkgs;
      specialArgs = { inherit inputs; };
      modules = [
        ../darwin/configuration.nix
        {
          networking.hostName = lib.mkDefault name;
          networking.computerName = lib.mkDefault name;
          networking.localHostName = lib.mkDefault name;
          # SMB keeps a fourth name of its own. Left unset it stays the
          # one Setup Assistant chose, and that is what Finder's sidebar
          # and the router's client list keep showing.
          system.defaults.smb.NetBIOSName = lib.mkDefault name;
          system.defaults.smb.ServerDescription = lib.mkDefault name;

          # nix-darwin applies user-scoped settings (system.defaults,
          # homebrew, ...) to this account.
          system.primaryUser = lib.mkDefault user;

          # Declares the existing account to nix-darwin without taking
          # ownership of it (users.knownUsers is left empty, so nothing
          # here creates or deletes macOS users). home-manager reads
          # `home` from here to derive home.homeDirectory.
          users.users.${user}.home = lib.mkDefault "/Users/${user}";
        }
        extraSystemConfig
      ] ++ (host.modules or [ ]) ++ [
        inputs.home-manager.darwinModules.home-manager
        {
          home-manager.useGlobalPkgs = true;
          home-manager.useUserPackages = true;
          home-manager.backupFileExtension = "backup";
          home-manager.users.${user}.imports = host.homeModules or [ ];
        }
        inputs.stylix.darwinModules.stylix
      ];
    };
in
inputs.nixpkgs.lib.mapAttrs make hosts
