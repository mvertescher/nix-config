#!/usr/bin/env bash
# triptych.sh — one image per era x screen, three rows top to bottom (a
# fourth with `--diff`):
#
#   1. the Behance source photo from images/ (gitignored; download_images.py)
#   2. docs/<era>/<screen>-trace.svg rasterised at 1600x900
#   3. a headless capture of the matching binary via scripts/render.sh
#
# This is the "does it look like the thing" view that the gates cannot
# give. fidelity_check.sh reports numbers and writes side-by-sides for one
# pair at a time (source vs trace under G1i, trace vs implementation under
# G2i); a review wants all three stages of one screen stacked so the eye
# can walk photo -> trace -> Rust and see where each step lost something.
#
# Unlike G2i the trace is rendered *with* its `class="photo"` elements --
# sharpening halos, neokitsch's blurred self-copy -- because row 2 is
# meant to be read against the photo above it, and the trace is a record
# of the photo. Row 3 is told not to draw those, so a soft glow present in
# rows 1-2 and absent in row 3 is expected, not a finding.
#
# `--diff` adds a fourth row that points at what rows 2 and 3 disagree
# on, so the review does not have to find it by eye: the trace and the
# capture subtracted per pixel (largest channel difference, square-rooted
# so a small drift still shows) on a black -> yellow -> red ramp -- 1-2
# levels stay black, 8 is a dim yellow, full yellow by ~65, red at 255 --
# over a dimmed grey copy of the trace for orientation, and the caption
# gives the share of pixels off by more than 8 levels. This row
# diffs the trace *without* its `class="photo"` elements, exactly the
# design G2i scores, so it lights up findings and not the expected halo.
# Text is always lit a little (two rasterisers never agree on AA), a
# filled shape lit solid is a colour miss, an outline lit is a placement
# miss. Only rows 2 and 3 are diffed: the photo is a different size,
# a different medium, and G1/G1i already compare it.
#
# Usage:
#   triptych.sh                        # every era x login,dashboard,mailbox,store
#   triptych.sh neomil                 # one era
#   triptych.sh neomil dashboard       # one pair
#   triptych.sh --diff kitsch dashboard
#   triptych.sh --bin-dir /tmp/bins kitsch
#   triptych.sh --at 0.9 --diff neomil login
#   triptych.sh --out /tmp/tri --no-labels
#
#   --diff          add the trace-vs-iced difference as a fourth row.
#   --at SECONDS    the moment to show: rows 2 and 4 come from frame.sh
#                   (headless Firefox running the trace's SMIL, seeked to
#                   this time) and row 3 from render.sh --at, the app's
#                   clock frozen there. Written as <era>-<screen>-at<t>.png
#                   next to the static one. Without it the rows are the
#                   trace at rest, rasterised by rsvg as ever, and the app
#                   at `motion::REST` (docs/PIPELINE.md, Motion).
#   --bin-dir DIR   where the cp-eras-ui-<screen> binaries are; default
#                   target/debug, which `cargo build --bin ...` writes.
#   --out DIR       where to write <era>-<screen>.png; default
#                   images/triptych/ (under the gitignored images/).
#   --no-labels     omit the caption strip above each row.
#
# The bar is not a screen here: it has no photo, so it has no row 1.
#
# Needs rsvg-convert and ImageMagick 7 (`magick`); either on PATH or
# supplied by `nix shell nixpkgs#librsvg nixpkgs#imagemagick`, which this
# script falls back to per invocation. The binaries are never built here:
#
#   nix-shell shell.nix --run 'cargo build --bin cp-eras-ui-login \
#     --bin cp-eras-ui-dashboard --bin cp-eras-ui-mailbox --bin cp-eras-ui-store'
set -u

here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
crate=$(dirname "$here")
eras=(entropism kitsch neomil neokitsch)
screens=(login dashboard mailbox store)
w=1600; h=900

# --- args -------------------------------------------------------------------
bin_dir="$crate/target/debug"
out_dir="$crate/images/triptych"
labels=1
diff=0
at=""
targets=()
while [ $# -gt 0 ]; do
  case "$1" in
    --diff)      diff=1; shift ;;
    --at)        at=$2; shift 2 ;;
    --bin-dir)   bin_dir=$2; shift 2 ;;
    --out)       out_dir=$2; shift 2 ;;
    --no-labels) labels=0; shift ;;
    -h|--help)   sed -n '2,/^set -u/p' "$0" | sed '$d' | cut -c3-; exit 0 ;;
    *)           targets+=("$1"); shift ;;
  esac
