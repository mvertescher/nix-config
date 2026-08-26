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
#      system/installer.nix) exists in the Vultr account. If not, build
#      it and publish it somewhere Vultr will actually fetch it from
#      (see "Publishing the ISO" below).
#   3. Attach the ISO (Vultr reboots the instance into it).
#   4. Wait for root SSH on the installer, then run nixos-anywhere
#      (kexec/disko/install phases only — no reboot, since the ISO must
#      be detached first or the box would boot the installer again).
#   5. Detach the ISO (Vultr reboots into the installed system).
#   6. Wait for SSH as mverte on the new system.
#
# Publishing the ISO
# ------------------
# POST /v2/iso is picky about URLs, and it fails *silently*: the request
# is accepted, the new record sits in "pending" for ~30 seconds, and
# then Vultr deletes the record without ever having contacted the host.
# GitHub release asset URLs fail exactly this way (they redirect to a
# CDN), and so, presumably, does anything else behind a redirect. What
# is known to work is plain HTTP served from a Vultr instance.
#
# So this script builds the ISO, copies it to a Vultr box over SSH,
# serves it there with a throwaway HTTP server, and hands Vultr that
# plain-HTTP URL. The default serving box is the instance being
# provisioned: it is about to be wiped anyway, it is guaranteed to
# exist, and it is inside Vultr's own network. Overrides:
#
#   ISO_URL         Use this URL and skip build/copy/serve entirely.
#                   Must be plain HTTP, no redirects, and must end in
#                   /nix-config-installer.iso — Vultr names the ISO
#                   after the URL's basename and step 2 matches on that
#                   exact filename.
#   ISO_SERVE_SSH   ssh target that serves the ISO (default root@<ip>).
#                   Needs python3 or busybox, plus ISO_SERVE_PORT free
#                   and reachable from the public internet.
#   ISO_SERVE_PORT  Port to serve on (default 80).
#
# If Vultr drops the record anyway, the script stops and prints the
# manual recipe instead of attaching an ISO that does not exist.
#
# Requirements: curl, jq, nix, ssh/scp. An SSH key for this machine must
# be in lib/ssh-keys.nix or on github.com/mvertescher.keys.
#
# To force a rebuild of the ISO (e.g. after changing installer.nix),
# delete the old one first: it shows up in `vultr-cli iso list` /
# GET /v2/iso, then DELETE /v2/iso/{id}.

set -euo pipefail

IP="${1:?usage: provision-server.sh <ip> [host]}"
HOST="${2:-server}"
: "${VULTR_API_KEY:?set VULTR_API_KEY}"

API="https://api.vultr.com/v2"
ISO_FILENAME="nix-config-installer.iso"
ISO_URL="${ISO_URL:-}"
ISO_SERVE_SSH="${ISO_SERVE_SSH:-root@$IP}"
ISO_SERVE_PORT="${ISO_SERVE_PORT:-80}"
ISO_SERVE_DIR="/tmp/nix-config-iso"
SSH_OPTS=(-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o ConnectTimeout=5 -o BatchMode=yes)

# This repo holds .#installer-iso; the *wrapper* flake is the cwd, which
# is where nixos-anywhere and the generated hardware config belong.
PUBLIC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[[ -f flake.nix ]] ||
  { echo ">>> run this from the root of the wrapper flake (no ./flake.nix here)" >&2; exit 1; }

WORK_DIR="$(mktemp -d)"
serve_started=0

log() { echo ">>> $*" >&2; }
die() { log "ERROR: $*"; exit 1; }

vcurl() {
  curl -sf -H "Authorization: Bearer $VULTR_API_KEY" "$@"
}

stop_iso_server() {
  [[ "$serve_started" == 1 ]] || return 0
  serve_started=0
  log "stopping the ISO HTTP server on $ISO_SERVE_SSH"
  # The bracket in the pattern keeps pkill from matching this very
  # command line (which contains "http[.]server", not "http.server").
  # shellcheck disable=SC2029 # client-side expansion is intended
  ssh "${SSH_OPTS[@]}" "$ISO_SERVE_SSH" \
    "pkill -f 'http[.]server' >/dev/null 2>&1; pkill -f 'busybox httpd' >/dev/null 2>&1; rm -rf '$ISO_SERVE_DIR'; true" ||
    log "warning: could not clean up $ISO_SERVE_DIR on $ISO_SERVE_SSH"
}

