#!/usr/bin/env bash
# render.sh — headless capture of one example, outside the nix sandbox.
#
# `tests/visual.nix` is the authority on how to do this: weston with the
# headless backend and the pixman renderer, a mode-700 XDG_RUNTIME_DIR, a
# software Vulkan ICD plus WGPU_BACKEND=vulkan, and a theme published into
# a scratch HOME. That harness is a nix build, which means it rebuilds the
# crate and takes minutes; this script is the same recipe pointed at a
# binary you already have, so a design iteration costs ~20s instead.
#
# What it is *not* is a replacement for the golden matrix. The matrix is
# hermetic and its captures are byte-identical run to run; this runs on a
# developer's box against whatever binary is on disk. Use it to look at a
# change, and `scripts/run_test_matrix.sh` to gate one.
#
# Usage:
#   render.sh [options] <binary-name>
#
#   --era NAME      publish home/themes/NAME into the scratch HOME, in the
#                   format home/themes/lib/era.nix writes. `--era none`
#                   publishes nothing, so the crate's compiled fallback
#                   renders (what tests/visual.nix calls the fallback case).
#   --size WxH      output size; default 1600x900. The bar wants 1600x220.
#   --out PATH      where to write the capture; default /tmp/<binary>.png.
#                   The app log always lands next to it as <out>.log.
#   --bin PATH      use this binary instead of resolving one by name.
#   --settle N      seconds to let the app draw before capturing. Default
#                   4; the note on DEFAULT_SETTLE below has the measurements
#                   (and why ICED_PRESENT_MODE is set) when tests/visual.nix
#                   waits 15.
#   --at SECONDS    freeze the app's clock at this moment (`motion::now`,
#                   via CP_ERAS_UI_AT_MS) so the capture is that frame of
#                   the trace's motion. Default 0: frame 0, the static
#                   design, which is what the goldens hold and what a
#                   capture with no `--at` has always meant. Fractions
#                   are fine; frame.sh takes the same number.
#   --keep-log      accepted and ignored: the log is always kept.
#
# Examples:
#   render.sh --era neomil --size 1600x220 --out /tmp/bar.png cp-eras-ui-bar-window
#   render.sh --era kitsch --out /tmp/login.png cp-eras-ui-login
#   render.sh --bin /tmp/cp-eras-ui-bins-0.13/cp-eras-ui-login --era neomil ...
#
# This script never builds the crate. Without --bin it looks for
# target/debug/<name>, which you get from
#
#   nix-shell shell.nix --run 'cargo build --bin cp-eras-ui-login'
#
# Everything else it needs (weston, weston-screenshooter, mesa's lavapipe
# ICD, and the runtime libraries an unwrapped cargo binary dlopens) it
# finds on PATH or nix-builds on demand; see `render_env` below.
set -u

here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
crate=$(dirname "$here")
home_root=$(cd -- "$crate/../../.." && pwd)   # home/, where themes/ lives

# 4s, with ICED_PRESENT_MODE=mailbox exported below. History: 3 until
# 2026-09-04, measured 2026-09-02 against the 0.13 binaries (login/neomil,
# bar-window/neokitsch, dashboard/kitsch and store/entropism all
# byte-identical to visual.nix's 15s capture at 0-8s). Then the login
# conversion added its software-rendered washes (`screens/login.rs`
# `wash_image`) and a 3s capture of *every* era's login lacked the wash
# while 5s, 8s and 15s were byte-identical to the goldens; G2i had been
# reporting entropism login at 28% FAIL for that reason alone, and the
# default went to 8. The delay was then blamed on the wash's raster cost
# and filed as a first-paint problem. It is not: instrumented, the app
# draws exactly two frames -- at ~60ms and ~130ms after weston starts,
# the wash rasterised in the first in ~19ms, release and debug alike --
# and never draws again. What took 3-5s was the *second frame reaching
# the compositor*: under wgpu's default FIFO presentation, lavapipe's
# swapchain blocks in present() until headless weston (pixman, no vblank)
# hands back a frame callback, which it does seconds late for an idle
# surface. What a 1-3.5s capture shows is the first frame, which comes
# out without the wash for a reason not chased (iced_wgpu uploads an
# image in the frame that draws it, so it is at most a one-frame gap; a
# real compositor turns it into a one-vblank flash). Under mailbox (or
# immediate) presentation the same binary's 2s capture is byte-identical
# to the goldens on 19 of 20 cells -- the 20th, store/neomil, differs by
# its kanji glyphs, which the sandbox has no CJK font for and this host
# does (crate TODO.md); no cell moved between 2s and 8s under either
# mode. 4 keeps the 2x headroom 3 used to have over the measured value;
# the sandbox keeps 15 and FIFO because three wasted minutes across the
# whole matrix is cheaper than one flaky build.
DEFAULT_SETTLE=4

