#!/usr/bin/env bash
# triptych.sh — one image per era x screen, three rows top to bottom:
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
# Usage:
#   triptych.sh                        # every era x login,dashboard,mailbox,store
#   triptych.sh neomil                 # one era
#   triptych.sh neomil dashboard       # one pair
#   triptych.sh --bin-dir /tmp/bins kitsch
#   triptych.sh --out /tmp/tri --no-labels
#
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
targets=()
while [ $# -gt 0 ]; do
  case "$1" in
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
    out="$out_dir/$era-$screen.png"

    # The photos are 3840x2160; the other two rows are 1600x900. Fit the
    # photo into the same box rather than assuming its aspect.
    "${magick[@]}" "$src" -resize "${w}x${h}" -background black \
      -gravity center -extent "${w}x${h}" "$photo" || { echo "FAIL $era/$screen: photo"; fail=1; continue; }
    "${rsvg[@]}" -w "$w" -h "$h" "$svg" -o "$trace" 2>/dev/null \
      || { echo "FAIL $era/$screen: rsvg-convert on $svg"; fail=1; continue; }
    # `env -u FONTCONFIG_FILE`: the app embeds its own faces and the
    # goldens were captured with no fontconfig override; keep it that way
    # so row 3 is the same capture the matrix would take.
    env -u FONTCONFIG_FILE "$here/render.sh" \
      --era "$era" --size "${w}x${h}" --bin "$bin" --out "$impl" >/dev/null \
      || { echo "FAIL $era/$screen: render.sh could not capture $app (see $impl.log)"; fail=1; continue; }

    caption "$photo" "$era / $screen — 1 source photo: images/$(basename "$src")"
    caption "$trace" "2 trace: docs/$era/$screen-trace.svg"
    caption "$impl"  "3 iced: $app --era $era"
    "${magick[@]}" "$photo" "$trace" "$impl" -append "$out" \
      || { echo "FAIL $era/$screen: append"; fail=1; continue; }
    echo "$out"
  done
done
exit $fail
