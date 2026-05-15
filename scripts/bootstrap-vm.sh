#!/usr/bin/env bash

set -eux pipefail

SCRIPT_DIR=$(dirname "$0")
WORKDIR="/tmp/nix-config-vm"

echo "Stopping existing VM if running..."
"${SCRIPT_DIR}/stop-vm.sh" || true

echo "Cleaning up working directory..."
rm -rf "${WORKDIR}"
mkdir -p "${WORKDIR}"

echo "Starting VM with CD-ROM attached..."

nohup bash "${SCRIPT_DIR}/start-vm.sh" --cdrom > "${WORKDIR}/vm.log" 2>&1 &

echo "Waiting for QEMU monitor socket to appear (ISO may be downloading)..."
while [ ! -S "${WORKDIR}/monitor.sock" ]; do
    sleep 5
done

echo "Monitor socket detected. Pressing Enter at boot menu..."
sleep 5
echo "sendkey ret" | socat - unix-connect:"${WORKDIR}/monitor.sock"

echo "Waiting 60 seconds for NixOS live CD to boot to console login..."
sleep 60

VM_HOST="localhost"
VM_PORT=2222
VM_USER="root"
VM_PASS="toor" # Password for root user in the VM console
FLAKE_DIR=$(pwd)

# Common SSH options to bypass host key checking and prompt
SSH_OPTS="-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o ServerAliveInterval=60 -p ${VM_PORT}"

# Wrapper command using cached sshpass binary directly
SSH_CMD="/nix/store/ml4rw2qalsx6cag1z2qp420gbws796w6-sshpass-1.10/bin/sshpass -p ${VM_PASS} ssh ${SSH_OPTS}"

MONITOR_PATH="/tmp/nix-config-vm/monitor.sock"
echo "Sending commands to QEMU monitor to enable SSH and reset root password..."
if [ -S "${MONITOR_PATH}" ]; then
    (
        echo "sendkey ret"
        sleep 1

        for char in s u d o spc s y s t e m c t l spc s t a r t spc s s h d; do
            echo "sendkey $char"
            sleep 0.2
        done
        echo "sendkey ret"
        sleep 1

        for char in s u d o spc p a s s w d spc r o o t; do
            echo "sendkey $char"
            sleep 0.2
        done
        echo "sendkey ret"
        sleep 2

        for (( i=0; i<${#VM_PASS}; i++ )); do
            echo "sendkey ${VM_PASS:$i:1}"
            sleep 0.2
        done
        echo "sendkey ret"
        sleep 2

        for (( i=0; i<${#VM_PASS}; i++ )); do
            echo "sendkey ${VM_PASS:$i:1}"
            sleep 0.2
        done
        echo "sendkey ret"
        sleep 2
    ) | socat - unix-connect:"${MONITOR_PATH}"
else
    echo "WARNING: QEMU monitor socket not found at ${MONITOR_PATH}"
fi

echo "Checking if VM SSH is reachable and authenticated on ${VM_HOST}:${VM_PORT}..."
if ! ${SSH_CMD} "${VM_USER}@${VM_HOST}" true 2>/dev/null; then
    echo "---------------------------------------------------------------------"
    echo "ERROR: Cannot connect or authenticate to VM on port ${VM_PORT}."
    echo "Please ensure the VM is running, SSH is enabled, and password is set."
    echo ""
    echo "In the NixOS live VM console, run the following commands:"
    echo "  sudo systemctl start sshd"
    echo "  sudo passwd nixos"
    echo "  (set password to '${VM_PASS}')"
    echo "---------------------------------------------------------------------"
    exit 1
fi

echo "Connecting to VM to partition disk and bootstrap NixOS..."
echo "Automatically authenticating as ${VM_USER} with password '${VM_PASS}'..."

# Partition disk and format with UUIDs matching terra/hardware-configuration.nix
${SSH_CMD} "${VM_USER}@${VM_HOST}" "sudo bash -s" << 'EOF'
set -euo pipefail

# Ensure any previous mounts are unmounted before re-partitioning
umount -R /mnt 2>/dev/null || true

DISK_DEV="/dev/vda"
if [ ! -b "${DISK_DEV}" ]; then
    DISK_DEV="/dev/sda"
fi
PART_ESP="${DISK_DEV}1"
PART_ROOT="${DISK_DEV}2"

echo "Partitioning ${DISK_DEV}..."
parted -s "${DISK_DEV}" -- mklabel gpt
parted -s "${DISK_DEV}" -- mkpart ESP fat32 1MiB 512MiB
parted -s "${DISK_DEV}" -- set 1 esp on
parted -s "${DISK_DEV}" -- mkpart primary ext4 512MiB 100%

partprobe "${DISK_DEV}"
udevadm settle || sleep 2

echo "Formatting partitions with UUIDs matching terra configuration..."
# FAT32 UUID: F8C6-5B0B (formatted as F8C65B0B for mkfs.vfat)
mkfs.vfat -F 32 -i F8C65B0B "${PART_ESP}"
# EXT4 UUID: 0f3f4493-27e9-4ca0-843f-4b9b00ab3933
mkfs.ext4 -F -U 0f3f4493-27e9-4ca0-843f-4b9b00ab3933 "${PART_ROOT}"

udevadm settle || sleep 2

echo "Mounting partitions..."
mount -t ext4 "${PART_ROOT}" /mnt
mkdir -p /mnt/boot
mount -t vfat "${PART_ESP}" /mnt/boot

mkdir -p /mnt/etc/nixos
chown nixos:users /mnt/etc/nixos
EOF

echo "Copying local flake directory to VM (/mnt/etc/nixos)..."
# Use tar over ssh with automatic password authentication
tar -czf - --exclude='.git' -C "${FLAKE_DIR}" . | ${SSH_CMD} "${VM_USER}@${VM_HOST}" "tar -xzf - -C /mnt/etc/nixos"

echo "Starting NixOS installation on VM..."
${SSH_CMD} "${VM_USER}@${VM_HOST}" "sudo bash -s" << 'EOF'
set -euo pipefail
echo "Running nixos-install for flake .#terra..."
nixos-install --no-root-passwd --option extra-substituters "http://10.0.2.2:8080" --option require-sigs false --flake /mnt/etc/nixos#terra 2>&1 | tee /dev/tty1
echo "---------------------------------------------------------------------"
echo "Installation complete! You can now power off or reboot the VM."
echo "---------------------------------------------------------------------"
EOF
