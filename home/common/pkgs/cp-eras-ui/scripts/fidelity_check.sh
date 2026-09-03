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
#   G2i (svg -> iced, inventory): what G1i is to G1, this is to G2. The
#       design SVG and a live headless capture of the matching binary,
#       compared as shape inventories with a pass/fail. This is the gate
#       an SVG->iced conversion iterates against: G2's grid correlation
#       moves smoothly and never says "done", whereas this names the
#       widgets the implementation has not drawn yet. The capture comes
#       from scripts/render.sh, so it needs a built binary but not a nix
#       build of the crate.
#
# Usage:
#   fidelity_check.sh                 # G2 for all eras x every screen
#   fidelity_check.sh neomil          # G2 for neomil x every screen
#   fidelity_check.sh neomil dashboard
#   fidelity_check.sh --source neomil dashboard   # G1 (needs images/<src>.png)
#   fidelity_check.sh --source neomil             # G1 for login,dashboard,mailbox,store
#   fidelity_check.sh --inventory neomil dashboard  # G1i, exits nonzero on fail
#   fidelity_check.sh --inventory neomil            # G1i for login,dashboard,mailbox,store
#   fidelity_check.sh --implementation neomil bar   # G2i, exits nonzero on fail
#   fidelity_check.sh --implementation kitsch       # G2i for every screen with a design SVG
#   fidelity_check.sh --implementation --bin-dir /tmp/bins neomil bar
#
# Screens: bar, dashboard, login, mailbox, store. The design for a screen is
# docs/<era>/<screen>-trace.svg (photo-shaped, gated against its source),
# or docs/<era>/bar.svg for the bar, which has no photo. Every gate uses
# the same file; there is no app-shaped composite any more.
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
# G2 and G2i are about the implementation, so they default to every screen
# that has a design to be judged against -- the bar included, which G1/G1i
# cannot touch because no Behance screen shows one.
screens=(bar dashboard login mailbox store)
g2i_screens=("${screens[@]}")
# G1/G1i are about traces, and every era x screen with a source has one,
# so the source gates default to the four sourced screens.
g1i_screens=(login dashboard mailbox store)

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

