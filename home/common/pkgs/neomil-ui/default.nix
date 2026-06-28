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
    for bin in neomil-ui-dashboard; do
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
