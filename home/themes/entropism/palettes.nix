# Preset palettes for the entropism theme.
#
# Entropism is the salvaged-hardware era of Cyberpunk 2077's style
# timeline: necessity over style, function only, no ornament. Each preset
# is a degraded display rather than a colour scheme -- one usable
# foreground, one dim, one alert, and nothing else.
#
# `tape` is deliberately omitted where it should simply follow `fg`; the
# resolver fills it in. Only dead-pixel wants a distinct label colour
# (the improvised gaffer-tape yellow you write on with a marker).
#
# One of these is sampled and three are not. `nexus` is transcribed from
# the pixel reads in home/common/pkgs/cp-eras-ui/docs/entropism, taken
# off the Behance modules; the other three predate that pass and were
# designed to the era's *description*. They are good schemes and stay,
# but if you want the era as published, that is `nexus`.
{
  # Sampled. One hue -- sage green on a warm dark olive-brown -- which
  # is the whole era: square, 1px, no glow, no gradients. Named for the
  # build strings the reference screens are footed with ("PROVIDED BY
  # NEXUS NETWORK V10.8").
  #
  # `alert` is the mid sage rather than a second hue on purpose. The
  # references have no red anywhere; escalation is carried by brightness,
  # and inventing an alarm colour here would break the one rule the era
  # actually has.
  nexus = {
    bg = "#110c07";
    panel = "#181109";
    border = "#5d7752";
    dim = "#3d4d38";
    fg = "#94bb94";
    alert = "#728f76";
    tape = "#9cb795";
  };

  burn-in = {
    # Amber phosphor with the ghost of a menu burned into it.
    bg = "#161009";
    panel = "#1c130a";
    border = "#4a3416";
    dim = "#8a6a34";
    fg = "#d9a24a";
    alert = "#c2452d";
  };

  dead-pixel = {
    # Salvaged e-waste LCD: grey, slightly green-shifted, uneven.
    bg = "#191a19";
    panel = "#131413";
    border = "#3d3e3a";
    dim = "#5f615c";
    fg = "#b6b3a8";
    alert = "#b03a2e";
    tape = "#c7b458";
  };

  salvage-phosphor = {
    # Desaturated green CRT, the cheapest surviving terminal.
    bg = "#0d120d";
    panel = "#101610";
    border = "#2c3a2a";
    dim = "#4c5f48";
    fg = "#93b48a";
    alert = "#b8923f";
  };
}
