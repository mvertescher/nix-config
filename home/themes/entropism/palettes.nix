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
{
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
