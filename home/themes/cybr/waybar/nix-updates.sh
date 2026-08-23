#!/usr/bin/env bash
# ---------------------------------------
# custom/updates for waybar, NixOS flavour
#
# Replaces upstream cybr-waybar's `waybar-updates`, an Arch/AUR pacman
# helper that is not in nixpkgs and would mean nothing here. Wired up in
# ../default.nix, which builds this with writeShellApplication and
# templates the store path into cybr-waybar/modules.jsonc.
#
# Design rules, because this runs on a timer inside the bar:
#   - no network, ever: everything read is already on this machine.
#   - no store-wide scans on the common path; the only nix call happens
#     when a new generation is actually waiting, and is wrapped in a
#     timeout so the module can never wedge the bar.
#   - any failure prints empty JSON and exits 0, so the module hides
#     itself instead of erroring the way the pacman helper did.
#
# Signals reported:
#   reboot pending  /run/booted-system != /run/current-system, i.e. a
#                   switch landed a generation the running kernel and
#                   userland are not on yet. `nix store diff-closures`
#                   fills in the tooltip.
#   stale           the current generation, and optionally a flake.lock
#                   passed as $1, are older than STALE_DAYS.
# ---------------------------------------

set -euo pipefail

STALE_DAYS=7   # nag threshold, in days
DIFF_LINES=12  # tooltip cap for the closure diff

# Anything unexpected: hide, do not error.
hide() { printf '{}\n'; exit 0; }
trap hide ERR

lock=${1:-}
now=$(date +%s)

reasons=()
tooltip=()
class=""

# --- pending reboot -------------------------------------------------
booted=$(readlink -f /run/booted-system 2>/dev/null || true)
current=$(readlink -f /run/current-system 2>/dev/null || true)
if [[ -n $booted && -n $current && $booted != "$current" ]]; then
  class="reboot"
  reasons+=("reboot pending")
  # Only reached while a reboot is actually pending, and diff-closures is
  # a local database read (~0.2s here); the timeout is the backstop.
  diff=""
  if command -v nix >/dev/null 2>&1; then
    # diff-closures colours its output whether or not it is on a tty,
    # and NO_COLOR does not reach it, so strip SGR escapes before they
    # end up in a pango tooltip.
    diff=$(timeout 5s nix store diff-closures \
      /run/booted-system /run/current-system 2>/dev/null |
      sed 's/\x1b\[[0-9;]*m//g' || true)
  fi
  if [[ -n $diff ]]; then
    n=$(printf '%s\n' "$diff" | wc -l)
    tooltip+=("Reboot pending: $n path(s) differ from the booted system")
    tooltip+=("$(printf '%s\n' "$diff" | head -n "$DIFF_LINES")")
    if ((n > DIFF_LINES)); then
      tooltip+=("... and $((n - DIFF_LINES)) more")
    fi
  else
    tooltip+=("Reboot pending: booted and current system differ")
  fi
fi

# --- age of the current generation ----------------------------------
# mtime of the profile symlink itself (not its target: store paths all
# carry mtime 1) is the time of the last successful switch.
gen=/nix/var/nix/profiles/system
if [[ -L $gen ]]; then
  built=$(stat -c %Y "$gen" 2>/dev/null || echo 0)
  days=$(((now - built) / 86400))
  if ((days >= STALE_DAYS)); then
    reasons+=("last switch ${days}d ago")
    [[ -n $class ]] || class="stale"
  fi
  tooltip+=("Last switch: ${days} day(s) ago")
fi

# --- age of the flake inputs ----------------------------------------
# Optional: the path of a flake.lock, if the caller knows where the
# config repo lives. Newest lastModified across the lock is the date of
# the last `nix flake update`.
if [[ -n $lock && -r $lock ]]; then
  locked=$(jq -r '[.nodes[].locked.lastModified // empty] | max // empty' \
    "$lock" 2>/dev/null || true)
  if [[ -n $locked ]]; then
    days=$(((now - locked) / 86400))
    if ((days >= STALE_DAYS)); then
      reasons+=("inputs ${days}d old")
      [[ -n $class ]] || class="stale"
    fi
    tooltip+=("Flake inputs locked: ${days} day(s) ago")
  fi
fi

# --- render ---------------------------------------------------------
if ((${#reasons[@]} == 0)); then
  # alt "updated" maps to an empty icon and the text is empty, so the
  # module disappears from the bar rather than sitting there idle.
  jq -nc '{text: "", alt: "updated", tooltip: ""}'
  exit 0
fi

summary=$(printf '%s, ' "${reasons[@]}")
summary=${summary%, }
body=$(printf '%s\n' "${tooltip[@]}")

jq -nc --arg text "$summary" --arg tooltip "$body" --arg class "$class" \
  '{text: $text, alt: "pending-updates", tooltip: $tooltip, class: $class}'
