# Custom NixOS installer ISO with SSH access baked in, so nixos-anywhere
# can provision a machine booted from it with zero console interaction.
#
# Build:  nix build .#installer-iso
# Used by a wrapper flake's provisioning tool (nix-config-private's
# ./provision), which builds it, has Vultr fetch it over plain HTTP once,
# and boots new or reinstalled instances from it via the API.

{ modulesPath, ... }:

let
  sshKeys = import ../lib/ssh-keys.nix;
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
