# Declarative disk layout for nixos-anywhere / disko.
#
# Mirrors the scheme scripts/vcs-install-nix.sh used to do imperatively:
# GPT with a bios_grub partition (legacy BIOS boot), 4G swap, ext4 root.
#
# Provision with:
#   nix run github:nix-community/nixos-anywhere -- \
#     --flake .#server \
#     --generate-hardware-config nixos-generate-config ./system/host/server/hardware-configuration.nix \
#     root@<server-ip>

{ lib, ... }:

{
  disko.devices = {
    disk.main = {
      type = "disk";
      # mkDefault so a different target disk can be overridden without
      # editing this file.
      device = lib.mkDefault "/dev/vda";
      content = {
        type = "gpt";
        partitions = {
          boot = {
            size = "1M";
            type = "EF02"; # bios_grub; disko sets boot.loader.grub.devices from this
          };
          swap = {
            size = "4G";
            content = {
              type = "swap";
            };
          };
          root = {
            size = "100%";
            content = {
              type = "filesystem";
              format = "ext4";
              mountpoint = "/";
            };
          };
        };
      };
    };
  };
}
