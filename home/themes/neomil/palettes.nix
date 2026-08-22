# Preset palettes for the neomil theme.
#
# Neo-Militarism is the default player UI era of Cyberpunk 2077: hard
# edges, military-industrial, hierarchy carried by red brightness rather
# than by hue.
#
# The `reference` palette is transcribed from
# home/common/pkgs/neomil-ui/src/colors.rs, which was sampled from the
# Behance reference images (img-07-dashboard et al., 2026-08-21) rather
# than eyeballed. Three reds on near-black, a cold blue ambient wash,
# and off-white reserved for rare secondary text. There is deliberately
# no yellow: the references contain none.
#
# Only three reds were sampled, so `dim` is interpolated between the
# deep and fill reds -- the references do hierarchy by brightness, so an
# intermediate step is in the spirit of the system even though no pixel
# was measured for it.
#
# This is a different thing from the `cybr` theme, which is also
# Neomilitarism-flavoured: cybr is the cybrcore community look with its
# own 33-colour palette, neomil is the reference-sampled one.
{
  # Sampled. Panel is COLOR_GLOW, the cold blue wash the references put
  # behind the top of every screen -- which is exactly where a bar sits.
  reference = {
    bg = "#050304";
    panel = "#001a33";
    border = "#5e1112"; # COLOR_RED_DEEP, sampled from reference borders
    dim = "#a32226"; # interpolated between deep and fill
    fg = "#de2e2e"; # COLOR_RED_FILL, sampled from the reference diamonds
    alert = "#ff3b45"; # COLOR_PRIMARY_RED, the hot end of the ramp
    tape = "#dedede"; # COLOR_OFF_WHITE, sparing labels and values only
  };

  # Light mode: the same system printed rather than lit. Greys and paper
  # white for surfaces, near-black for body text so it stays legible,
  # and the sampled reds kept for escalation and labels so the era is
  # still recognisable.
  bleach = {
    bg = "#e9e7e3";
    panel = "#dcd9d3";
    border = "#b3ada4";
    dim = "#6e6960";
    fg = "#1a1917";
    alert = "#c0201f";
    tape = "#b02226";
  };

  # Dark, but neutral: for when a full red field is too much. Greys take
  # the structural roles and red is reserved for what actually matters.
  ash = {
    bg = "#0d0d0e";
    panel = "#141416";
    border = "#2e2e31";
    dim = "#6b6b70";
    fg = "#c8c8cc";
    alert = "#de2e2e";
    tape = "#ff3b45";
  };
}
