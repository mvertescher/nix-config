#!/usr/bin/env bash
set -euo pipefail

USE_CDROM=false
HEADLESS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --cdrom|-c)
            USE_CDROM=true
            shift
            ;;
        --headless|-h)
            HEADLESS=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--cdrom] [--headless]"
            exit 1
            ;;
    esac
done

WORKDIR="/tmp/nix-config-vm"
ISO_URL="https://channels.nixos.org/nixos-unstable/latest-nixos-minimal-x86_64-linux.iso"
ISO_PATH="${WORKDIR}/nixos.iso"
DISK_PATH="${WORKDIR}/disk.qcow2"
QMP_PATH="${WORKDIR}/qmp.sock"
MONITOR_PATH="${WORKDIR}/monitor.sock"

echo "Using working directory: ${WORKDIR}"
mkdir -p "${WORKDIR}"
cd "${WORKDIR}"

if [ ! -f "${ISO_PATH}" ] || [ $(stat -c %s "${ISO_PATH}") -lt 1000000000 ]; then
    echo "Downloading NixOS minimal ISO..."
    env -u http_proxy -u https_proxy -u HTTP_PROXY -u HTTPS_PROXY curl -k -L -o "${ISO_PATH}" "${ISO_URL}"
else
    echo "NixOS ISO already exists at ${ISO_PATH}"
fi

if [ ! -f "${DISK_PATH}" ]; then
    echo "Creating 50G QEMU disk image..."
    qemu-img create -f qcow2 "${DISK_PATH}" 50G
else
    echo "Disk image already exists at ${DISK_PATH}"
fi

echo "Starting NixOS QEMU VM..."
echo "SSH port 22 in the VM is forwarded to localhost:2222"
echo "QMP socket: ${QMP_PATH}"
echo "Monitor socket: ${MONITOR_PATH}"

QEMU_ARGS=(
    -enable-kvm
    -cpu host
    -m 8192
    -smp 4
    -drive file="${DISK_PATH}",format=qcow2,if=virtio,cache=writeback
    -netdev user,id=net0,hostfwd=tcp::2222-:22
    -device virtio-net-pci,netdev=net0
    -usb -device usb-tablet
    -vga virtio
    -qmp unix:"${QMP_PATH}",server,nowait
    -monitor unix:"${MONITOR_PATH}",server,nowait
)

if [[ "${HEADLESS}" == "true" ]]; then
    echo "Running in headless mode (-display none)..."
    QEMU_ARGS+=( -display none )
fi

if [[ "${USE_CDROM}" == "true" ]]; then
    echo "Attaching ISO and booting from CD-ROM..."
    QEMU_ARGS+=( -cdrom "${ISO_PATH}" -boot d )
else
    echo "Booting directly from hard disk..."
    QEMU_ARGS+=( -bios /nix/store/6jq9jmkhsg1swfyrnf2shnqzrkmfzlgv-OVMF-202602-fd/FV/OVMF.fd -boot c )
fi

exec qemu-system-x86_64 "${QEMU_ARGS[@]}"
