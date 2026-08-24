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
, vulkan-loader
, libGL
, libxkbcommon
, libpulseaudio
, xorg
, wayland
, orbitron
, rajdhani-fontshare
}:

let
  cleanSrc = craneLib.cleanCargoSource ./.;

  cargoArtifacts = craneLib.buildDepsOnly {
    src = cleanSrc;
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
  pname = "cyberpunk-ui";
  version = "0.0.0";

  nativeBuildInputs = [ pkg-config cmake makeWrapper ];
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
  '';

  postInstall = ''
    mkdir -p $out/share/fonts/truetype
    cp fonts/Orbitron-*.ttf $out/share/fonts/truetype/
  '';

  postFixup = ''
    for bin in cyberpunk-ui-bar cyberpunk-ui-bar-window cyberpunk-ui-store cyberpunk-ui-login \
              cyberpunk-ui-mailbox cyberpunk-ui-dashboard cyberpunk-ui-mail cyberpunk-ui-floppy; do
      # This machine class exposes several Vulkan adapters (discrete
      # nvidia, the CPU's integrated RADV, llvmpipe). wgpu otherwise
      # picks one that cannot present to the display and the app draws a
      # solid black window - alive, silent, no error. set-default so it
      # can still be overridden.
      wrapProgram $out/bin/$bin \
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
      license = licenses.mit;
      platforms = platforms.linux;
    };
  };
in

# Visual regression: render the dashboard on a headless compositor and
# compare it against a committed golden image.
#
# passthru.tests rather than a gating checkPhase on purpose: a GPU-less
# compositor is exactly the kind of thing that fails for environmental
# reasons, and it should not block every build of the toolkit until it
# has proven stable.
#
# To run them: `./scripts/run_test_matrix.sh`. Nothing else reaches
# these -- the package takes callPackage arguments and this repo exports
# no configurations -- so before that script existed everyone wrote
# their own instantiation under /tmp, and one of those filled a 1.8 TB
# disk. tests/matrix.nix is the door; it explains why.
let
  # Each era's own scheme, so a case renders the palette the desktop
  # would actually publish rather than a copy of it. Editing a palette
  # in home/themes therefore moves these goldens, which is the point:
  # the two sides of that contract cannot drift silently.
  eraCase =
    { screen, era, variant }:
    import ./tests/visual.nix {
      inherit
        lib
        runCommand
        weston
        mesa
        python3
        era
        variant
        ;
      cyberpunk-ui = package;
      example = "cyberpunk-ui-${screen}";
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
      cyberpunk-ui = package;
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
      # The original case: no published theme, so this is the crate's
      # compiled fallback rendering the neomil dashboard.
      visual = import ./tests/visual.nix {
        inherit lib runCommand weston mesa python3;
        cyberpunk-ui = package;
      };

      store = matrix "store";
      login = matrix "login";
      mailbox = matrix "mailbox";
      dashboard = matrix "dashboard";

      bar = lib.genAttrs eras (era: barCase { inherit era; variant = variantOf era; });
    };
  };
})
