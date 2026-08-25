{ pkgs ? import <nixpkgs> { } }:

let
  rajdhani-fontshare = pkgs.callPackage ../rajdhani-fontshare { };
  orbitron = pkgs.callPackage ../orbitron { };

  driversPath = "${pkgs.mesa}/lib/dri";
  vulkanICD = "${pkgs.mesa}/share/vulkan/icd.d/lvp_icd.x86_64.json";
  eglVendor = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
    cmake
    weston
    grim
  ];

  buildInputs = with pkgs; [
    fontconfig
    vulkan-loader
    libGL
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    wayland
    # libpulse-binding links against the PulseAudio client library and
    # finds it through pkg-config's `libpulse`.
    libpulseaudio
    rajdhani-fontshare
    orbitron
    mesa
  ];

  shellHook = ''
    # Ensure fonts are present for include_bytes! in cargo builds
    mkdir -p fonts
    if [ -f "${orbitron}/share/fonts/truetype/Orbitron-Regular.ttf" ]; then
      cp -f ${orbitron}/share/fonts/truetype/*.ttf fonts/
    else
      cp -f "${orbitron}/share/fonts/truetype/Orbitron Light.ttf" fonts/Orbitron-Regular.ttf
      cp -f "${orbitron}/share/fonts/truetype/Orbitron Medium.ttf" fonts/Orbitron-Medium.ttf
      cp -f "${orbitron}/share/fonts/truetype/Orbitron Bold.ttf" fonts/Orbitron-SemiBold.ttf
      cp -f "${orbitron}/share/fonts/truetype/Orbitron Bold.ttf" fonts/Orbitron-Bold.ttf
    fi
    cp -f ${rajdhani-fontshare}/share/fonts/truetype/*.ttf fonts/
    chmod +w fonts/*.ttf

    # If running in headless/software-force mode, configure the drivers
    if [ -n "''${FORCE_SOFTWARE_GL:-}" ]; then
      echo "[shell.nix] Forcing software rendering (llvmpipe/lavapipe)..."
      export LIBGL_ALWAYS_SOFTWARE=1
      export GALLIUM_DRIVER=llvmpipe
      export WGPU_BACKEND=gl
      export LIBGL_DRIVERS_PATH=${driversPath}
      export VK_ICD_FILENAMES=${vulkanICD}
      export __EGL_VENDOR_LIBRARY_FILENAMES=${eglVendor}
    fi

    # Prepend our library paths (always needed for runtime links)
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (with pkgs; [
      vulkan-loader
      libGL
      libxkbcommon
      wayland
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      mesa
    ])}:$LD_LIBRARY_PATH
  '';
}
