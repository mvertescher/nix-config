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
  # Every generated theme speaks at least these. A maximalist era adds
  # its own on top -- see `extraNames` below, which is where the
  # ornament and metal roles this comment used to anticipate now live.
  # The base seven are what the shared component skins assume exist, and
  # the only ones `toBase16` ever reads.
  names = [
    "bg" # desktop/terminal background
    "panel" # bar, titlebar, popup background
    "border" # 1px borders and separators
    "dim" # secondary text, comments
    "fg" # primary text
    "alert" # failure states only
    "tape" # improvised label accent; defaults to fg
  ];

  # The maximalist extension. Every one of these is optional: a palette
  # that declares none of them resolves to exactly the attrset it did
  # before this list existed, which is what keeps the minimalist eras
  # and the base16 projection untouched.
  #
  # These are not guesses at what an ornamental era might like. Each one
  # is a fill or an ink that appears in `home/common/pkgs/cp-eras-ui/
  # docs/{kitsch,neokitsch}/target-{app,components}.svg` and that the
  # base seven cannot name:
  #
  #   banner/onBanner  The notched accent band. Kitsch's shelf band
  #                    (`M360 228 h242 v20 h-230 l-12 8 Z`, #fcc428)
  #                    pokes past the card's left edge on every product
  #                    card and carries compliance glyphs in #37220f;
  #                    the same shape is the BRAINDANCE flag tag and the
  #                    selected mail row. Neokitsch's is the card footer
  #                    nameplate (#d3b279 with #3a2410 ink) and the
  #                    BASKET panel. Not `tape`: tape is a *frame*
  #                    colour in kitsch (the orange CRT bezel) and never
  #                    carries text. Not `select` either -- neokitsch
  #                    selects with veneer and banners in champagne.
  #                    The ink is a role of its own because the band is
  #                    a light fill in a dark palette and `fg` is
  #                    illegible on it.
  #
  #   emphasis/onEmphasis
  #                    The highlight band behind key figures: kitsch's
  #                    mint stat bar under DPS/PNT/ACC/ROF (#87f4d9 with
  #                    #0b3b31 figures), also its ENTER pill. The crate
  #                    already had this as an era-owned constant; naming
  #                    it here is what lets a variant retint it instead
  #                    of shipping one sampled mint for light and dark
  #                    alike.
  #
  #   bevel/shade      The two edges of a raised or extruded surface.
  #                    Kitsch's fan-menu slabs are a lit face (#2bc4ac)
  #                    over stacked outlines receding up-right in a
  #                    darker teal (#177a6b); neokitsch's device frame
  #                    is a double stroke, an outer gold highlight
  #                    (#c69a55, top stop of `frameG`) against an inner
  #                    #5e3414. One `border` cannot be both sides of a
  #                    bevel.
  #
  #   ornament         Non-structural decoration, drawn as a solid.
  #                    Kitsch's page-curl at the foot of the nav
  #                    container (#1cb39b, "one solid page-curl per
  #                    screen"), its chip squares and PROTECTED bars;
  #                    neokitsch's strata dividers, the fine lines
  #                    bunching into a wedge (#634427). Distinct from
  #                    `border` -- it never encloses anything -- and
  #                    from `fg`, which it is deliberately more (kitsch)
  #                    or less (neokitsch) saturated than.
  #
  #   inset            A recessed fill, below the surface it sits on:
  #                    neokitsch's login field (#2c1c14) and socket
  #                    wells. `bg` is the page, `panel` is the raised
  #                    thing; neither is the hole in it.
  #
  # A role nothing in an era's references uses stays undeclared --
  # kitsch has no `inset` (its cards are unfilled outlines) and
  # neokitsch has no `emphasis` (it has no highlight band). Consumers
  # are expected to fall back rather than assume presence.
  extraNames = [
    "banner" # notched accent band fill
    "onBanner" # text and line-work on `banner`
    "emphasis" # highlight band behind key figures
    "onEmphasis" # figures sitting on `emphasis`
    "bevel" # lit edge/face of a raised surface
    "shade" # its receding, shaded edge
    "ornament" # solid non-structural decoration
    "inset" # recessed fill: input wells, sockets
  ];

  # Which optional roles a resolved palette actually declares, in
  # `extraNames` order. Consumers that serialise the palette (era.nix's
  # theme file) iterate this rather than `extraNames`, so an era that
  # declares nothing emits nothing.
  #
  # `builtins.hasAttr` and not `resolved ? n`: the `?` operator takes an
  # attrpath, so `? n` asks for an attribute literally *named* "n" and
  # silently returns the empty list for every era. That version type-
  # checked, evaluated and produced no output at all.
  extrasOf = resolved: builtins.filter (n: builtins.hasAttr n resolved) extraNames;

  # Fallbacks *within* a declared pair, and only there. Declaring a
  # banner fill without its ink is a plausible slip; being handed an ink
  # for a banner that does not exist is not. So this emits a key only
  # when its partner is present, which means an era that declares no
  # extras gets `{ }` back and resolves bit-for-bit as before.
  pairFallbacks =
    base:
    (if base ? banner && !(base ? onBanner) then { onBanner = base.bg; } else { })
    // (if base ? emphasis && !(base ? onEmphasis) then { onEmphasis = base.bg; } else { })
    // (if base ? bevel && !(base ? shade) then { shade = base.border; } else { })
    // (if base ? shade && !(base ? bevel) then { bevel = base.border; } else { });

  # Preset palette < per-host overrides, then `tape` falls back to the
  # resolved `fg`, so retinting the foreground carries the label accent
  # along instead of stranding it on the preset value.
  #
  # Optional roles ride through untouched: they were already preserved
  # by the `preset // overrides` merge, and `pairFallbacks` adds nothing
  # for a palette that declares none.
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
    base // { tape = base.tape or base.fg; } // pairFallbacks base;

  # Hex parsing, because a light variant has to be recognised as such and
  # nix has no builtin for it.
  hexDigit =
    c:
    let
      digits = {
        "0" = 0; "1" = 1; "2" = 2; "3" = 3; "4" = 4;
        "5" = 5; "6" = 6; "7" = 7; "8" = 8; "9" = 9;
        a = 10; b = 11; c = 12; d = 13; e = 14; f = 15;
        A = 10; B = 11; C = 12; D = 13; E = 14; F = 15;
      };
    in
    digits.${c} or (throw "roles: '${c}' is not a hex digit");

  channel =
    hex: offset:
    (hexDigit (builtins.substring offset 1 hex)) * 16
    + (hexDigit (builtins.substring (offset + 1) 1 hex));

  # Rec. 601 luma, which is close enough to decide light from dark and
  # avoids a gamma-correct implementation nobody needs here.
  luma =
    color:
    let
      hex = builtins.replaceStrings [ "#" ] [ "" ] color;
    in
    (0.299 * (channel hex 0) + 0.587 * (channel hex 2) + 0.114 * (channel hex 4)) / 255.0;

  # Stylix needs to know which way round a scheme runs: with polarity
  # left at "either" it guesses, and picks the wrong GTK and icon
  # variants for a light palette. Infer it from the background rather
  # than making every palette restate it.
  polarityOf = roles: if luma roles.bg < 0.5 then "dark" else "light";

  # Project roles onto base16. The mapping is deliberately lossy for the
  # monochrome eras: syntax highlighting collapses onto fg/dim/alert
  # rather than a rainbow, because a degraded display has one working
  # colour. A theme that wants more chroma passes `accents` to override
  # individual slots.
  #
  # `extraNames` deliberately has no path into this. Base16 has sixteen
  # slots and stylix drives the console, the greeter and every themed
  # app from them; a new role leaking in here would retint the whole
  # desktop as a side effect of an app-level ornament. The projection
  # reads the seven base roles by name and nothing else, so adding an
  # optional role cannot move a slot. An era that genuinely wants one of
  # its ornaments in the syntax palette says so explicitly through
  # `accents`, which is what kitsch already does with its mint.
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
