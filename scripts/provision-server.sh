#!/usr/bin/env bash
#
# Unattended (re)provisioning of a Vultr instance with this flake's config.
#
# Usage:
#   VULTR_API_KEY=... ./scripts/provision-server.sh <ip> [host]
#
#   <ip>    main IP of an existing Vultr instance
#   [host]  flake nixosConfiguration to install (default: server)
#
# Flow:
#   1. Find the instance ID for <ip> via the Vultr API.
#   2. Ensure our custom installer ISO (SSH keys baked in — see
#      system/installer.nix) exists in the Vultr account. If not:
#      build it, publish it as a GitHub release asset, and have Vultr
#      pull it from there.
#   3. Attach the ISO (Vultr reboots the instance into it).
#   4. Wait for root SSH on the installer, then run nixos-anywhere
#      (kexec/disko/install phases only — no reboot, since the ISO must
#      be detached first or the box would boot the installer again).
#   5. Detach the ISO (Vultr reboots into the installed system).
#   6. Wait for SSH as mverte on the new system.
#
# Requirements: curl, jq, gh (authenticated), nix. An SSH key for this
# machine must be in lib/ssh-keys.nix or on github.com/mvertescher.keys.
#
# To force a rebuild of the ISO (e.g. after changing installer.nix),
# delete the old one first: it shows up in `vultr-cli iso list` /
# GET /v2/iso, then DELETE /v2/iso/{id}, and delete the GitHub release
# tag "installer-iso".

set -euo pipefail

IP="${1:?usage: provision-server.sh <ip> [host]}"
HOST="${2:-server}"
: "${VULTR_API_KEY:?set VULTR_API_KEY}"

API="https://api.vultr.com/v2"
ISO_TAG="installer-iso" # GitHub release tag
REPO="mvertescher/nix-config"
SSH_OPTS=(-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o ConnectTimeout=5 -o BatchMode=yes)

cd "$(dirname "$0")/.."

log() { echo ">>> $*" >&2; }

vcurl() {
  curl -sf -H "Authorization: Bearer $VULTR_API_KEY" "$@"
}

wait_for_ssh() {
  local user="$1" deadline=$((SECONDS + 600))
  log "waiting for SSH as $user@$IP (up to 10m)"
  until ssh "${SSH_OPTS[@]}" "$user@$IP" true 2>/dev/null; do
    if ((SECONDS > deadline)); then
      log "timed out waiting for $user@$IP"
      return 1
    fi
    sleep 5
  done
}

# --- 1. Instance lookup ---
instance_id="$(vcurl "$API/instances?per_page=500" |
  jq -r --arg ip "$IP" '.instances[] | select(.main_ip == $ip) | .id')"
[[ -n "$instance_id" ]] || { log "no Vultr instance with main IP $IP"; exit 1; }
log "instance: $instance_id"

# --- 2. Ensure installer ISO exists in the Vultr account ---
iso_id="$(vcurl "$API/iso?per_page=500" |
  jq -r '.isos[] | select(.filename | test("nixos-.*\\.iso")) | select(.status == "complete") | .id' |
  head -1)"

if [[ -z "$iso_id" ]]; then
  log "no installer ISO in Vultr account; building"
  nix build .#installer-iso -o result-iso
  iso_file="$(find -L result-iso/iso -name '*.iso' | head -1)"
  [[ -n "$iso_file" ]] || { log "ISO build produced no .iso"; exit 1; }

  log "publishing $(basename "$iso_file") as GitHub release asset"
  if gh release view "$ISO_TAG" --repo "$REPO" >/dev/null 2>&1; then
    gh release upload "$ISO_TAG" "$iso_file" --repo "$REPO" --clobber
  else
    gh release create "$ISO_TAG" "$iso_file" --repo "$REPO" \
      --title "Installer ISO" --notes "NixOS installer with SSH keys baked in; built from system/installer.nix. Consumed by scripts/provision-server.sh."
  fi
  url="https://github.com/$REPO/releases/download/$ISO_TAG/$(basename "$iso_file")"

  log "asking Vultr to fetch $url"
  iso_id="$(vcurl -X POST "$API/iso" -H 'Content-Type: application/json' \
    -d "{\"url\": \"$url\"}" | jq -r '.iso.id')"

  log "waiting for Vultr to finish downloading the ISO"
  until [[ "$(vcurl "$API/iso/$iso_id" | jq -r '.iso.status')" == "complete" ]]; do
    sleep 10
  done
fi
log "iso: $iso_id"

# --- 3. Attach ISO (reboots the instance into the installer) ---
log "attaching ISO (instance will reboot into it)"
vcurl -X POST "$API/instances/$instance_id/iso/attach" \
  -H 'Content-Type: application/json' -d "{\"iso_id\": \"$iso_id\"}" >/dev/null
sleep 15 # let the reboot begin so we don't hit the old system's sshd

# --- 4. Install ---
wait_for_ssh root
log "running nixos-anywhere"
nix run github:nix-community/nixos-anywhere -- \
  --flake ".#$HOST" \
  --generate-hardware-config nixos-generate-config "./system/host/$HOST/hardware-configuration.nix" \
  --phases kexec,disko,install \
  --ssh-option StrictHostKeyChecking=no \
  --ssh-option UserKnownHostsFile=/dev/null \
  "root@$IP"

# --- 5. Detach ISO (reboots into the installed system) ---
log "detaching ISO (instance will reboot into the installed system)"
vcurl -X POST "$API/instances/$instance_id/iso/detach" >/dev/null
sleep 15

# --- 6. Verify ---
wait_for_ssh mverte
log "done: $(ssh "${SSH_OPTS[@]}" "mverte@$IP" hostname) is up at $IP"
log "remember to commit the regenerated system/host/$HOST/hardware-configuration.nix"
