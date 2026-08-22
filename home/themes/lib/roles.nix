# Shared role vocabulary for the generated themes.
#
# The Cyberpunk 2077 style eras (entropism, neomil, kitsch, neokitsch)
# are all built the same way: a set of preset palettes expressed as
# semantic roles, resolved against per-host overrides, then projected
# into a base16 scheme for stylix. Only the palettes and a few style
# knobs differ, so that machinery lives here rather than being copied
# into each theme.
#
# cybr deliberately does not use this: it vendors upstream cybrcore
# assets (jsonc, rasi, SVG, shell scripts) rather than generating them,
# and its palette is a fixed 33-colour file.
#
#   roles = import ../lib/roles.nix;
#   resolved = roles.resolve { palettes = import ./palettes.nix; variant = "burn-in"; };
#   scheme = roles.toBase16 { name = "Entropism"; variant = "burn-in"; roles = resolved; };
rec {
  # Every generated theme speaks at least these. A maximalist era can add
  # its own on top (kitsch will want ornament/metal roles); the base
  # seven are what the shared component skins assume exist.
  names = [
    "bg" # desktop/terminal background
    "panel" # bar, titlebar, popup background
    "border" # 1px borders and separators
    "dim" # secondary text, comments
    "fg" # primary text
    "alert" # failure states only
    "tape" # improvised label accent; defaults to fg
  ];

  # Preset palette < per-host overrides, then `tape` falls back to the
  # resolved `fg`, so retinting the foreground carries the label accent
  # along instead of stranding it on the preset value.
  resolve =
    {
      palettes,
      variant,
      overrides ? { },
    }:
    let
      preset =
        palettes.${variant} or (throw "roles.resolve: no palette named '${variant}' (have: ${
          builtins.concatStringsSep ", " (builtins.attrNames palettes)
        })");
      base = preset // overrides;
    in
    base // { tape = base.tape or base.fg; };

  # Project roles onto base16. The mapping is deliberately lossy for the
  # monochrome eras: syntax highlighting collapses onto fg/dim/alert
  # rather than a rainbow, because a degraded display has one working
  # colour. A theme that wants more chroma passes `accents` to override
  # individual slots.
  toBase16 =
    {
      name,
      variant,
      roles,
      accents ? { },
    }:
    let
      hex = role: builtins.replaceStrings [ "#" ] [ "" ] roles.${role};
    in
    {
      scheme = "${name} ${variant}";
      author = "generated from home/themes/lib/roles.nix";

      base00 = hex "bg";
      base01 = hex "panel";
      base02 = hex "border";
      base03 = hex "dim";
      base04 = hex "dim";
      base05 = hex "fg";
      base06 = hex "fg";
      base07 = hex "fg";

      base08 = hex "alert";
      base09 = hex "alert";
      base0A = hex "tape";
      base0B = hex "fg";
      base0C = hex "dim";
      base0D = hex "fg";
      base0E = hex "dim";
      base0F = hex "alert";
    }
    // accents;
}
