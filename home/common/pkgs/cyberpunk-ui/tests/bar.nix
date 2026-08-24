# Visual regression for the status bar.
#
# The bar cannot go through `visual.nix` the way the screens do. That
# harness runs the example under weston's headless backend, and weston
# implements no `wlr-layer-shell` -- so `cyberpunk-ui-bar` would come up
# with nowhere to put its surface. Nor is a layer surface something
# `weston-screenshooter` would capture if it could: the shooter asks the
# compositor for an output, and the bar is a shell surface hanging off
# one.
#
# So this case renders `cyberpunk-ui-bar-window` instead: the same
# `bar()` view, the same `Style`, a fixed `Readings`, in an ordinary
# window. What that buys and what it does not:
#
#   * Covered: every era's palette and geometry as the bar uses them --
#     cell corners, the host tape, the alert ink on a muted sink or a
#     tray item asking for attention, module widths, and the contract
#     with the published theme in `~/.config/theme/current.toml`.
#   * Not covered: that the binary maps a layer surface, that it
#     reserves an exclusive zone, or that any sensor thread produces a
#     reading. Those are properties of `cyberpunk-ui-bar`, and only a
#     running compositor can speak to them.
#
# The two halves are held together by `examples/bar/style.rs`, which
# both binaries use to resolve their era. If they resolved it separately
# the goldens would be evidence about a style the live bar never wears.
#
# 220px tall rather than the bar's own 26: `weston --width/--height`
# sets the output, the kiosk shell fullscreens the window into it, and a
# 26px strip is unreadable as a screenshot. The extra ground under the
# bar also puts the era's background role into the diff.
#
# To regenerate the goldens after a deliberate change: add
# `threshold = "0";` below, build the four `tests.bar.<era>` cases, copy
# each `$out/render.png` over `golden/bar-<era>-1600x220.png`, and take
# the line out again. There is no lower-friction path on purpose -- a
# golden that is easy to overwrite is a golden nobody reads.
{
  lib,
  runCommand,
  weston,
  mesa,
  python3,
  cyberpunk-ui,
  era,
  variant,
  roles,
}:

import ./visual.nix {
  inherit
    lib
    runCommand
    weston
    mesa
    python3
    cyberpunk-ui
    era
    variant
    roles
    ;
  example = "cyberpunk-ui-bar-window";
  width = 1600;
  height = 220;
  golden = ./golden/bar-${era}-1600x220.png;
}
