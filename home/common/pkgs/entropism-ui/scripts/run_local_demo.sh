#!/usr/bin/env bash
set -euo pipefail

# This script runs any Entropism UI example locally and interactively.
# By default, it uses a high-performance software rendering fallback (CPU-only via llvmpipe)
# which is 100% stable and crash-proof on all workstations.
#
# Usage:
#   ./run_local_demo.sh [example_name] [--hardware]

EXAMPLE="entropism-ui-demo"
FORCE_HW=0

# Parse arguments
for arg in "$@"; do
    if [ "$arg" = "--hardware" ] || [ "$arg" = "-h" ]; then
        FORCE_HW=1
    elif [[ "$arg" != -* ]]; then
        EXAMPLE="$arg"
    fi
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Always execute from the project directory
cd "$PROJECT_DIR"

if [ "$FORCE_HW" -eq 1 ]; then
    # Auto-detect graphics wrappers for non-NixOS hosts (experimental)
    GL_WRAPPER=""
    if command -v nixGLIntel &>/dev/null; then
        GL_WRAPPER="nixGLIntel"
    elif command -v nixGL &>/dev/null; then
        GL_WRAPPER="nixGL"
    elif command -v nixGLNvidia &>/dev/null; then
        GL_WRAPPER="nixGLNvidia"
    fi

    CMD="cargo run --bin $EXAMPLE"
    if [ -n "$GL_WRAPPER" ]; then
        echo "Experimental Hardware Acceleration enabled using $GL_WRAPPER."
        CMD="env WGPU_BACKEND=gl $GL_WRAPPER $CMD"
    else
        echo "No nixGL wrapper detected. Running native (recommended for NixOS)."
    fi
    
    echo "Launching '$EXAMPLE' in interactive local session..."
    nix develop -f shell.nix -c bash -c "$CMD"
else
    echo "Launching '$EXAMPLE' using stable Software Rendering (CPU/llvmpipe)..."
    # Force software rendering via shell.nix overrides
    export FORCE_SOFTWARE_GL=1
    nix develop -f shell.nix -c cargo run --bin "$EXAMPLE"
fi