cleanup() {
  stop_iso_server
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT
trap 'exit 130' INT TERM

manual_publish_help() {
  cat >&2 <<HELP
>>>
>>> Vultr would not fetch the ISO. Publish it by hand, then re-run:
>>>   1. nix build 'path:$PUBLIC_DIR#installer-iso' -o result-iso
>>>   2. copy result-iso/iso/*.iso onto any Vultr instance, named
>>>      exactly $ISO_FILENAME, and serve its directory over plain
>>>      HTTP there:  python3 -m http.server $ISO_SERVE_PORT
>>>   3. curl -sf -H "Authorization: Bearer \$VULTR_API_KEY" \\
>>>        -X POST $API/iso -H 'Content-Type: application/json' \\
>>>        -d '{"url":"http://<that-instance-ip>/$ISO_FILENAME"}'
>>>   4. poll GET $API/iso until that record reads "complete"
>>>      (if it disappears instead, the URL is one Vultr refuses).
>>>
>>> Known-bad URLs: GitHub release assets and anything else that
>>> redirects — Vultr accepts the POST, leaves the record "pending"
>>> for ~30s, then deletes it without ever contacting the host.
>>> Known-good: plain HTTP straight off a Vultr instance.
>>>
>>> Once the ISO is in the account this script finds it by filename and
>>> skips all of the above. ISO_URL=... also skips the build+serve step.
HELP
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

# Copy the ISO to ISO_SERVE_SSH and serve it over plain HTTP. Sets
# ISO_URL to the resulting URL.
serve_iso() {
  local iso_file="$1" host server cmd
  host="${ISO_SERVE_SSH##*@}"

  # shellcheck disable=SC2029 # client-side expansion is intended
  ssh "${SSH_OPTS[@]}" "$ISO_SERVE_SSH" "rm -rf '$ISO_SERVE_DIR' && mkdir -p '$ISO_SERVE_DIR'" ||
    { log "ERROR: cannot ssh to $ISO_SERVE_SSH to stage the ISO."
      log "Set ISO_SERVE_SSH to a Vultr box you can reach, or ISO_URL to a"
      log "plain-HTTP URL that already serves the ISO."
      manual_publish_help; exit 1; }

  server="$(ssh "${SSH_OPTS[@]}" "$ISO_SERVE_SSH" \
    'if command -v python3 >/dev/null 2>&1; then echo python3;
     elif command -v busybox >/dev/null 2>&1; then echo busybox;
     else echo none; fi')"
  case "$server" in
    python3) cmd="python3 -m http.server '$ISO_SERVE_PORT'" ;;
    busybox) cmd="busybox httpd -f -p '$ISO_SERVE_PORT' -h '$ISO_SERVE_DIR'" ;;
    *) log "ERROR: $ISO_SERVE_SSH has neither python3 nor busybox to serve the ISO."
       manual_publish_help; exit 1 ;;
  esac

  log "copying $ISO_FILENAME to $ISO_SERVE_SSH (multi-GB upload, be patient)"
  scp "${SSH_OPTS[@]}" "$iso_file" "$ISO_SERVE_SSH:$ISO_SERVE_DIR/$ISO_FILENAME" ||
    { log "ERROR: scp of the ISO to $ISO_SERVE_SSH failed"; manual_publish_help; exit 1; }

  log "serving it there with $server on port $ISO_SERVE_PORT"
  serve_started=1
  # shellcheck disable=SC2029 # client-side expansion is intended
  ssh "${SSH_OPTS[@]}" "$ISO_SERVE_SSH" \
    "cd '$ISO_SERVE_DIR' && nohup $cmd >'$ISO_SERVE_DIR/http.log' 2>&1 </dev/null &" ||
    { log "ERROR: could not start the HTTP server on $ISO_SERVE_SSH"; manual_publish_help; exit 1; }

  if [[ "$ISO_SERVE_PORT" == 80 ]]; then
    ISO_URL="http://$host/$ISO_FILENAME"
  else
    ISO_URL="http://$host:$ISO_SERVE_PORT/$ISO_FILENAME"
  fi

  # Vultr fetches from the public internet, so reachability from here is
  # the closest check we can make before handing over the URL.
  local attempt
  for attempt in {1..10}; do
    if curl -sfI --max-time 10 "$ISO_URL" >/dev/null; then
      log "serving $ISO_URL"
      return 0
    fi
    log "$ISO_URL not answering yet (attempt $attempt/10)"
    sleep 3
  done
  log "ERROR: $ISO_URL is not reachable from this machine."
  log "Check that port $ISO_SERVE_PORT is free on $ISO_SERVE_SSH and not"
  log "blocked by a Vultr cloud firewall; see $ISO_SERVE_DIR/http.log there."
  manual_publish_help
  exit 1
}

# Poll an ISO record until it reads "complete". A record that vanishes
# is Vultr's silent refusal of the URL, not a transient error.
wait_for_iso() {
  local id="$1" deadline=$((SECONDS + 1800)) code status body="$WORK_DIR/iso.json"
  log "waiting for Vultr to download the ISO (up to 30m)"
  status="pending"
  while :; do
    code="$(curl -s -o "$body" -w '%{http_code}' \
      -H "Authorization: Bearer $VULTR_API_KEY" "$API/iso/$id" || echo 000)"
    case "$code" in
      200)
        status="$(jq -r '.iso.status // empty' <"$body")"
        case "$status" in
          complete) log "Vultr finished downloading the ISO"; return 0 ;;
          pending | "") ;;
          *)
            log "ERROR: Vultr reports ISO status \"$status\" for record $id"
            manual_publish_help
            exit 1
            ;;
        esac
        ;;
      404 | 410)
        log "ERROR: Vultr deleted ISO record $id without downloading it."
        log "That is how it refuses a URL it will not fetch: accept the"
        log "POST, sit in \"pending\" ~30s, then drop the record. The URL"
        log "it was given was: ${ISO_URL:-<unset>}"
        manual_publish_help
        exit 1
        ;;
      *) log "transient Vultr API response $code while polling; retrying" ;;
    esac
    ((SECONDS < deadline)) || {
      log "ERROR: ISO record $id still \"$status\" after 30m"
      manual_publish_help
      exit 1
    }
    sleep 10
  done
}

# --- 1. Instance lookup ---
instance_id="$(vcurl "$API/instances?per_page=500" |
  jq -r --arg ip "$IP" '[.instances[] | select(.main_ip == $ip) | .id] | first // empty')"
[[ -n "$instance_id" ]] || die "no Vultr instance with main IP $IP"
log "instance: $instance_id"

# --- 2. Ensure installer ISO exists in the Vultr account ---
# Match only our custom installer by its exact filename — the account may
# also hold stock NixOS ISOs, which have no SSH keys baked in.
iso_id="$(vcurl "$API/iso?per_page=500" |
  jq -r --arg f "$ISO_FILENAME" \
    '[.isos[] | select(.filename == $f and .status == "complete") | .id] | first // empty')"

if [[ -z "$iso_id" ]]; then
  log "no $ISO_FILENAME in Vultr account; publishing one"

  if [[ -n "$ISO_URL" ]]; then
    log "using ISO_URL as given: $ISO_URL"
    [[ "$ISO_URL" == http://* ]] ||
      log "warning: ISO_URL is not plain http:// — Vultr may silently refuse it"
    [[ "$ISO_URL" == */"$ISO_FILENAME" ]] ||
      die "ISO_URL must end in /$ISO_FILENAME (Vultr names the ISO from the URL basename)"
  else
    log "building the installer ISO"
    nix build "path:$PUBLIC_DIR#installer-iso" -o "$WORK_DIR/result-iso"
    iso_file="$(find -L "$WORK_DIR/result-iso/iso" -name '*.iso' -print -quit)"
    [[ -n "$iso_file" ]] || die "ISO build produced no .iso"
    serve_iso "$iso_file"
  fi

  log "asking Vultr to fetch $ISO_URL"
  post_body="$(vcurl -X POST "$API/iso" -H 'Content-Type: application/json' \
    -d "$(jq -n --arg url "$ISO_URL" '{url: $url}')")" ||
    { log "ERROR: POST $API/iso was rejected"; manual_publish_help; exit 1; }
  iso_id="$(jq -r '.iso.id // empty' <<<"$post_body")"
  [[ -n "$iso_id" ]] ||
    { log "ERROR: POST $API/iso returned no iso.id: $post_body"; manual_publish_help; exit 1; }

  wait_for_iso "$iso_id"
  stop_iso_server
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
vcurl -X POST "$API/instances/$instance_id/reboot" >/dev/null 2>&1 || true
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
