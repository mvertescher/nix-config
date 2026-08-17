#!/usr/bin/env bash
#
# Unattended (re)provisioning of a Vultr instance.
#
# Run from the root of a *wrapper* flake (this repo defines no hosts):
# the flake must expose nixosConfigurations.<host>, with the host's
# hardware config at ./hosts/<host>/hardware-configuration.nix.
#
# Usage (from the wrapper repo root):
#   VULTR_API_KEY=... ./public/scripts/provision-server.sh <ip> [host]
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
# Match only our custom installer by its exact filename — the account may
# also hold stock NixOS ISOs, which have no SSH keys baked in.
ISO_FILENAME="nix-config-installer.iso"
iso_id="$(vcurl "$API/iso?per_page=500" |
  jq -r --arg f "$ISO_FILENAME" '.isos[] | select(.filename == $f) | select(.status == "complete") | .id' |
  head -1)"

if [[ -z "$iso_id" ]]; then
  log "no $ISO_FILENAME in Vultr account; building"
  nix build .#installer-iso -o result-iso
  iso_file="$(find -L result-iso/iso -name '*.iso' | head -1)"
  [[ -n "$iso_file" ]] || { log "ISO build produced no .iso"; exit 1; }

  # Upload under the distinctive name (asset name = source file basename).
  staged_iso="$(mktemp -d)/$ISO_FILENAME"
  cp "$iso_file" "$staged_iso"

  log "publishing $ISO_FILENAME as GitHub release asset"
  if gh release view "$ISO_TAG" --repo "$REPO" >/dev/null 2>&1; then
    gh release upload "$ISO_TAG" "$staged_iso" --repo "$REPO" --clobber
  else
    gh release create "$ISO_TAG" "$staged_iso" --repo "$REPO" \
      --title "Installer ISO" --notes "NixOS installer with SSH keys baked in; built from system/installer.nix. Consumed by scripts/provision-server.sh."
  fi
  rm -rf "$(dirname "$staged_iso")"
  url="https://github.com/$REPO/releases/download/$ISO_TAG/$ISO_FILENAME"

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
# Detach any currently attached ISO first (attach fails otherwise).
attached="$(vcurl "$API/instances/$instance_id/iso" | jq -r '.iso_status.state // "ready"')"
if [[ "$attached" == "isomounted" ]]; then
  log "detaching currently attached ISO first"
  vcurl -X POST "$API/instances/$instance_id/iso/detach" >/dev/null
  sleep 20
fi

log "attaching ISO (instance will reboot into it)"
vcurl -X POST "$API/instances/$instance_id/iso/attach" \
  -H 'Content-Type: application/json' -d "{\"iso_id\": \"$iso_id\"}" >/dev/null
sleep 15 # let the reboot begin so we don't hit the old system's sshd

# --- 4. Install ---
wait_for_ssh root
log "running nixos-anywhere"
nix run github:nix-community/nixos-anywhere -- \
  --flake ".?submodules=1#$HOST" \
  --generate-hardware-config nixos-generate-config "./hosts/$HOST/hardware-configuration.nix" \
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
log "remember to commit the regenerated hosts/$HOST/hardware-configuration.nix"
