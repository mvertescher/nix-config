# Preset palettes for the kitsch theme.
#
# Kitsch is "style over substance": teal line-work and yellow selection
# over a rose bloom on warm black. Everything rounded, no chamfers.
#
# The `reference` palette is transcribed from the pixel reads recorded
# in home/common/pkgs/cp-eras-ui/docs/kitsch/README.md, taken off the
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
#
# Each variant also declares the optional roles from ../lib/roles.nix
# that the kitsch references actually use: the shelf band and its ink,
# the mint stat bar and its ink, the two edges of an extruded fan slab,
# and the page-curl. `inset` is deliberately absent -- kitsch cards are
# unfilled outlines, there is no well anywhere in the era -- which is
# the point of the roles being individually optional.
{
  # Sampled. `panel` is derived rather than measured: the reference has
  # no bar, and its bloom is a background wash -- painting a status bar
  # in full #a63355 would be a misreading of where that colour lives.
  # This is the bloom's dark tail, which is what the wash actually looks
  # like away from its core.
  reference = {
    bg = "#0b0b07";
    panel = "#1c0f16";
    # The outline teal the store and mailbox traces sample off card
    # frames and dividers -- a stop under `fg`, not the dim #2e5f57 this
    # once was, which dropped the frames out of the teal ink family.
    border = "#5fd6c2";
    dim = "#4d9484";
    fg = "#7ddec8"; # sampled teal, carries all structure
    alert = "#fcc428"; # sampled yellow -- selection, not alarm
    tape = "#f08c1e"; # sampled bezel orange

    # Sampled alongside the seven, off the same modules. The mint pair
    # is the one the crate already compiles in as its era constant, so
    # publishing it here is a restatement, not a restyle.
    banner = "#fcc428"; # shelf band / flag tag fill
    onBanner = "#37220f"; # the glyphs and brand tag on it
    emphasis = "#87f4d9"; # mint stat bar
    onEmphasis = "#0b3b31"; # figures on the mint
    bevel = "#2bc4ac"; # lit face of a fan-menu slab
    shade = "#177a6b"; # its extrusion, receding up-right
    ornament = "#1cb39b"; # solid teal: page-curl, chips
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

    # The band stays hot -- it is what "selected" looks like -- but its
    # ink darkens with the rest of the palette.
    #
    # No `emphasis` here, deliberately. The crate's card already reads
    # its stat band from that role, so publishing a retinted mint would
    # restyle a shipped widget rather than just widen the vocabulary --
    # and this variant's mint genuinely does need rethinking for paper.
    # `reference` can declare it because its value is the sampled one
    # the crate already compiles in, so it changes nothing. Retinting
    # bleach and ash belongs with whoever looks at the result.
    banner = "#e8a80f";
    onBanner = "#2a1c06";
    bevel = "#2aa891";
    shade = "#0e6152";
    ornament = "#177a6b";
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

    # Same rule as the seven: the yellow band survives because it is
    # meaning, the decoration goes grey-teal because it is not.
    # `emphasis` left undeclared for the reason given under `bleach`.
    banner = "#fcc428";
    onBanner = "#37220f";
    bevel = "#9fb5ac";
    shade = "#3f4a45";
    ornament = "#6d9a8c";
  };
}
