#!/usr/bin/env bash
# frame.sh — a design SVG at a moment in time.
#
#   frame.sh --at 0.4 docs/neomil/login-trace.svg /tmp/login-0.4.png
#   frame.sh --at 0 --no-photo docs/kitsch/store-trace.svg /tmp/store.png
#
# The traces carry their motion as SMIL (`<animate>`, `<animateTransform>`,
# `<set>` on the element they move; docs/PIPELINE.md, "Motion"). rsvg
# ignores SMIL and renders frame 0, which is why every static gate and
# golden is unaffected by an annotation -- and why none of them can see
# one. This renders the frame at `--at` seconds instead: the SVG is
# copied with a script appended that pauses the document's timeline and
# seeks it, and headless Firefox screenshots the result. Firefox because
# it is the one rasteriser on hand that runs SMIL and can be told the
# time; its frame 0 of the neomil login trace differed from rsvg's on
# 0.016% of pixels (2026-09-06, --no-photo, 8-level fuzz), so a frame
# from here is comparable with one from the static pipeline.
#
# The seek has to be a <script> *inside* the SVG. Loading the file
# through an <object> and seeking its contentDocument from the page
# screenshots frame 0 every time -- --screenshot fires before the
# object's load handler has run.
#
# `--no-photo` hides `class="photo"` elements the way G2i does, for a
# frame that is to be compared with an implementation capture; without
# it the frame is the trace as it reads against the photo, like
# triptych.sh's row 2.
#
# Needs firefox on PATH (terra has it) and, for the fonts, a fontconfig
# that knows Rajdhani and Orbitron -- the desktop's does. A scratch
# profile is made per run so a desktop Firefox is neither disturbed nor
# consulted; headless with a *missing* profile directory hangs, which is
# why the directory is created first.
#
# Times are SVG document seconds: `--at 0.4` is 400 ms after the
# document's begin, which is what the traces' `begin=` values count
# from. Elements with `begin="click"` or other event-based starts never
# run under this and render in their unstarted state, on purpose --
# a trace that wants an interaction shown at a moment gives it a
# clock-based begin as well (see the convention).

set -euo pipefail

at=0
photo=1
size=1600x900
args=()
while [ $# -gt 0 ]; do
  case "$1" in
    --at) at="$2"; shift 2 ;;
    --at=*) at="${1#--at=}"; shift ;;
    --no-photo) photo=0; shift ;;
    --size) size="$2"; shift 2 ;;
    --size=*) size="${1#--size=}"; shift ;;
    -h|--help) sed -n '2,45p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    -*) echo "frame.sh: unknown option $1" >&2; exit 2 ;;
    *) args+=("$1"); shift ;;
  esac
done
[ "${#args[@]}" -eq 2 ] || { echo "usage: frame.sh [--at SECONDS] [--no-photo] [--size WxH] SVG OUT.png" >&2; exit 2; }
svg="$(realpath "${args[0]}")"
out="$(realpath -m "${args[1]}")"
[ -f "$svg" ] || { echo "frame.sh: no such file $svg" >&2; exit 1; }
command -v firefox >/dev/null || { echo "frame.sh: firefox not on PATH" >&2; exit 1; }

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/profile"

# The seek, and the photo-class rule when asked for, written into a
# copy next to the original so relative hrefs still resolve. The style
# goes right after the root element the way fidelity_check.sh does it.
frame_svg="$(dirname "$svg")/.frame-$$-$(basename "$svg")"
trap 'rm -rf "$scratch" "$frame_svg"' EXIT
{
  if [ "$photo" -eq 0 ]; then
    sed '0,/<svg[^>]*>/s//&<style>.photo{display:none}<\/style>/' "$svg"
  else
    cat "$svg"
  fi
} | perl -pe 's|</svg>\s*$|<script>document.documentElement.pauseAnimations(); document.documentElement.setCurrentTime('"$at"');</script>\n</svg>\n|' > "$frame_svg"

# Firefox writes the screenshot at the window size, not the SVG's, so
# the two must agree or the frame is letterboxed.
firefox --headless --no-remote --profile "$scratch/profile" \
  --window-size="${size/x/,}" --screenshot "$out" "file://$frame_svg" \
  >"$scratch/firefox.log" 2>&1 || {
  echo "frame.sh: firefox failed:" >&2; cat "$scratch/firefox.log" >&2; exit 1; }
[ -s "$out" ] || { echo "frame.sh: no screenshot written" >&2; cat "$scratch/firefox.log" >&2; exit 1; }
