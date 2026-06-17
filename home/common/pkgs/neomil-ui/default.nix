{ lib
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
in
craneLib.buildPackage {
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
    cp ${orbitron}/share/fonts/truetype/*.ttf fonts/
  '';

  postFixup = ''
    for bin in neomil-ui-demo; do
      wrapProgram $out/bin/$bin \
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
}
