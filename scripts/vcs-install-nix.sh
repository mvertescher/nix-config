#!/usr/bin/env bash
set -euo pipefail

# --- Clean up any partial state from a previous attempt ---
umount -R /mnt 2>/dev/null || true
swapoff /dev/vda2 2>/dev/null || true
wipefs -a /dev/vda

# --- Partition the disk (single disk, LEGACY BIOS/GRUB) ---
parted -s /dev/vda -- mklabel gpt
parted -s /dev/vda -- mkpart bios_boot 1MB 2MB
parted -s /dev/vda -- set 1 bios_grub on
parted -s /dev/vda -- mkpart swap linux-swap 2MB 4.1GB
parted -s /dev/vda -- mkpart root ext4 4.1GB 100%

mkswap -L swap /dev/vda2
mkfs.ext4 -F -L nixos /dev/vda3

udevadm settle

mount /dev/vda3 /mnt
swapon /dev/vda2

nixos-generate-config --root /mnt

cat > /mnt/etc/nixos/configuration.nix <<'EOF'
{ config, pkgs, ... }:

{
  imports = [ ./hardware-configuration.nix ];

  boot.loader.grub.enable = true;
  boot.loader.grub.device = "/dev/vda";

  networking.hostName = "devbox";
  networking.useDHCP = true;

  time.timeZone = "UTC";

  users.users.mverte = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    openssh.authorizedKeys.keys = [
      "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILBrJP87O00JevRDmMIOvR23XvB820Ta62on5GGyTvMZ"
    ];
  };

  security.sudo.wheelNeedsPassword = false;

  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    settings.PermitRootLogin = "no";
  };

  environment.systemPackages = with pkgs; [
    git
    vim
    curl
    rustup
  ];

  networking.firewall.allowedTCPPorts = [ 22 ];

  system.stateVersion = "26.05";
}
EOF

echo "Config written. Installing..."
nixos-install --no-root-passwd

echo ""
echo "Install complete. Run 'reboot' now, then SSH in as: mverte@<your-vultr-ip>"
