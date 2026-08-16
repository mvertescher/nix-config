# Placeholder hardware config for the server host.
#
# Filesystems and swap are declared in ./disko.nix — do not add them here.
# This file is regenerated on the target by nixos-anywhere when passed
# --generate-hardware-config nixos-generate-config <this path>; commit the
# regenerated version after provisioning.
{ config, lib, pkgs, modulesPath, ... }:

{
  imports = [ (modulesPath + "/profiles/qemu-guest.nix") ];

  boot.initrd.availableKernelModules = [ "ata_piix" "uhci_hcd" "virtio_pci" "sr_mod" "virtio_blk" ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ ];
  boot.extraModulePackages = [ ];

  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
}