usage() { sed -n '2,/^set -u/p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//;$d'; }

era=""
size="1600x900"
out=""
bin=""
settle=$DEFAULT_SETTLE
name=""
at=0

while [ $# -gt 0 ]; do
  case "$1" in
    --era)    era=$2; shift 2 ;;
    --size)   size=$2; shift 2 ;;
    --out)    out=$2; shift 2 ;;
    --bin)    bin=$2; shift 2 ;;
    --settle) settle=$2; shift 2 ;;
    --at)     at=$2; shift 2 ;;
    --keep-log) shift ;;
    -h|--help) usage; exit 0 ;;
    -*) echo "render.sh: unknown option $1" >&2; usage >&2; exit 2 ;;
    *)  name=$1; shift ;;
  esac
done

width=${size%x*}
height=${size#*x}
if [ "$width" = "$size" ] || [ -z "$height" ]; then
  echo "render.sh: --size wants WxH, got '$size'" >&2; exit 2
fi

# --- resolve the binary -----------------------------------------------------
# --bin wins outright, including over a stale target/debug: a caller who
# names a path has usually just built it somewhere else on purpose.
if [ -n "$bin" ]; then
  [ -x "$bin" ] || { echo "render.sh: $bin is not executable" >&2; exit 2; }
  # Absolute, because weston is started from a scratch cwd below and
  # execs the path as given: a relative one fails with weston's
  # unhelpful "Failed to execute command line command".
  bin=$(realpath "$bin")
  [ -n "$name" ] || name=$(basename "$bin")
else
  [ -n "$name" ] || { echo "render.sh: need a binary name or --bin PATH" >&2; usage >&2; exit 2; }
  bin="$crate/target/debug/$name"
  if [ ! -x "$bin" ]; then
    echo "render.sh: no $bin" >&2
    echo "  build it first:  nix-shell shell.nix --run 'cargo build --bin $name'" >&2
    echo "  or point at one: render.sh --bin /path/to/$name ..." >&2
    exit 2
  fi
fi

[ -n "$out" ] || out="/tmp/$name.png"

# --- tools ------------------------------------------------------------------
# Three ways to get weston + the ICD were tried on a warm store:
#
#   nix shell nixpkgs#weston nixpkgs#mesa   1.8s, but only sets PATH. A
#     binary from `cargo build` is unwrapped, and the crate's real
#     wrapper (default.nix postFixup) sets LD_LIBRARY_PATH — libvulkan,
#     libxkbcommon and wayland are dlopened, not in DT_NEEDED, so without
#     that the app dies on a missing libvulkan.so.1 and the capture is
#     the compositor's empty output. Fine in the sandbox, where the
#     example is the wrapped one; not fine here.
#   nix-shell shell.nix                     1.6s, and does set
#     LD_LIBRARY_PATH — but its shellHook rewrites the crate's fonts/
#     directory as a side effect of asking for a compositor.
#   a buildEnv of both halves                1.6s cold, ~0.4s warm, no
#     side effects, and gives PATH and LD_LIBRARY_PATH from one path.
#
# So: buildEnv. The `2>/dev/null` swallows nixpkgs' xorg.* deprecation
# warnings, which are not this script's business.
render_env() {
  nix build --impure --no-link --print-out-paths --expr \
    'with import <nixpkgs> {}; buildEnv {
       name = "cp-eras-ui-render-env";
       paths = [ weston mesa vulkan-loader libglvnd libxkbcommon
                 libpulseaudio wayland
                 xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi ];
     }' 2>/dev/null | tail -1
}