done
[ ${#targets[@]} -ge 1 ] && eras=("${targets[0]}")
[ ${#targets[@]} -ge 2 ] && screens=("${targets[1]}")

# --- tools ------------------------------------------------------------------
# Same escape hatch as fidelity_check.sh: use what is on PATH, else a
# transient nix shell per call (cached after the first, so cheap enough).
if command -v rsvg-convert >/dev/null; then
  rsvg=(rsvg-convert)
else
  rsvg=(nix shell nixpkgs#librsvg --command rsvg-convert)
fi
if command -v magick >/dev/null; then
  magick=(magick)
else
  magick=(nix shell nixpkgs#imagemagick --command magick)
fi

scratch=$(mktemp -d "${TMPDIR:-/tmp}/tri.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

# Row 2 and the diff's reference: the trace rasterised at rest by rsvg,
# or at `--at` by frame.sh. $1 svg, $2 out, $3 "photo"|"no-photo".
trace_png() {
  if [ -n "$at" ]; then
    local opt=()
    [ "$3" = no-photo ] && opt=(--no-photo)
    "$here/frame.sh" --at "$at" "${opt[@]}" --size "${w}x${h}" "$1" "$2"
  elif [ "$3" = no-photo ]; then
    # The sed is fidelity_check.sh's.
    local hidden="$scratch/$(basename "$2" .png).svg"
    sed '0,/<svg[^>]*>/s//&<style>.photo{display:none}<\/style>/' "$1" > "$hidden"
    "${rsvg[@]}" -w "$w" -h "$h" "$hidden" -o "$2" 2>/dev/null
  else
    "${rsvg[@]}" -w "$w" -h "$h" "$1" -o "$2" 2>/dev/null
  fi
}

# Pin the fonts for the SVG render, exactly as fidelity_check.sh does:
# the traces name Rajdhani / Orbitron and the crate ships both in fonts/,
# and without this a box that lacks them renders row 2 in a fallback face
# with different widths. The caption strip uses the same file directly.
cat > "$scratch/fonts.conf" <<EOF
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <include ignore_missing="yes">/etc/fonts/fonts.conf</include>
  <dir>$crate/fonts</dir>
  <cachedir>$scratch/fc-cache</cachedir>
</fontconfig>
EOF
export FONTCONFIG_FILE="$scratch/fonts.conf"
label_font="$crate/fonts/Rajdhani-SemiBold.ttf"

# The Behance photo for a pair; the same table fidelity_check.sh carries
# for G1i, including the entropism swap (the hub is entropism-store.png
# and the store is entropism-dashboard.png -- docs/sources.md).
source_for() {
  case "$1-$2" in
    neomil-login)        echo "img-06-private.png" ;;
    neomil-dashboard)    echo "img-07-dashboard.png" ;;
    neomil-mailbox)      echo "img-08-main.png" ;;
    neomil-store)        echo "img-09-store.png" ;;
    entropism-login)     echo "entropism-login.png" ;;
    entropism-dashboard) echo "entropism-store.png" ;;
    entropism-mailbox)   echo "entropism-mail.png" ;;
    entropism-store)     echo "entropism-dashboard.png" ;;
    kitsch-mailbox|neokitsch-mailbox) echo "$1-mail.png" ;;
    *)                   echo "$1-$2.png" ;;
  esac
}

# Put a caption strip above an image, in place. $1 file, $2 text.
caption() {
  [ "$labels" = 1 ] || return 0
  "${magick[@]}" "$1" -gravity northwest -background '#181818' -splice 0x44 \
    -font "$label_font" -pointsize 30 -fill '#e0e0e0' -annotate +14+6 "$2" "$1"
}

