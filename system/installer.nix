# Custom NixOS installer ISO with SSH access baked in, so nixos-anywhere
# can provision a machine booted from it with zero console interaction.
#
# Build:  nix build .#installer-iso
# Used by a wrapper flake's provisioning tool (nix-config-private's
# ./provision), which builds it, has Vultr fetch it over plain HTTP once,
# and boots new or reinstalled instances from it via the API.

{ modulesPath, ... }:

let
  # Installer-only keys, on top of the shared list. Kept here rather than
  # in ../lib/ssh-keys.nix because that file is authorized on every
  # running host too, and these machines must only be able to *install*
  # a box, not log into the fleet (decided 2026-10-17).
  #
  # arca: the small always-on cloud box, so an on-demand dev box can be
  # provisioned while terra is down. terra's own key is already in
  # ssh-keys.nix; the ISO uploaded 2026-08-17 predates it, which is why
  # `./provision up arca` from terra failed with "no SSH as root".
  # Rebuild and re-upload the ISO after changing either list.
  installerOnlyKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPevIyUUgWN5WCCnQFrueP2OTX/gcAD8PO0OWappqQVK mverte@arca"
  ];
  sshKeys = (import ../lib/ssh-keys.nix) ++ installerOnlyKeys;
in
{
  imports = [
    (modulesPath + "/installer/cd-dvd/installation-cd-minimal.nix")
    ./github-authorized-keys.nix
  ];

  users.users.root.openssh.authorizedKeys.keys = sshKeys;
  users.users.nixos.openssh.authorizedKeys.keys = sshKeys;

  # Also accept any key on the GitHub account, so a machine added later
  # via `gh ssh-key add` can use this ISO without rebuilding/reuploading.
  # localUsers is deliberately left empty: nixos-anywhere logs in as
  # root, the console user is nixos, and this image has no other
  # accounts worth protecting from a key we already trust.
  custom.githubAuthorizedKeys = {
    enable = true;
    githubUsers = [ "mvertescher" ];
  };

  # Faster build at slightly larger image size.
  isoImage.squashfsCompression = "zstd -Xcompression-level 3";
}
