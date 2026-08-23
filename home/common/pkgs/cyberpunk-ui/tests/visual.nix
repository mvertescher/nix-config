# Visual regression for the toolkit.
#
# Renders an example on a headless compositor inside the build sandbox
# and compares the result against a committed golden image. Two
# independent sandbox runs were verified to produce byte-identical
# captures, so this can hold a strict threshold rather than a fuzzy one.
#
# Two things had to be true for this to work at all, both established by
# probing rather than assumed:
#
#   * weston runs in a nix build sandbox with the headless backend and
#     the pixman renderer, given XDG_RUNTIME_DIR under $TMPDIR at mode
#     700 and HOME set. No seat, no /dev/dri.
#   * iced needs WGPU_BACKEND=vulkan *and* a software Vulkan ICD. The
#     ICD alone is not enough: wgpu otherwise selects GLES and panics in
#     wgpu-hal's gles/egl.rs on a missing EGL display.
#
# The golden is our own render, not the Behance reference art -- that
# lives in the gitignored images/ directory, so it cannot be in a
# hermetic build, and vendoring someone else's artwork to diff against
# would be the wrong answer anyway.
{
  lib,
  runCommand,
  weston,
  mesa,
  python3,
  cyberpunk-ui,
  example ? "cyberpunk-ui-dashboard",
  width ? 1280,
  height ? 800,
  golden ? ../tests/golden/dashboard-1280x800.png,
  # Captures are byte-identical run to run, so anything below this is a
  # real change rather than noise.
  threshold ? "99.9",
  # Seconds to let the app draw before capturing.
  settle ? 15,
}:

let
  python = python3.withPackages (ps: [ ps.pillow ]);
in
runCommand "cyberpunk-ui-visual-test"
  {
    nativeBuildInputs = [
      weston
      mesa
      cyberpunk-ui
      python
    ];
    meta = {
      description = "Headless render of ${example} diffed against a golden image";
    };
  }
  ''
    export XDG_RUNTIME_DIR="$TMPDIR/xdg"
    mkdir -p "$XDG_RUNTIME_DIR"
    chmod 700 "$XDG_RUNTIME_DIR"
    export HOME="$TMPDIR/home"
    mkdir -p "$HOME"
    export WAYLAND_DISPLAY=neomil-test

    icd=$(find ${mesa} -name 'lvp_icd*.json' | head -1)
    if [ -z "$icd" ]; then
      echo "no software Vulkan ICD found in ${mesa}; cannot render headless" >&2
      exit 1
    fi
    export VK_ICD_FILENAMES="$icd"
    # Without this wgpu picks GLES and panics on a missing EGL display.
    export WGPU_BACKEND=vulkan

    cd "$TMPDIR"

    # --debug is what permits weston-screenshooter to attach.
    weston --backend=headless --renderer=pixman --shell=kiosk \
      --no-config --debug --socket="$WAYLAND_DISPLAY" \
      --width=${toString width} --height=${toString height} -- \
      ${example} > "$TMPDIR/app.log" 2>&1 &
    westonPid=$!

    for _ in $(seq 1 40); do
      [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ] && break
      sleep 0.5
    done

    if [ ! -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; then
      echo "compositor never created its socket" >&2
      cat "$TMPDIR/app.log" >&2
      exit 1
    fi

    sleep ${toString settle}
    weston-screenshooter || true
    sleep 2
    kill "$westonPid" 2>/dev/null || true

    shot=$(ls -t "$TMPDIR"/wayland-screenshot*.png "$HOME"/wayland-screenshot*.png 2>/dev/null | head -1)
    if [ -z "$shot" ]; then
      echo "no screenshot was produced" >&2
      cat "$TMPDIR/app.log" >&2
      exit 1
    fi

    mkdir -p "$out"
    cp "$shot" "$out/render.png"
    cp "$TMPDIR/app.log" "$out/app.log"

    # The diff is written whether or not the check passes, so a failing
    # build leaves something to look at.
    python3 ${../scripts/check_similarity.py} \
      ${golden} "$out/render.png" ${threshold} "$out/diff.png"
  ''
