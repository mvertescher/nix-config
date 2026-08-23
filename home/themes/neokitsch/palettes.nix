# Preset palettes for the neokitsch theme.
#
# Neokitsch is "substance and style": gold line-work on true black under
# a violet haze. Kitsch's later, quieter descendant -- no page-curl, no
# shelf bands, far fewer captions.
#
# The `reference` palette is transcribed from the pixel reads recorded
# in home/common/pkgs/cyberpunk-ui/docs/neokitsch/README.md, taken off
# the 1400px Behance modules rather than eyeballed.
#
# This is the era the word "kitsch" makes people picture: gilded, and
# with wood veneer filling every selected element. See ../kitsch for the
# other half of that mix-up.
#
# The veneer itself is not a role. It is a *material* -- the toolkit
# synthesises it (cyberpunk-ui `Selection::Veneer`) rather than naming a
# colour -- so `tape` carries its mid-tone for the desktop's sake and
# the app does the rest.
{
  # Sampled. `panel` is derived, not measured, for the same reason as
  # kitsch's: the violet haze is a background wash and painting a status
  # bar in full #34344c would misread where that colour lives.
  reference = {
    bg = "#0a0a0a";
    panel = "#16161f";
    border = "#916424"; # sampled frame gold
    dim = "#8a7048";
    fg = "#e7c686"; # sampled gold text
    alert = "#fcc474"; # sampled amber -- the only strong CTA colour
    tape = "#e3af5f"; # veneer mid-tone
  };

  # Light mode: the catalogue on paper. Warm off-white, the gold
  # darkened to a legible bronze, amber kept for escalation.
  bleach = {
    bg = "#f4f1ea";
    panel = "#e8e3d7";
    border = "#b09a6c";
    dim = "#7a6844";
    fg = "#2a2318";
    alert = "#9c6b12";
    tape = "#8a6a2c";
  };

  # Dark and neutral: greys structural, gold reserved for what matters.
  ash = {
    bg = "#0c0c0d";
    panel = "#151517";
    border = "#2e2c28";
    dim = "#6e6a60";
    fg = "#cfcabd";
    alert = "#fcc474";
    tape = "#e3af5f";
  };
}