mesa_path() {
  nix build --no-link --print-out-paths nixpkgs#mesa 2>/dev/null | tail -1
}

app_ld_path=""
if command -v weston >/dev/null && command -v weston-screenshooter >/dev/null; then
  : # a devshell already put them here; trust it for the libraries too
else
  env_path=$(render_env)
  [ -n "$env_path" ] || { echo "render.sh: could not nix-build a weston/mesa env" >&2; exit 1; }
  PATH="$env_path/bin:$PATH"
  export PATH
  app_ld_path="$env_path/lib"
fi

# The ICD is looked up under mesa's own store path, not the env's, because
# the json inside it names an absolute .so and a buildEnv symlink would
# still resolve to the same file — but mesa is also what visual.nix
# searches, so keep the two lookups identical.
mesa_out=$(mesa_path)
icd=""
[ -n "$mesa_out" ] && icd=$(find "$mesa_out/share/vulkan/icd.d" -name 'lvp_icd*.json' 2>/dev/null | head -1)
if [ -z "$icd" ] && [ -n "${env_path:-}" ]; then
  icd=$(find "$env_path/share/vulkan/icd.d" -name 'lvp_icd*.json' 2>/dev/null | head -1)
fi
if [ -z "$icd" ]; then
  echo "render.sh: no software Vulkan ICD (lvp_icd*.json) found; cannot render headless" >&2
  exit 1
fi

# --- theme ------------------------------------------------------------------
# The [colors] block comes from the era's own scheme.nix, so this renders
# the palette the desktop would publish rather than a copy of it — the
# same reasoning default.nix gives for `eraCase`. The eval is a second or
# two, which matters in a loop, so the result is cached keyed by era *and*
# by the content of the two files that decide it: edit a palette and the
# cache misses, which is the only invalidation rule that cannot go stale.
theme_cache="${TMPDIR:-/tmp}/cp-eras-ui-render-themes"
theme_toml=""

