# The crate's dev shell: `nix develop -f shell.nix` (or `nix-shell`)
# from this directory, for `cargo build`/`cargo test` and the headless
# screenshot scripts.
#
# Not the flake, on purpose: `builtins.getFlake` on this repo costs
# about a minute per entry (a git+file copy plus every input's eval),
# which is not a dev-shell price. NixOS's default flake registry pins
# `<nixpkgs>` to the nixpkgs the running system was built from (the
# flake's, once switched; /etc/nix/registry.json says which), and
# `lib/in-tree.nix` is the overlay the flake applies, so the fonts
# staged into `fonts/` below are the overlay's instances -- the same
# store paths `pkgs.cp-eras-ui`'s preBuild copies -- and not a second
# `callPackage` free to drift from them.
{ pkgs ? import <nixpkgs> { overlays = [ (import ../../../../lib/in-tree.nix) ]; } }:

let
  inherit (pkgs) rajdhani-fontshare noto-cjk-subset;
  orbitron = pkgs.orbitron-vf;

  # Where `fonts/` goes: this directory, not the cwd. Entering the
  # shell from `public/` used to leave a stray `public/fonts/` behind.
  crate = toString ./.;

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
    noto-cjk-subset
    orbitron
    mesa
  ];

  shellHook = ''
    # Stage the fonts `include_bytes!` reads. `orbitron-vf`'s files are
    # already named as `src/fonts.rs` expects; the nixpkgs-orbitron
    # rename that `default.nix` tolerates cannot arise here.
    mkdir -p "${crate}/fonts"
    cp -f ${orbitron}/share/fonts/truetype/*.ttf "${crate}/fonts/"
    cp -f ${rajdhani-fontshare}/share/fonts/truetype/*.ttf "${crate}/fonts/"
    cp -f ${noto-cjk-subset}/share/fonts/opentype/*.otf "${crate}/fonts/"
    chmod +w "${crate}"/fonts/*.ttf "${crate}"/fonts/*.otf

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
