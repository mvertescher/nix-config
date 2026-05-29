# Cybrcolors Base16 Palette
# Derived from the central palette.nix
let
  c = import ./palette.nix;
in
{
  scheme = "Cybrcolors";
  author = "cybrcore";

  # Backgrounds
  base00 = c.no0; # Black - Default Background
  base01 = c.no1; # Black mid - Lighter Background
  base02 = c.no2; # Black light - Selection Background

  # Foregrounds / Text / Guides
  base03 = c.wh1; # White mid - Dark grey, Comments/Invisibles
  base04 = c.me0; # Metal grey base - Dark Foreground/Status
  base05 = c.wh0; # White base - Default Foreground/Main text
  base06 = c.wh0; # White base - Light Foreground/Secondary text
  base07 = c.pi0; # Pink base - Lightest Foreground

  # Accents
  base08 = c.re0; # Red base
  base09 = c.or0; # Orange base
  base0A = c.ye0; # Yellow base
  base0B = c.gr0; # Green base
  base0C = c.cy0; # Cyan base
  base0D = c.bl0; # Blue base
  base0E = c.vi0; # Violet base
  base0F = c.pi0; # Pink base
}
