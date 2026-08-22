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
  pname = "neomil-ui";
  version = "0.1.0";

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
    for bin in neomil-ui-dashboard neomil-ui-mail neomil-ui-floppy; do
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
package.overrideAttrs (old: {
  passthru = (old.passthru or { }) // {
    tests.visual = import ./tests/visual.nix {
      inherit lib runCommand weston mesa python3;
      neomil-ui = package;
    };
  };
})