# Per-process scratch for the SVG renders. This used to be images/_fid-*.png
# with a blanket rm on exit, which let concurrent runs delete each other's
# intermediates mid-gate.
scratch=$(mktemp -d "${TMPDIR:-/tmp}/fid.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

# Pin the fonts. The traces set Rajdhani / Orbitron by name and the crate
# ships both in fonts/; without this, a rsvg-convert from `nix shell` on a
# box that lacks them silently falls back and text widths drift 3-5px,
# which the gates then report as trace error. A private fontconfig that
# includes the system config and adds fonts/ makes the render the same
# everywhere.
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

python_bin=$(find_python) || { echo "no python3 with Pillow (set PILLOW_PYTHON)" >&2; exit 1; }
rsvg_bin=$(find_rsvg)
[ "$rsvg_bin" = "nix-shell-rsvg" ] && rsvg_bin="nix shell nixpkgs#librsvg --command rsvg-convert"

# --- args -------------------------------------------------------------------
mode="G2"
targets=()
# Where G2i takes its binaries from. target/debug is what
# `cargo build --bin ...` writes and what render.sh defaults to; a
# --bin-dir points at a set built elsewhere (a nix result, or a copy kept
# while src/ is being rewritten under you).
bin_dir="$crate/target/debug"
if [ $# -gt 0 ] && [ "$1" = "--source" ]; then
  mode="G1"; shift
elif [ $# -gt 0 ] && [ "$1" = "--inventory" ]; then
  mode="G1i"; shift
elif [ $# -gt 0 ] && [ "$1" = "--implementation" ]; then
  mode="G2i"; shift
fi
while [ $# -gt 0 ]; do
  case "$1" in
    --bin-dir) bin_dir=$2; shift 2 ;;
    *) targets+=("$1"); shift ;;
  esac
done

selected_eras=("${eras[@]}")
selected_screens=("${screens[@]}")
{ [ "$mode" = "G1" ] || [ "$mode" = "G1i" ]; } && selected_screens=("${g1i_screens[@]}")
[ "$mode" = "G2i" ] && selected_screens=("${g2i_screens[@]}")
if [ ${#targets[@]} -ge 2 ]; then
  selected_eras=("${targets[0]}")
  selected_screens=("${targets[1]}")
elif [ ${#targets[@]} -eq 1 ]; then
  selected_eras=("${targets[0]}")
fi

if [ "$mode" = "G1i" ] || [ "$mode" = "G2i" ]; then
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
    # One design per screen: the photo-shaped trace where the screen has a
    # Behance source, and docs/<era>/bar.svg for the bar, which has none.
    # The app-shaped dashboard.svg composites were deleted 2026-09-03 --
    # every gate now holds the implementation to what the material shows,
    # which is what the screen is supposed to become.
    svg="$crate/docs/$era/$screen-trace.svg"
    [ -f "$svg" ] || svg="$crate/docs/$era/$screen.svg"
    [ -f "$svg" ] || { echo "SKIP $era/$screen: no $svg"; continue; }

    if [ "$mode" = "G2i" ]; then
      # Both sides are rendered at the screen's own geometry rather than a
      # shared 1600x900. The bar is 220 tall; stretching it 4x to fit one
      # canvas refits every chamfer and diamond as something else -- on
      # both sides at once, so the numbers would still agree while
      # measuring nothing the design language contains.
      w=1600; h=900
      [ "$screen" = "bar" ] && h=220
      # The bar has no binary of its own that weston can host: it maps a
      # layer surface and weston implements no wlr-layer-shell, so the
      # golden matrix renders `bar-window` (same view, ordinary window --
      # tests/bar.nix explains what that covers and what it does not) and
      # so does this.
      case "$screen" in
        bar)     app="cp-eras-ui-bar-window" ;;
        mailbox) app="cp-eras-ui-mailbox" ;;
        mail)    app="cp-eras-ui-mail" ;;
        *)       app="cp-eras-ui-$screen" ;;
      esac
      if [ ! -x "$bin_dir/$app" ]; then
        echo "SKIP G2i $era/$screen: no $bin_dir/$app"
        echo "  build it: nix-shell shell.nix --run 'cargo build --bin $app'"
        continue
      fi

      design="$scratch/g2i-$era-$screen-design.png"
      impl="$scratch/g2i-$era-$screen-impl.png"
      # A trace records how the material *photographs*, and some of that
      # is not design: the sharpening halo around every entropism edge,
      # neokitsch's blurred copy of its own content. The trace draws it
      # (G1i needs it -- the photo has it), but the implementation is told
      # not to, and the extractor bins it as its own ink family, which on
      # the mailbox put 48% (entropism) and 77% (neokitsch) of the design's
      # shape area behind a 60% gate no faithful screen could reach. The
      # trace marks such elements `class="photo"`; G2i renders the design
      # with them hidden, so what is compared is what the screen must draw.
      g2i_svg="$scratch/g2i-$era-$screen-design.svg"
      sed '0,/<svg[^>]*>/s//&<style>.photo{display:none}<\/style>/' "$svg" > "$g2i_svg"
      $rsvg_bin -w "$w" -h "$h" "$g2i_svg" -o "$design" 2>/dev/null || {
        echo "FAIL $era/$screen: rsvg-convert errored"; overall_fail=1; continue; }
      # `env -u FONTCONFIG_FILE`: the pinned fontconfig above exists so
      # rsvg-convert finds Rajdhani/Orbitron the same way everywhere. The
      # app is a different font stack (it embeds its own faces) and the
      # goldens were captured with no FONTCONFIG_FILE at all, so leaving
      # ours set would make this capture answer a different question than
      # the matrix does.
      env -u FONTCONFIG_FILE "$here/render.sh" \
        --era "$era" --size "${w}x${h}" --bin "$bin_dir/$app" --out "$impl" >/dev/null || {
        echo "FAIL $era/$screen: render.sh could not capture $app (see $impl.log)"
        overall_fail=1; continue; }

      echo "--- G2i $era/$screen: $(basename "$svg") vs $app @ ${w}x${h}, as shape inventories ---"
      "$python_bin" "$here/extract_spec.py" "$design" --canvas "${w}x${h}" \
        -o "/tmp/spec-g2i-$era-$screen-design.json" || { overall_fail=1; continue; }
      "$python_bin" "$here/extract_spec.py" "$impl" --canvas "${w}x${h}" \
        -o "/tmp/spec-g2i-$era-$screen-impl.json" || { overall_fail=1; continue; }
      # `shapes` for every era, including the two G1i has to gate on
      # `inks`. That exception is about the *photo*: rotated, translucent
      # geometry fragments one way under a camera's glow and another way
      # in a clean render, so fragment identity is not a stable invariant
      # across that pair. Here both sides are clean renders of our own,
      # and measured 2026-09-02 on the 0.13 binaries the shape inventory
      # is the sharper instrument for all four -- kitsch's bar reads 38%
      # of design shape area matched and neokitsch's 29%, each naming the
      # cells the Rust has not drawn, where the same pairs under `inks`
      # collapse to one number (0.46 and 0.17) that says a screen is
      # wrong without saying where. Nothing here needs `inks`; if a
      # future era does, this is the line that changes.
      #
      # --match-iou 0.65, against spec_diff's own default of 0.30. G1i
      # needs a loose match because a photo's glow spreads a widget's
      # bbox; two clean renders have no such excuse, and 0.30 let
      # entropism's bar PASS while its menu panel sat 140px left of the
      # design's and 67px wider (the dominant shape, matched at IoU
      # 0.50). The headroom was measured, not guessed: the two pairs
      # that have actually converged -- entropism/dashboard at 14/14
      # shapes, centre error 0.0px, and neokitsch/dashboard at 87% of
      # design area, median 0.7px -- keep their score unchanged all the
      # way to 0.90, so 0.65 sits in the middle of a wide empty gap
      # rather than on either side's edge. Raise it only with the same
      # kind of evidence.
      "$python_bin" "$here/spec_diff.py" --gate shapes --match-iou 0.65 \
        "/tmp/spec-g2i-$era-$screen-design.json" \
        "/tmp/spec-g2i-$era-$screen-impl.json" || overall_fail=1
      # A numeric verdict is not a look at the thing. compare_ref's
      # overlays put the design and the capture side by side and in a
      # checkerboard, which is how a human settles what "unmatched in
      # source" actually meant.
      "$python_bin" "$here/compare_ref.py" "$design" "$impl" \
        --out "/tmp/g2i-$era-$screen" >/dev/null 2>&1
      # The two inputs go into the same directory: $scratch is removed on
      # exit, and an overlay you cannot pull the originals out of is half
      # an answer.
      cp "$design" "/tmp/g2i-$era-$screen/design.png" 2>/dev/null
      cp "$impl" "/tmp/g2i-$era-$screen/implementation.png" 2>/dev/null
      cp "$impl.log" "/tmp/g2i-$era-$screen/implementation.log" 2>/dev/null
      echo "  overlays: /tmp/g2i-$era-$screen/ (side-by-side.png, checker.png,"
      echo "            edges.png, heatmap.png, design.png, implementation.png)"
      echo
      continue
    fi

    render="$scratch/$era-$screen.png"
    $rsvg_bin -w 1600 "$svg" -o "$render" 2>/dev/null || { echo "FAIL $era/$screen: rsvg-convert errored"; overall_fail=1; continue; }

    # One Behance screen per era x {login,dashboard,mailbox,store}; ids and
    # what each shows are in docs/sources.md. NB the two entropism files are
    # named the wrong way round: the hub is entropism-store.png and the store
    # is entropism-dashboard.png.
    src=""
    case "$era-$screen" in
      neomil-login)        src="$crate/images/img-06-private.png" ;;
      neomil-dashboard)    src="$crate/images/img-07-dashboard.png" ;;
      neomil-mailbox)      src="$crate/images/img-08-main.png" ;;
      neomil-store)        src="$crate/images/img-09-store.png" ;;
      entropism-login)     src="$crate/images/entropism-login.png" ;;
      entropism-dashboard) src="$crate/images/entropism-store.png" ;;
      entropism-mailbox)   src="$crate/images/entropism-mail.png" ;;
      entropism-store)     src="$crate/images/entropism-dashboard.png" ;;
      kitsch-login)        src="$crate/images/kitsch-login.png" ;;
      kitsch-dashboard)    src="$crate/images/kitsch-dashboard.png" ;;
      kitsch-mailbox)      src="$crate/images/kitsch-mail.png" ;;
      kitsch-store)        src="$crate/images/kitsch-store.png" ;;
      neokitsch-login)     src="$crate/images/neokitsch-login.png" ;;
      neokitsch-dashboard) src="$crate/images/neokitsch-dashboard.png" ;;
      neokitsch-mailbox)   src="$crate/images/neokitsch-mail.png" ;;
      neokitsch-store)     src="$crate/images/neokitsch-store.png" ;;
      *) ;;
    esac

    if [ "$mode" = "G1i" ]; then
      if [ -z "$src" ] || [ ! -f "$src" ]; then
        echo "SKIP G1i $era/$screen: no source downloaded (see docs/sources.md)"
        continue
      fi
      g1svg="$svg"
      render="$scratch/$era-$screen.png"
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
      golden="$crate/tests/golden/$screen-$era-1600x900.png"
      [ "$screen" = "bar" ] && golden="$crate/tests/golden/$screen-$era-1600x220.png"
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
exit $overall_fail