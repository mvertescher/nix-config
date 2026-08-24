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
# With `era` set the case also publishes a theme into the sandbox HOME,
# so the render exercises the contract between the nix theme layer and
# the toolkit rather than the crate's compiled fallback. That is the
# interesting half: the fallback can only drift from itself, whereas the
# published palette has two sides that can move apart.
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
  # The matrix geometry. The defaults are the no-theme fallback case,
  # which used to be a 1280x800 render of a neo-militarism-only
  # dashboard; that screen no longer exists -- `panels::dashboard` is
  # written against `Style` like the rest -- so the case moved onto the
  # matrix's geometry rather than keeping a size nothing else uses.
  width ? 1600,
  height ? 900,
  golden ? ../tests/golden/dashboard-fallback-1600x900.png,
  # Captures are byte-identical run to run, so anything below this is a
  # real change rather than noise.
  threshold ? "99.9",
  # Seconds to let the app draw before capturing.
  settle ? 15,
  # Publish a theme into the sandbox HOME. `era` is the name the toolkit
  # matches on, `roles` the seven-role attrset -- normally taken straight
  # from home/themes/<era>/scheme.nix, so this fails if either side of
  # that contract moves. Leave `era` null to exercise the fallback.
  era ? null,
  variant ? "reference",
  roles ? null,
  uiFont ? "Rajdhani",
}:

let
  python = python3.withPackages (ps: [ ps.pillow ]);

  roleNames = (import ../../../../themes/lib/roles.nix).names;

  # The same shape lib/era.nix writes. Deliberately restated rather than
  # imported: this test's job includes noticing if that format changes,
  # and sharing the generator would hide exactly that.
  themeToml = ''
    era = "${era}"
    variant = "${variant}"
    polarity = "${(import ../../../../themes/lib/roles.nix).polarityOf roles}"

    [font]
    ui = "${uiFont}"

    [colors]
    ${lib.concatStringsSep "\n" (map (r: ''${r} = "${roles.${r}}"'') roleNames)}
  '';

  # The screen belongs in the name as much as the era does: with only
  # the era, store/kitsch and mailbox/kitsch both build as
  # "cyberpunk-ui-visual-test-kitsch", and there is no telling their
  # store paths or `nix log` output apart. The dashboard used to be
  # exempt because it was the only screen; it is not any more, and
  # leaving it unnamed made `dashboard/neomil` and a bare `neomil`
  # indistinguishable in exactly the same way.
  suffix =
    "-${lib.removePrefix "cyberpunk-ui-" example}"
    + (if era == null then "-fallback" else "-${era}");
in
runCommand "cyberpunk-ui-visual-test${suffix}"
  {
    nativeBuildInputs = [
      weston
      mesa
      cyberpunk-ui
      python
    ];
    meta = {
      description = "Headless render of ${example}${
        lib.optionalString (era != null) " in ${era}"
      } diffed against a golden image";
    };
  }
  ''
    export XDG_RUNTIME_DIR="$TMPDIR/xdg"
    mkdir -p "$XDG_RUNTIME_DIR"
    chmod 700 "$XDG_RUNTIME_DIR"
    export HOME="$TMPDIR/home"
    mkdir -p "$HOME"

    # The toolkit resolves its palette from XDG_CONFIG_HOME before
    # HOME/.config. A build sandbox leaves it unset, but say so anyway:
    # driving this harness by hand outside the sandbox otherwise picks up
    # the developer's live desktop and renders the "reference" screen in
    # whatever era they happen to be sitting in. That is not
    # hypothetical -- it produced a light-mode neomil capture during the
    # four-era bring-up.
    unset XDG_CONFIG_HOME

    ${lib.optionalString (era != null) ''
      mkdir -p "$HOME/.config/theme"
      cat > "$HOME/.config/theme/current.toml" <<'THEME_EOF'
      ${themeToml}
      THEME_EOF
    ''}

    export WAYLAND_DISPLAY=cpui-test

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
