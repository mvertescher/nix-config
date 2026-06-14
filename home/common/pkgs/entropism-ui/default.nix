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
  pname = "entropism-ui";
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
    cp ${rajdhani-fontshare}/share/fonts/truetype/*.ttf fonts/
  '';

  postFixup = ''
    for bin in entropism-ui-demo entropism-ui-login entropism-ui-mail entropism-ui-store entropism-ui-default-layout; do
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
    description = "A UI demo app for a cybr themed application using Iced.";
    license = licenses.mit;
    platforms = platforms.linux;
  };
}