if [ -n "$era" ] && [ "$era" != "none" ]; then
  scheme="$home_root/themes/$era/scheme.nix"
  [ -f "$scheme" ] || { echo "render.sh: no theme '$era' ($scheme)" >&2; exit 2; }
  # Entropism's sampled palette is `nexus`; every other era calls its
  # own `reference`. Same rule as default.nix's `variantOf`.
  variant="reference"
  [ "$era" = "entropism" ] && variant="nexus"

  mkdir -p "$theme_cache"
  # The scheme imports its palettes.nix, so that has to be in the key too:
  # without it, retinting a role left this cache serving the old value.
  key=$(cat "$scheme" "$home_root/themes/$era/palettes.nix" "$home_root/themes/lib/roles.nix" | cksum | tr -d ' /')
  theme_toml="$theme_cache/$era-$key.toml"

  if [ ! -s "$theme_toml" ]; then
    # `r.names ++ r.extrasOf s` is the same tail lib/era.nix writes: the
    # base seven plus whichever ornamentals this era's resolved palette
    # actually declares. Run from home/ because the expression's paths
    # are relative.
    body=$(cd "$home_root" && nix eval --impure --raw --expr "
      let r = import ./themes/lib/roles.nix;
          s = (import ./themes/$era/scheme.nix).resolve { variant = \"$variant\"; };
      in r.polarityOf s + \"\n\" +
         builtins.concatStringsSep \"\n\" (map (n: \"\${n} = \\\"\${s.\${n}}\\\"\") (r.names ++ r.extrasOf s))
    " 2>&1) || { echo "render.sh: theme eval failed for '$era':" >&2; echo "$body" >&2; exit 1; }
    polarity=${body%%$'\n'*}
    colors=${body#*$'\n'}
    # Exactly the shape tests/visual.nix's themeToml produces. Restated
    # here for the same reason it restates it: if that format moves, this
    # should stop matching rather than silently follow.
    {
      echo "era = \"$era\""
      echo "variant = \"$variant\""
      echo "polarity = \"$polarity\""
      echo
      echo "[font]"
      echo 'ui = "Rajdhani"'
      echo
      echo "[colors]"
      echo "$colors"
    } > "$theme_toml.$$"
    mv "$theme_toml.$$" "$theme_toml"
  fi
fi

# --- scratch ----------------------------------------------------------------
run=$(mktemp -d "${TMPDIR:-/tmp}/cpui-render.XXXXXX")
trap 'rm -rf "$run"' EXIT

export XDG_RUNTIME_DIR="$run/xdg"
mkdir -p "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"        # weston refuses a world-readable one
export HOME="$run/home"
mkdir -p "$HOME"

# The toolkit resolves its palette from XDG_CONFIG_HOME before
# HOME/.config, so leaving it set renders the "reference" screen in
# whatever era the developer happens to be sitting in. tests/visual.nix
# unsets it in a sandbox that never had it; here it is load-bearing.
unset XDG_CONFIG_HOME

if [ -n "$theme_toml" ]; then
  mkdir -p "$HOME/.config/theme"
  cp "$theme_toml" "$HOME/.config/theme/current.toml"
fi

# Unique per run, so two renders can be in flight at once. The socket
# lives under this run's XDG_RUNTIME_DIR anyway, but weston also uses the
# name for its lock file.
export WAYLAND_DISPLAY="cpui-render-$$"
export VK_ICD_FILENAMES="$icd"
# Without this wgpu picks GLES and panics in wgpu-hal's gles/egl.rs on a
# missing EGL display. The ICD alone is not enough.
export WGPU_BACKEND=vulkan
# Mailbox presentation, so present() never blocks on a frame callback the
# headless compositor is slow to send; the DEFAULT_SETTLE note has the
# measurement. Pixel content is unaffected -- it is when the frame lands.
export ICED_PRESENT_MODE=mailbox
# The frame of the motion to capture, in ms (`--at`, seconds). Always set:
# an app left on its own clock blinks its caret against the settle time.
CP_ERAS_UI_AT_MS=$(perl -e "printf q{%d}, $at * 1000")
export CP_ERAS_UI_AT_MS
[ -n "$app_ld_path" ] && export LD_LIBRARY_PATH="$app_ld_path${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

log="$run/app.log"
cd "$run"

# --debug is what permits weston-screenshooter to attach.
weston --backend=headless --renderer=pixman --shell=kiosk \
  --no-config --debug --socket="$WAYLAND_DISPLAY" \
  --width="$width" --height="$height" -- \
  "$bin" > "$log" 2>&1 &
weston_pid=$!

for _ in $(seq 1 40); do
  [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ] && break
  sleep 0.5
done

finish() {
  # The log is the only way to tell a black capture (app crashed, wrong
  # backend) from a real one, so it is copied whatever happens.
  mkdir -p "$(dirname "$out")"
  cp "$log" "$out.log" 2>/dev/null || true
}

if [ ! -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; then
  finish
  echo "render.sh: compositor never created its socket" >&2
  cat "$log" >&2
  kill "$weston_pid" 2>/dev/null
  exit 1
fi

sleep "$settle"
weston-screenshooter || true
sleep 2
kill "$weston_pid" 2>/dev/null

shot=$(ls -t "$run"/wayland-screenshot*.png "$HOME"/wayland-screenshot*.png 2>/dev/null | head -1)
finish
if [ -z "$shot" ]; then
  echo "render.sh: no screenshot was produced (log copied to $out.log)" >&2
  cat "$log" >&2
  exit 1
fi

cp "$shot" "$out"
echo "$out"
