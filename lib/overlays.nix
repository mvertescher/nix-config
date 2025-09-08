{ inputs, system }:

let
    overlays = f: p: {
        builders = {
            mkHome = { pkgs ? f, extraHomeConfig ? { } }:
                import ../outputs/hm.nix { inherit extraHomeConfig inputs pkgs system; };
            mkNixos = { pkgs ? f, extraSystemConfig ? { } }:
                import ../outputs/os.nix { inherit extraSystemConfig inputs pkgs system; };
        };
    };
in
[
    overlays
    inputs.nixgl.overlay
]
