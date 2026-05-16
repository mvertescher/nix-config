#!/usr/bin/env bash
set -euo pipefail

WORKDIR="/tmp/nix-config-vm"
MONITOR_PATH="${WORKDIR}/monitor.sock"

echo "Stopping NixOS QEMU VM..."

if [ -S "${MONITOR_PATH}" ] && echo "quit" | socat - unix-connect:"${MONITOR_PATH}" 2>/dev/null; then
    echo "VM stopped cleanly via monitor socket."
    while pgrep -f "qemu-system-x86_64.*${WORKDIR}" >/dev/null; do
        sleep 1
    done
else
    echo "Monitor socket not available or unresponsive."
    echo "Checking for running qemu processes associated with ${WORKDIR}..."
    if pgrep -f "qemu-system-x86_64.*${WORKDIR}" >/dev/null; then
        echo "Killing QEMU VM process..."
        pkill -f "qemu-system-x86_64.*${WORKDIR}"
        while pgrep -f "qemu-system-x86_64.*${WORKDIR}" >/dev/null; do
            sleep 1
        done
        echo "VM stopped."
    else
        echo "No running QEMU VM found."
    fi
    # Clean up dead sockets if they exist
    rm -f "${WORKDIR}/monitor.sock" "${WORKDIR}/qmp.sock"
fi
