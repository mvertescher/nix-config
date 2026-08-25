#!/usr/bin/env bash
set -euo pipefail

EXAMPLE="${1:-neomil-ui-dashboard}"
OUTPUT="${2:-screenshot_headless.png}"
SOCKET="wayland-headless-$$"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Ensure target directory exists for logs and screenshots
mkdir -p "$PROJECT_DIR/target"
# Ensure output path is absolute so mv works reliably
OUTPUT_ABS="$(cd "$(dirname "$OUTPUT")" && pwd)/$(basename "$OUTPUT")"

# Force software rendering for both compositor and app via shell.nix
export FORCE_SOFTWARE_GL=1
export RUST_BACKTRACE=1
# Clear any parent environment pollution
unset WLR_RENDERER

# CRITICAL: cd to the project directory first.
cd "$PROJECT_DIR"

# DYNAMIC RESOLUTION DETECTION
# Default fallback resolution
WIDTH=1920
HEIGHT=1080

echo "Detecting active monitor resolution..."
if command -v hyprctl &>/dev/null; then
    # Parse the focused monitor's resolution from hyprctl monitors.
    # We use a stateful awk script to locate the resolution of the focused monitor.
    RES=$(hyprctl monitors | awk '
        /^Monitor/ { active=1; next }
        active && /^[ \t]*[0-9]+x[0-9]+/ { res=$1; active=0 }
        /focused: yes/ { focused_res=res }
        END { print focused_res }
    ' | cut -d'@' -f1 | tr 'x' ' ' || echo "")
    
    if [ -n "$RES" ]; then
        read -r DETECTED_WIDTH DETECTED_HEIGHT <<< "$RES"
        # Validate that we got numbers
        if [[ "$DETECTED_WIDTH" =~ ^[0-9]+$ ]] && [[ "$DETECTED_HEIGHT" =~ ^[0-9]+$ ]]; then
            WIDTH=$DETECTED_WIDTH
            HEIGHT=$DETECTED_HEIGHT
            echo "Detected active monitor resolution: ${WIDTH}x${HEIGHT}"
        else
            echo "Warning: Parsed resolution is invalid ('$RES'). Falling back to ${WIDTH}x${HEIGHT}."
        fi
    else
        echo "Warning: Could not parse active monitor resolution. Falling back to ${WIDTH}x${HEIGHT}."
    fi
else
    echo "Hyprctl not found. Falling back to ${WIDTH}x${HEIGHT}."
fi

echo "Pre-compiling the application '$EXAMPLE' to ensure instant launch..."
nix develop -f "shell.nix" -c cargo build --bin "$EXAMPLE"

echo "Starting headless Weston compositor (${WIDTH}x${HEIGHT}, kiosk shell) with '$EXAMPLE' on socket '$SOCKET'..."

# Run weston inside the nix develop shell.
# We use the headless backend and force the GL renderer (which will use llvmpipe).
# We use --shell=kiosk to remove the top bar and automatically fullscreen the app.
# We pass --no-config and --debug (required for screenshots).
env WLR_BACKENDS=headless WAYLAND_DISPLAY="$SOCKET" \
    nix develop -f "shell.nix" -c \
    weston --backend=headless --renderer=gl --shell=kiosk --socket="$SOCKET" --width="$WIDTH" --height="$HEIGHT" --no-config --debug -- \
    ./target/debug/$EXAMPLE &> "target/headless_app.log" &
WESTON_PID=$!

# Ensure we clean up on exit
cleanup() {
    echo "Cleaning up background processes..."
    kill "$WESTON_PID" 2>/dev/null || true
    rm -f "${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/$SOCKET" || true
}
trap cleanup EXIT

echo "Waiting for virtual display to initialize..."
SOCKET_PATH="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/$SOCKET"
TIMEOUT=15
while [ ! -S "$SOCKET_PATH" ]; do
    sleep 0.5
    TIMEOUT=$((TIMEOUT - 1))
    if [ "$TIMEOUT" -le 0 ]; then
        echo "Error: Headless compositor failed to start socket in time." >&2
        if [ -f "target/headless_app.log" ]; then
            echo "--- Headless App Log ---" >&2
            cat "target/headless_app.log" >&2
        fi
        exit 1
    fi
done

echo "Headless display active. Waiting 6 seconds for app to render..."
sleep 6

echo "Capturing screenshot of the virtual display using weston-screenshooter..."
env WAYLAND_DISPLAY="$SOCKET" \
    nix develop -f "shell.nix" -c weston-screenshooter

echo "Waiting for screenshot file to be written..."
sleep 2

# Locate the screenshot file and move it to the desired output path.
SUCCESS=0
shopt -s nullglob
files=( wayland-screenshot*.png )
if [ ${#files[@]} -gt 0 ]; then
    newest=$(ls -t wayland-screenshot*.png | head -n 1)
    mv "$newest" "$OUTPUT_ABS"
    SUCCESS=1
else
    files=( "$HOME"/wayland-screenshot*.png )
    if [ ${#files[@]} -gt 0 ]; then
        newest=$(ls -t "$HOME"/wayland-screenshot*.png | head -n 1)
        mv "$newest" "$OUTPUT_ABS"
        SUCCESS=1
    fi
fi

if [ $SUCCESS -eq 1 ]; then
    echo "Screenshot successfully saved to '$OUTPUT_ABS'."
else
    echo "Error: Screenshot file 'wayland-screenshot*.png' not found in CWD or Home." >&2
    if [ -f "target/headless_app.log" ]; then
        echo "--- Headless App Log ---" >&2
        cat "target/headless_app.log" >&2
    fi
    exit 1
fi
