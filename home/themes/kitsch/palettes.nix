# Preset palettes for the kitsch theme.
#
# Kitsch is "style over substance": teal line-work and yellow selection
# over a rose bloom on warm black. Everything rounded, no chamfers.
#
# The `reference` palette is transcribed from the pixel reads recorded
# in home/common/pkgs/cyberpunk-ui/docs/kitsch/README.md, taken off the
# 1400px Behance modules rather than eyeballed.
#
# Worth knowing before editing: the era's name invites the wrong guess.
# An earlier pass here assumed "maximalist" meant gold, damask and
# filigree, and got the era backwards -- that description belongs to
# neokitsch, whose selected cards are filled with wood veneer. Kitsch is
# not ornamental at all; it is rounded silhouettes and flat fills.
#
# Note the role inversion the era forces: yellow is *selection*, not
# alarm. Failure states are essentially absent from the references, so
# `alert` and `tape` carry the two accents and nothing here means
# "error" in the way the other eras' `alert` does.
{
  # Sampled. `panel` is derived rather than measured: the reference has
  # no bar, and its bloom is a background wash -- painting a status bar
  # in full #a63355 would be a misreading of where that colour lives.
  # This is the bloom's dark tail, which is what the wash actually looks
  # like away from its core.
  reference = {
    bg = "#0b0b07";
    panel = "#1c0f16";
    border = "#2e5f57";
    dim = "#4d9484";
    fg = "#7ddec8"; # sampled teal, carries all structure
    alert = "#fcc428"; # sampled yellow -- selection, not alarm
    tape = "#f08c1e"; # sampled bezel orange
  };

  # Light mode: the catalogue printed rather than lit. Warm paper, the
  # teal darkened until it is legible as ink, and the yellow kept hot so
  # selection still reads as selection.
  bleach = {
    bg = "#f2efe6";
    panel = "#e5e0d2";
    border = "#b9b2a0";
    dim = "#5e7d72";
    fg = "#123c33";
    alert = "#b8860b";
    tape = "#c2621a";
  };

  # Dark and neutral: for when the rose field is too much. Greys take
  # the structural roles, teal and yellow reserved for what matters.
  ash = {
    bg = "#0c0d0c";
    panel = "#151716";
    border = "#2b2f2d";
    dim = "#6d7a73";
    fg = "#c6d4ce";
    alert = "#fcc428";
    tape = "#f08c1e";
  };
}
