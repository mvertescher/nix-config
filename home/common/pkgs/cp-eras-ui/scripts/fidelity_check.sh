#!/usr/bin/env bash
# fidelity_check.sh — run the design-pipeline gates (see docs/PIPELINE.md).
#
#   G2 (svg -> iced): rasterise docs/<era>/<screen>.svg and structurally
#       compare against the headless golden, so a schematic and its
#       implementation are held to agree.
#   G1 (source -> svg): compare the same SVG render against the original
#       Behance source in images/ (gitignored), so a trace is faithful to
#       the material. Only eras/screens with a downloaded source can run G1.
#   G1i (source -> svg, inventory): the same pair, compared as shape
#       inventories by extract_spec.py + spec_diff.py rather than as grid
#       statistics. G1 answers "is the mass in the right places", which a
#       trace can satisfy while drawing the wrong widgets entirely; G1i
#       answers "does it draw the same things", and unlike G1 it has a
#       pass/fail. Run it whenever a trace changes.
#
# Usage:
#   fidelity_check.sh                 # G2 for all eras x bar,dashboard
#   fidelity_check.sh neomil          # G2 for neomil x bar,dashboard
#   fidelity_check.sh neomil dashboard
#   fidelity_check.sh --source neomil dashboard   # G1 (needs images/<src>.png)
#   fidelity_check.sh --inventory neomil dashboard  # G1i, exits nonzero on fail
#
# Needs: rsvg-convert and a python3 with Pillow. Supply them via
# PILLOW_PYTHON=/path/to/python3 (must import PIL) and RSVG=rsvg-convert, or
# let this script fall back to `nix shell nixpkgs#librsvg` / a nix-built
# python3.withPackages (ps: [ ps.pillow ]). --inventory additionally needs
# numpy and scipy; the same fallback builds them.
set -u

here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
crate=$(dirname "$here")
eras=(entropism kitsch neomil neokitsch)
screens=(bar dashboard)

# --- tool discovery ---------------------------------------------------------
# $1 is the import list the caller needs, as a python import statement.
find_python() {
  local need=${1:-PIL} expr_pkgs=${2:-[ ps.pillow ]}
  if [ -n "${PILLOW_PYTHON:-}" ] && "$PILLOW_PYTHON" -c "import $need" 2>/dev/null; then
    echo "$PILLOW_PYTHON"; return 0
  fi
  if command -v python3 >/dev/null && python3 -c "import $need" 2>/dev/null; then
    command -v python3; return 0
  fi
  # nix escape hatch: a python3 with what we need (same shape tests/visual.nix builds)
  local py
  py=$(nix build --impure --no-link --print-out-paths \
        --expr "with import <nixpkgs> {}; python3.withPackages (ps: $expr_pkgs)" \
        2>/dev/null | tail -1) || return 1
  [ -n "${py:-}" ] && [ -x "$py/bin/python3" ] && echo "$py/bin/python3"
}
find_rsvg() {
  if [ -n "${RSVG:-}" ] && command -v "$RSVG" >/dev/null; then
    echo "$RSVG"; return 0
  fi
  if command -v rsvg-convert >/dev/null; then
    echo "rsvg-convert"; return 0
  fi
  # nix shell escape hatch; used per invocation below because the shell is transient
  echo "nix-shell-rsvg"
}

python_bin=$(find_python) || { echo "no python3 with Pillow (set PILLOW_PYTHON)" >&2; exit 1; }
rsvg_bin=$(find_rsvg)
[ "$rsvg_bin" = "nix-shell-rsvg" ] && rsvg_bin="nix shell nixpkgs#librsvg --command rsvg-convert"

