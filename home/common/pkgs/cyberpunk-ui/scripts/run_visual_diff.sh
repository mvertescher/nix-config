#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_DIR"
mkdir -p target

# Reference paths (should be the perfect crops)
REF_SELECTED="/tmp/floppy_selected_design.png"
REF_UNSELECTED="/tmp/floppy_unselected_design.png"

# Output paths
RENDER_SELECTED="target/render_selected.png"
RENDER_UNSELECTED="target/render_unselected.png"
DIFF_SELECTED="target/diff_selected.png"
DIFF_UNSELECTED="target/diff_unselected.png"

# Ensure references exist
if [ ! -f "$REF_SELECTED" ] || [ ! -f "$REF_UNSELECTED" ]; then
    echo "Error: Reference crops not found in /tmp. Please run crop_perfect.py first." >&2
    exit 1
fi

echo "Pre-compiling neomil-ui-floppy..."
nix develop -f "shell.nix" -c cargo build --bin neomil-ui-floppy

capture_mode() {
    local mode="$1"
    local output="$2"
    local socket="wayland-diff-$$"
    
    echo "Capturing $mode floppy at 240x220..."
    
    # Run weston with the app in DIFF_MODE
    export DIFF_MODE="$mode"
    export FORCE_SOFTWARE_GL=1
    export RUST_BACKTRACE=1
    unset WLR_RENDERER
    
    env WLR_BACKENDS=headless WAYLAND_DISPLAY="$socket" \
        nix develop -f "shell.nix" -c \
        weston --backend=headless --renderer=gl --shell=kiosk --socket="$socket" --width=240 --height=220 --no-config --debug -- \
        ./target/debug/neomil-ui-floppy &> "target/headless_diff_${mode}.log" &
    local weston_pid=$!
    
    # Wait for socket
    local socket_path="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}/$socket"
    local timeout=15
    while [ ! -S "$socket_path" ]; do
        sleep 0.5
        timeout=$((timeout - 1))
        if [ "$timeout" -le 0 ]; then
            echo "Error: Headless compositor failed to start for $mode." >&2
            kill "$weston_pid" 2>/dev/null || true
            exit 1
        fi
    done
    
    # Wait for render
    sleep 3
    
    # Capture screenshot
    env WAYLAND_DISPLAY="$socket" \
        nix develop -f "shell.nix" -c weston-screenshooter
    
    sleep 1
    
    # Clean up weston
    kill "$weston_pid" 2>/dev/null || true
    rm -f "$socket_path" || true
    
    # Move screenshot
    local success=0
    shopt -s nullglob
    local files=( wayland-screenshot*.png )
    if [ ${#files[@]} -gt 0 ]; then
        local newest=$(ls -t wayland-screenshot*.png | head -n 1)
        mv "$newest" "$output"
        success=1
    else
        files=( "$HOME"/wayland-screenshot*.png )
        if [ ${#files[@]} -gt 0 ]; then
            local newest=$(ls -t "$HOME"/wayland-screenshot*.png | head -n 1)
            mv "$newest" "$output"
            success=1
        fi
    fi
    
    if [ $success -ne 1 ]; then
        echo "Error: Failed to capture screenshot for $mode." >&2
        exit 1
    fi
}

# Capture both
capture_mode "selected" "$RENDER_SELECTED"
capture_mode "unselected" "$RENDER_UNSELECTED"

# Run visual diff tool (with 3x enhancement to make differences pop)
echo ""
echo "=== VISUAL DIFF RESULTS ==="
echo "Comparing Selected Floppy..."
nix-shell -p python3Packages.pillow --run "python3 scripts/visual_diff.py \"$REF_SELECTED\" \"$RENDER_SELECTED\" \"$DIFF_SELECTED\" 3.0"

echo ""
echo "Comparing Unselected Floppy..."
nix-shell -p python3Packages.pillow --run "python3 scripts/visual_diff.py \"$REF_UNSELECTED\" \"$RENDER_UNSELECTED\" \"$DIFF_UNSELECTED\" 3.0"

echo ""
echo "Diff images saved to:"
echo "  - Selected Diff: $DIFF_SELECTED"
echo "  - Unselected Diff: $DIFF_UNSELECTED"
echo "  - Selected Render: $RENDER_SELECTED"
echo "  - Unselected Render: $RENDER_UNSELECTED"
