#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)

case "${1:-start}" in
    start)
        echo "Starting VM on Hyprland Workspace 3..."
        if pgrep -f "qemu-system-x86_64" >/dev/null; then
            echo "VM is already running."
        else
            # Dynamically discover active Hyprland instance signature to support tmux/SSH sessions
            ACTIVE_HYPR=$(ls -dt /run/user/$(id -u)/hypr/* 2>/dev/null | head -n 1 || true)
            if [ -n "${ACTIVE_HYPR:-}" ] && [ -S "${ACTIVE_HYPR}/.socket.sock" ]; then
                export HYPRLAND_INSTANCE_SIGNATURE=$(basename "${ACTIVE_HYPR}")
            fi

            if [ -n "${HYPRLAND_INSTANCE_SIGNATURE:-}" ] && which hyprctl >/dev/null 2>&1; then
                if ! hyprctl dispatch exec "[workspace 3 silent] bash ${SCRIPT_DIR}/start-vm.sh" 2>/dev/null; then
                    echo "Warning: Failed to dispatch to Hyprland. Falling back to standard launch..."
                    bash "${SCRIPT_DIR}/start-vm.sh" & disown
                fi
            else
                bash "${SCRIPT_DIR}/start-vm.sh" & disown
            fi
        fi
        ;;
    stop)
        echo "Stopping VM..."
        bash "${SCRIPT_DIR}/stop-vm.sh"
        ;;
    restart)
        "$0" stop
        "$0" start
        ;;
    *)
        echo "Usage: $0 [start|stop|restart]"
        exit 1
        ;;
esac
