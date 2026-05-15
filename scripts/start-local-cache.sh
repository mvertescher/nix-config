#!/usr/bin/env bash
set -euo pipefail

CACHE_DIR="${CACHE_DIR:-/tmp/nix-flake-cache}"
PORT="${PORT:-8080}"

echo "====================================================================="
echo "Starting Local Nix Binary Cache Server"
echo "Cache Directory: ${CACHE_DIR}"
echo "Port: ${PORT}"
echo "====================================================================="

mkdir -p "${CACHE_DIR}"

# Default flake outputs to cache (standalone homeConfigurations require homeDirectory to be set)
if [ "$#" -gt 0 ]; then
    TARGETS=("$@")
else
    TARGETS=(
        ".#nixosConfigurations.terra.config.system.build.toplevel"
    )
fi

echo "Archiving flake inputs to the local binary cache..."
nix flake archive --to "file://${CACHE_DIR}"

echo "---------------------------------------------------------------------"
echo "Building and copying derivations and build-time closures to the local cache..."
nix copy --derivation --to "file://${CACHE_DIR}" "${TARGETS[@]}"

echo "---------------------------------------------------------------------"
echo "Copying runtime closures to the local binary cache..."
nix copy --to "file://${CACHE_DIR}" "${TARGETS[@]}"

echo "---------------------------------------------------------------------"
echo "Cache populated successfully!"
echo ""
echo "To use this local binary cache on this host or inside a VM, pass the following flags"
echo "to your nix commands (e.g., nix build, nixos-install):"
echo ""
echo "  --option extra-substituters \"http://localhost:${PORT}\" --option require-sigs false"
echo ""
echo "If connecting from inside a QEMU VM (user-mode networking), use the host gateway IP:"
echo "  --option extra-substituters \"http://10.0.2.2:${PORT}\" --option require-sigs false"
echo "====================================================================="
echo "Starting HTTP server on port ${PORT}..."

exec nix run nixpkgs#python3 -- -m http.server "${PORT}" -d "${CACHE_DIR}"
