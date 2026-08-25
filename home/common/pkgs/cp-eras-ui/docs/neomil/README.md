# Design targets

Hand-drawn SVG mockups of the full toolkit, in the strict reference
palette (sampled from the Neo-Militarism Behance images — see
`src/colors.rs` and TODO.md's palette-correction entry):

- `target-components.svg` — every widget the toolkit should grow, with
  states: buttons (primary/ghost/override/disabled/icon), text input
  (+focus), select (+open list), slider, toggles/checkbox/radio, tab
  bar, meters (segmented/bar/indeterminate), badges/tags/status dots,
  toast+banner (warn/error), modal, table with selection + scrollbar,
  key-value rows, log view, status bar, context menu, tooltip, 16px
  icon set.
- `target-app.svg` — "NEOMIL OPS", a realistic app composed purely
  from those components (services table, meters, live log, detail
  panel, action buttons, nav rail, status bar). The acceptance test
  for the toolkit: when this screen can be built from library
  widgets, the toolkit is feature-complete.

Render with Rajdhani + Orbitron available to fontconfig:

    FONTCONFIG_FILE=<conf with the fonts> rsvg-convert -w 1600 -h 900 target-components.svg -o /tmp/sheet.png
