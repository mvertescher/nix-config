{ inputs }:

let
    overlays = f: p: {
        craneLib = inputs.crane.mkLib p;
        builders = {
            mkHome = { pkgs ? f, extraHomeConfig ? { } }:
                import ../outputs/hm.nix { inherit extraHomeConfig inputs pkgs; };
            mkNixos = { pkgs ? f, extraSystemConfig ? { } }:
                import ../outputs/os.nix { inherit extraSystemConfig inputs pkgs; };
        };
    };
in
[
    overlays
    inputs.nixgl.overlay
]