# The fourth row: $1 trace (photo elements hidden), $2 capture, $3 out.
# Prints the share of pixels off by more than 8 levels, as a percentage.
# 8 is where the G2i verdict was measured to turn (docs/PIPELINE.md, the
# sRGB-vs-linear note): pasting every pixel further off than that into
# the capture moved the kitsch dashboard from FAIL to 49%.
heat() {
  # subtract 0.8% = a 2-level floor: inside a converged fill the two
  # rasterisers still disagree by a level or two, and that is not news.
  "${magick[@]}" "$1" "$2" -compose difference -composite \
    -separate -evaluate-sequence max \
    -evaluate subtract 0.8% -evaluate pow 0.5 \
    \( \( -size 1x128 gradient:'black-#ffe030' \) \
       \( -size 1x128 gradient:'#ffe030-#ff2020' \) -append \) -clut \
    "$3" || return 1
  "${magick[@]}" "$1" -colorspace gray -evaluate multiply 0.22 -colorspace sRGB \
    "$3" -compose lighten -composite "$3" || return 1
  # `compare -metric AE` counts the differing pixels; fuzz 3.2% of the
  # quantum range is 8 levels. It reports "count (fraction)" on stderr.
  "${magick[@]}" compare -metric AE -fuzz 3.2% "$1" "$2" null: 2>&1 \
    | sed -n 's/.*(\([0-9.e-]*\)).*/\1/p' \
    | awk '{ printf "%.1f", $1 * 100 }' | grep . || return 1
}

mkdir -p "$out_dir"
fail=0
for era in "${eras[@]}"; do
  for screen in "${screens[@]}"; do
    src="$crate/images/$(source_for "$era" "$screen")"
    svg="$crate/docs/$era/$screen-trace.svg"
    app="cp-eras-ui-$screen"
    bin="$bin_dir/$app"
    skip=""
    [ -f "$src" ] || skip="no source $src (scripts/download_images.py)"
    [ -f "$svg" ] || skip="no trace $svg"
    [ -x "$bin" ] || skip="no binary $bin"
    if [ -n "$skip" ]; then echo "SKIP $era/$screen: $skip"; fail=1; continue; fi

    photo="$scratch/$era-$screen-photo.png"
    trace="$scratch/$era-$screen-trace.png"
    impl="$scratch/$era-$screen-impl.png"
    out="$out_dir/$era-$screen${at:+-at$at}.png"

    # The photos are 3840x2160; the other two rows are 1600x900. Fit the
    # photo into the same box rather than assuming its aspect.
    "${magick[@]}" "$src" -resize "${w}x${h}" -background black \
      -gravity center -extent "${w}x${h}" "$photo" || { echo "FAIL $era/$screen: photo"; fail=1; continue; }
    trace_png "$svg" "$trace" photo \
      || { echo "FAIL $era/$screen: rasterising $svg"; fail=1; continue; }
    # `env -u FONTCONFIG_FILE`: the app embeds its own faces and the
    # goldens were captured with no fontconfig override; keep it that way
    # so row 3 is the same capture the matrix would take.
    env -u FONTCONFIG_FILE "$here/render.sh" \
      --era "$era" --size "${w}x${h}" --bin "$bin" --out "$impl" --at "${at:-2.4}" >/dev/null \
      || { echo "FAIL $era/$screen: render.sh could not capture $app (see $impl.log)"; fail=1; continue; }

    rows=("$photo" "$trace" "$impl")
    if [ "$diff" = 1 ]; then
      # The same design G2i scores: the trace with its `class="photo"`
      # elements hidden.
      g2i="$scratch/$era-$screen-g2i.png"
      heat="$scratch/$era-$screen-diff.png"
      trace_png "$svg" "$g2i" no-photo \
        || { echo "FAIL $era/$screen: rasterising $svg without photo"; fail=1; continue; }
      off=$(heat "$g2i" "$impl" "$heat") \
        || { echo "FAIL $era/$screen: diff"; fail=1; continue; }
      rows+=("$heat")
    fi

    caption "$photo" "$era / $screen — 1 source photo: images/$(basename "$src")"
    caption "$trace" "2 trace: docs/$era/$screen-trace.svg${at:+ at ${at}s (frame.sh)}"
    caption "$impl"  "3 iced: $app --era $era${at:+ at ${at}s (render.sh --at)}"
    [ "$diff" = 1 ] && caption "$heat" \
      "4 diff: |trace − iced|, trace without class=photo — ${off}% of pixels off by >8 levels"
    "${magick[@]}" "${rows[@]}" -append "$out" \
      || { echo "FAIL $era/$screen: append"; fail=1; continue; }
    echo "$out"
  done
done
exit $fail
