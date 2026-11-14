# The cp-eras-ui toolkit and its screens.
#
# Do not `callPackage` this at a use site: `lib/in-tree.nix` does it
# once, and `pkgs.cp-eras-ui` is the canonical instance that the GUI
# module, the era builder and the golden matrix all name. Two
# instantiations of one package are two things free to drift, and the
# passthru tests below would then be evidence about the wrong one.
#
# `orbitron` is expected to be the overlay's `orbitron-vf` (this repo's
# googlefonts/orbitron-vf build). nixpkgs' own `orbitron` is a much
# older release with differently named files; preBuild tolerates it, but
# it is not what the goldens were rendered against.
{ lib
, runCommand
, weston
, mesa
, python3
, craneLib
, pkg-config
, cmake
, makeWrapper
, fontconfig
, freefont_ttf
, vulkan-loader
, libGL
, libxkbcommon
, libpulseaudio
, xorg
, wayland
, orbitron
, rajdhani-fontshare
, noto-cjk-subset
, google-fonts
}:

let
  # Michroma Regular (SIL OFL 1.1), the Kitsch wide label face: only that
  # family out of the pinned google/fonts checkout, and its OFL.txt from
  # the same source tree.
  michroma = google-fonts.override { fonts = [ "Michroma" ]; };
  michromaLicense = "${google-fonts.src}/ofl/michroma/OFL.txt";
  # Krona One Regular (SIL OFL 1.1), the Kitsch badge-note face, the same
  # way and from the same checkout.
  kronaOne = google-fonts.override { fonts = [ "Krona One" ]; };
  kronaOneLicense = "${google-fonts.src}/ofl/kronaone/OFL.txt";

  # crane's Cargo filter excludes .wgsl files, which the patched iced_wgpu
  # and cryoglyph crates include at compile time through include_str!.
  cleanSrc = lib.cleanSourceWith {
    src = lib.cleanSource ./.;
    filter = path: type:
      craneLib.filterCargoSources path type
      || (toString path == "${toString ./scripts}")
      || (toString path == "${toString ./scripts/make-kitsch-sans.py}")
      || (toString path == "${toString ./scripts/KITSCH-SANS-NOTICE.md}")
      || (lib.hasPrefix "${toString ./vendor/iced_wgpu/src/shader}/" (toString path)
        && lib.hasSuffix ".wgsl" (toString path))
      || (toString path == "${toString ./vendor/cryoglyph/src/shader.wgsl}");
  };

  cargoArtifacts = craneLib.buildDepsOnly {
    src = cleanSrc;
    # Registry dependents need iced_wgpu's and cryoglyph's actual APIs. Crane
    # replaces local crates with dummy targets; restore these patched
    # dependencies afterward.
    # Reference only the vendor directory to retain the dependency cache
    # when application sources change.
    extraDummyScript = ''
      rm -rf "$out/vendor/iced_wgpu"
      cp -r ${./vendor/iced_wgpu} "$out/vendor/iced_wgpu"
      chmod -R u+w "$out/vendor/iced_wgpu"
      rm -rf "$out/vendor/cryoglyph"
      cp -r ${./vendor/cryoglyph} "$out/vendor/cryoglyph"
      chmod -R u+w "$out/vendor/cryoglyph"
    '';
    nativeBuildInputs = [ pkg-config cmake ];
    buildInputs = [
      fontconfig
      vulkan-loader
      libGL
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      wayland
      libpulseaudio
    ];
  };
  package = craneLib.buildPackage {
  src = cleanSrc;
  inherit cargoArtifacts;
  pname = "cp-eras-ui";
  version = "0.0.0";

  nativeBuildInputs = [ pkg-config cmake makeWrapper (python3.withPackages (ps: [ ps.fonttools ])) ];
  buildInputs = [
    fontconfig
    vulkan-loader
    libGL
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    wayland
    libpulseaudio
  ];

  preBuild = ''
    mkdir -p fonts
    if [ -f "${orbitron}/share/fonts/truetype/Orbitron-Regular.ttf" ]; then
      cp ${orbitron}/share/fonts/truetype/*.ttf fonts/
    else
      cp "${orbitron}/share/fonts/truetype/Orbitron Light.ttf" fonts/Orbitron-Regular.ttf
      cp "${orbitron}/share/fonts/truetype/Orbitron Medium.ttf" fonts/Orbitron-Medium.ttf
      cp "${orbitron}/share/fonts/truetype/Orbitron Bold.ttf" fonts/Orbitron-SemiBold.ttf
      cp "${orbitron}/share/fonts/truetype/Orbitron Bold.ttf" fonts/Orbitron-Bold.ttf
    fi
    cp ${rajdhani-fontshare}/share/fonts/truetype/*.ttf fonts/
    cp ${noto-cjk-subset}/share/fonts/opentype/*.otf fonts/
    cp ${freefont_ttf}/share/fonts/truetype/FreeSansBold.ttf fonts/
    cp ${michroma}/share/fonts/truetype/Michroma-Regular.ttf fonts/
    cp ${kronaOne}/share/fonts/truetype/KronaOne-Regular.ttf fonts/
    python3 scripts/make-kitsch-sans.py fonts/FreeSansBold.ttf fonts/CP-Eras-Kitsch-Sans-Bold.ttf fonts/CP-Eras-Kitsch-Sans-Bold.audit.json
  '';

  postInstall = ''
    mkdir -p $out/share/fonts/truetype
    cp fonts/CP-Eras-Kitsch-Sans-Bold.ttf $out/share/fonts/truetype/
    cp fonts/Orbitron-*.ttf $out/share/fonts/truetype/
    # Preserve the bundled font license, embedding exception and credits.
    mkdir -p $out/share/doc/cp-eras-ui/fonts/freefont
    cp ${freefont_ttf.src}/{COPYING,README,CREDITS} $out/share/doc/cp-eras-ui/fonts/freefont/
    cp scripts/KITSCH-SANS-NOTICE.md $out/share/doc/cp-eras-ui/fonts/freefont/
    # Michroma is embedded under the SIL Open Font License 1.1, which
    # requires its text to travel with the font.
    mkdir -p $out/share/doc/cp-eras-ui/fonts/michroma
    cp ${michromaLicense} $out/share/doc/cp-eras-ui/fonts/michroma/OFL.txt
    mkdir -p $out/share/doc/cp-eras-ui/fonts/krona-one
    cp ${kronaOneLicense} $out/share/doc/cp-eras-ui/fonts/krona-one/OFL.txt
  '';

  postFixup = ''
    # Wrap every binary that landed in $out/bin: this covers any current
    # or future [[bin]] in Cargo.toml, so a new binary can never ship
    # unwrapped (no WGPU_POWER_PREF high, no LD_LIBRARY_PATH -> silent
    # black window). Guard against non-files just in case.
    for bin in $out/bin/*; do
      [ -f "$bin" ] || continue
      # This machine class exposes several Vulkan adapters (discrete
      # nvidia, the CPU's integrated RADV, llvmpipe). wgpu otherwise
      # picks one that cannot present to the display and the app draws a
      # solid black window - alive, silent, no error. set-default so it
      # can still be overridden.
      wrapProgram "$bin" \
        --set-default WGPU_POWER_PREF high \
        --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [
          vulkan-loader
          libGL
          libxkbcommon
          libpulseaudio
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ]}
    done
  '';

    meta = with lib; {
      description = "A UI toolkit using Rust and Iced.";
      # Application code is MIT; the embedded FreeSans face is GPL-3.0+;
      # the embedded Michroma and Krona One faces are OFL-1.1.
      license = [ licenses.mit licenses.gpl3Plus licenses.ofl ];
      platforms = platforms.linux;
    };
  };

  # Each era's own scheme, so a case renders the palette the desktop
  # would actually publish rather than a copy of it. Editing a palette
  # in home/themes therefore moves these goldens, which is the point:
  # the two sides of that contract cannot drift silently.
  eraCase =
    { screen, era, variant, freezeClock ? true }:
    import ./tests/visual.nix {
      inherit
        lib
        runCommand
        weston
        mesa
        python3
        era
        variant
        freezeClock
        ;
      cp-eras-ui = package;
      example = "cp-eras-ui-${screen}";
      width = 1600;
      height = 900;
      golden = ./tests/golden/${screen}-${era}-1600x900.png;
      roles = (import ../../../themes/${era}/scheme.nix).resolve { inherit variant; };
    };

  # The bar gets its own case file rather than another `eraCase`: it is
  # a different binary at a different size, and the reasoning for why it
  # cannot be the real bar wants somewhere to live. See tests/bar.nix.
  barCase =
    { era, variant }:
    import ./tests/bar.nix {
      inherit
        lib
        runCommand
        weston
        mesa
        python3
        era
        variant
        ;
      cp-eras-ui = package;
      roles = (import ../../../themes/${era}/scheme.nix).resolve { inherit variant; };
    };

  # Entropism's sampled palette is `nexus`; the rest call theirs
  # `reference`.
  variantOf = era: if era == "entropism" then "nexus" else "reference";

  eras = [
    "entropism"
    "kitsch"
    "neomil"
    "neokitsch"
  ];

  # One case per (screen, era). Every screen here is written once and
  # worn by all four eras, so the matrix is the standing evidence for
  # that claim rather than a promise in a comment.
  matrix =
    screen:
    lib.genAttrs eras (
      era:
      eraCase {
        inherit screen era;
        variant = variantOf era;
      }
    );
in
package.overrideAttrs (old: {
  passthru = (old.passthru or { }) // {
    tests = {
      # Source comparisons must use the Scene's uniform scale at fractional
      # viewport sizes. Keep this helper checked by the existing matrix.
      sourceSampling = runCommand "cp-eras-ui-source-sampling" {
        nativeBuildInputs = [ (python3.withPackages (ps: [ ps.pillow ])) ];
      } ''
        mkdir scripts
        cp ${./scripts/sample_source.py} scripts/sample_source.py
        cp ${./scripts/test_sample_source.py} scripts/test_sample_source.py
        python3 scripts/test_sample_source.py
        touch "$out"
      '';

      # The original case: no published theme, so this is the crate's
      # compiled fallback rendering the neomil dashboard.
      visual = import ./tests/visual.nix {
        inherit lib runCommand weston mesa python3;
        cp-eras-ui = package;
      };

      store = matrix "store";
      storeLive.neomil = eraCase {
        screen = "store";
        era = "neomil";
        variant = "reference";
        freezeClock = false;
      };
      login = matrix "login";
      mailbox = matrix "mailbox";
      # A frozen clock hid the standalone launcher's missing subscription.
      mailboxLive.neomil = eraCase {
        screen = "mailbox";
        era = "neomil";
        variant = "reference";
        freezeClock = false;
      };
      dashboard = matrix "dashboard";
      # The working mail client: the one screen assembled from iced
      # built-ins and `widgets` rather than a `Prim` table, so it is the
      # standing evidence for `catalog` and `panels`. Same frame as the
      # display-only `mailbox` it mirrors.
      mail = matrix "mail";

      bar = lib.genAttrs eras (era: barCase { inherit era; variant = variantOf era; });
    };
  };
})
