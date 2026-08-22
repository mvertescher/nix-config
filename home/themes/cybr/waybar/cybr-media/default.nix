{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "cybr-media";
  version = "0.1.0";

  # Filter out the cargo build directory: a local `cargo build` here would
  # otherwise drag hundreds of MB into the store on every evaluation.
  src = lib.cleanSourceWith {
    src = ./.;
    filter =
      path: type:
      let
        base = baseNameOf path;
      in
      !(type == "directory" && base == "target");
  };

  # Vendored from the checked-in lock file, so the build is offline and needs
  # no cargoHash to be refreshed whenever a dependency moves.
  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "MPRIS now-playing line for waybar, replacing cybr-waybar's mediaplayer.py";
    longDescription = ''
      Speaks D-Bus directly via zbus rather than going through PyGObject and
      the Playerctl GIR typelib, so the bar's music module needs no python3
      at runtime. Emits waybar's JSON custom-module format on stdout.
    '';
    license = lib.licenses.gpl3Only;
    mainProgram = "cybr-media";
    platforms = lib.platforms.linux;
  };
}
