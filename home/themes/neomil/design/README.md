# Neomil desktop design

The design for the Neomil Hyprland theme, drawn before it is built. Open
`index.html` in a browser; `tiled.html`, `empty.html`, `launcher.html`
and `lock.html` are the single scenes. Every screen we implement gets a
scene here first, and the scene is changed before the implementation is.

- `neomil-design.js` draws each scene as SVG. `INK` is the palette,
  `TARGET` is everything that differs between the desktop and the TV
  fitting, and each component (`badge`, `bar`, `plate`, `echo`, `win`) is
  a function, so a new scene is assembled from the same parts.
- `neomil-design.css` holds the page chrome and the motion studies. The
  era is hard-edged: motion is stepped (`steps()`), never eased.

Units are the traces' design px (1600x900, 2.4 native px per unit at
3840x2160). Line weights, inks and plate treatments come from
`home/common/pkgs/cp-eras-ui/docs/neomil/line-weights.md`. Nothing is
traced from reference pixels; fonts load from Google Fonts for the
mock-up only (the theme itself takes them from nixpkgs).

## Open choices (Matt, 2026-11-14: "show me both")

- Top bar: the reference header (62 units) or a slim bar (30 units).
- Windows: dark with red lines, or a red title plate on the focused one.

## What stock Hyprland cannot draw

A title plate needs a title-bar plugin or our own; the printed echo
around a window may be possible as a hard offset shadow (untried);
chamfered window corners are not attempted. The launcher, notifications
and lock are drawn by our own programs, so they are not limited this way.

## Not yet designed

Application interiors (terminal, browser and editor themes are
stand-ins), the bar's real status cells, menus, the TV's selection
behaviour under a remote, and durations for each animation.
