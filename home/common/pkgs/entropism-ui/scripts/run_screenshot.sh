#!/usr/bin/env bash
set -euo pipefail

# Auto-detect Hyprland signature if the current one is invalid
if ! hyprctl activeworkspace &>/dev/null; then
    for dir in /run/user/$(id -u)/hypr/*; do
        if [ -S "$dir/.socket.sock" ]; then
            export HYPRLAND_INSTANCE_SIGNATURE="${dir##*/}"
            break
        fi
    done
fi

APP="${1:-entropism-ui-demo}"
OUTPUT="${2:-screenshot.png}"

# Ensure the app binary is available
if ! command -v "$APP" &> /dev/null; then
    echo "Error: Command '$APP' not found." >&2
    exit 1
fi

echo "Capturing current workspace..."
CURRENT_WORKSPACE=$(hyprctl activeworkspace -j | jq -r '.id')

echo "Switching to workspace 9..."
hyprctl dispatch workspace 9

echo "Starting $APP on workspace 9..."
# Launch app
"$APP" &
APP_PID=$!

echo "Waiting for app to render..."
sleep 2.5

echo "Saving screenshot to $OUTPUT..."
grimblast save screen "$OUTPUT"

echo "Terminating $APP (PID: $APP_PID)..."
kill -9 "$APP_PID" || true

echo "Restoring workspace $CURRENT_WORKSPACE..."
hyprctl dispatch workspace "$CURRENT_WORKSPACE"

echo "Done!"
