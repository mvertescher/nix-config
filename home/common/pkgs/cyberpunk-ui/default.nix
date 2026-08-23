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
    for bin in cyberpunk-ui-store cyberpunk-ui-dashboard cyberpunk-ui-mail cyberpunk-ui-floppy; do
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
let
  # Each era's own scheme, so a case renders the palette the desktop
  # would actually publish rather than a copy of it. Editing a palette
  # in home/themes therefore moves these goldens, which is the point:
  # the two sides of that contract cannot drift silently.
  eraCase =
    { era, variant }:
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
      example = "cyberpunk-ui-store";
      width = 1600;
      height = 900;
      golden = ./tests/golden/store-${era}-1600x900.png;
      roles = (import ../../../themes/${era}/scheme.nix).resolve { inherit variant; };
    };
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

      # The store screen in each era. One implementation, four dresses --
      # so if the era abstraction ever collapses back into "the same
      # screen four times", these four goldens are where it shows.
      store = {
        entropism = eraCase {
          era = "entropism";
          variant = "nexus";
        };
        kitsch = eraCase {
          era = "kitsch";
          variant = "reference";
        };
        neomil = eraCase {
          era = "neomil";
          variant = "reference";
        };
        neokitsch = eraCase {
          era = "neokitsch";
          variant = "reference";
        };
      };
    };
  };
})