# --- args -------------------------------------------------------------------
mode="G2"
targets=()
if [ $# -gt 0 ] && [ "$1" = "--source" ]; then
  mode="G1"; shift
elif [ $# -gt 0 ] && [ "$1" = "--inventory" ]; then
  mode="G1i"; shift
fi
for a in "$@"; do targets+=("$a"); done

selected_eras=("${eras[@]}")
selected_screens=("${screens[@]}")
if [ ${#targets[@]} -ge 2 ]; then
  selected_eras=("${targets[0]}")
  selected_screens=("${targets[1]}")
elif [ ${#targets[@]} -eq 1 ]; then
  selected_eras=("${targets[0]}")
fi

if [ "$mode" = "G1i" ]; then
  python_bin=$(find_python "numpy, scipy, PIL" "[ ps.numpy ps.scipy ps.pillow ]") || {
    echo "no python3 with numpy+scipy+Pillow (set PILLOW_PYTHON)" >&2; exit 1; }
fi

echo "== cp-eras-ui fidelity check ($mode) =="
echo "eras:   ${selected_eras[*]}"
echo "screens:${selected_screens[*]}"
echo

overall_fail=0
for era in "${selected_eras[@]}"; do
  for screen in "${selected_screens[@]}"; do
    svg="$crate/docs/$era/$screen.svg"
    [ -f "$svg" ] || { echo "SKIP $era/$screen: no $svg"; continue; }

    render="$crate/images/_fid-$era-$screen.png"
    $rsvg_bin -w 1600 "$svg" -o "$render" 2>/dev/null || { echo "FAIL $era/$screen: rsvg-convert errored"; overall_fail=1; continue; }

    src=""
    case "$era-$screen" in
      neomil-dashboard) src="$crate/images/img-07-dashboard.png" ;;
      neomil-target-app) src="$crate/images/img-08-main.png" ;;
      # NB the entropism files are named the wrong way round (docs/sources.md):
      # the module-hub material lives in entropism-store.png.
      entropism-dashboard) src="$crate/images/entropism-store.png" ;;
      kitsch-dashboard) src="$crate/images/kitsch-dashboard.png" ;;
      neokitsch-dashboard) src="$crate/images/neokitsch-dashboard.png" ;;
      *) ;;
    esac

    if [ "$mode" = "G1i" ]; then
      if [ -z "$src" ] || [ ! -f "$src" ]; then
        echo "SKIP G1i $era/$screen: no source downloaded (see docs/sources.md)"
        continue
      fi
      # G1 is about the trace, so prefer <screen>-trace.svg where one exists:
      # dashboard.svg is shaped like the app, dashboard-trace.svg like the
      # photo, and only the second is the thing the material can judge.
      g1svg="$crate/docs/$era/$screen-trace.svg"
      [ -f "$g1svg" ] || g1svg="$svg"
      render="$crate/images/_fid-$era-$screen.png"
      $rsvg_bin -w 1600 -h 900 "$g1svg" -o "$render" 2>/dev/null || {
        echo "FAIL $era/$screen: rsvg-convert errored"; overall_fail=1; continue; }
      echo "--- G1i $era/$screen: $(basename "$g1svg") vs source, as shape inventories ---"
      "$python_bin" "$here/extract_spec.py" "$src"    -o "/tmp/spec-$era-$screen-src.json" || { overall_fail=1; continue; }
      "$python_bin" "$here/extract_spec.py" "$render" -o "/tmp/spec-$era-$screen-svg.json" || { overall_fail=1; continue; }
      # Rotated / translucent design languages (kitsch's fans, neokitsch's
      # cascades) fragment unstably under the axis-aligned templates, so
      # their verdict comes from ink-family placement instead.
      gate="shapes"
      case "$era" in kitsch|neokitsch) gate="inks" ;; esac
      "$python_bin" "$here/spec_diff.py" --gate "$gate" \
        "/tmp/spec-$era-$screen-src.json" "/tmp/spec-$era-$screen-svg.json" || overall_fail=1
      echo
      continue
    fi

    if [ "$mode" = "G2" ]; then
      golden="$crate/tests/golden/$screen-$era-1600x220.png"
      [ "$screen" = "dashboard" ] && golden="$crate/tests/golden/$screen-$era-1600x900.png"
      [ -f "$golden" ] || { echo "SKIP $era/$screen: no golden $golden"; continue; }
      echo "--- G2 $era/$screen: svg vs golden ---"
      "$python_bin" "$here/compare_ref.py" "$render" "$golden" \
        --out "/tmp/fid-$era-$screen" 2>/dev/null |
        grep -E '^(layout|edge density|palette overlap|reading)'
    else
      # G1: source named in docs/sources.md for this era/screen.
      if [ -z "$src" ] || [ ! -f "$src" ]; then
        echo "SKIP G1 $era/$screen: no source downloaded (see docs/sources.md)"
        continue
      fi
      echo "--- G1 $era/$screen: source vs svg ---"
      "$python_bin" "$here/compare_ref.py" "$src" "$render" \
        --out "/tmp/fid-$era-$screen" 2>/dev/null |
        grep -E '^(layout|edge density|palette overlap|reading)'
    fi
    echo
  done
done
rm -f "$crate"/images/_fid-*.png
exit $overall_fail