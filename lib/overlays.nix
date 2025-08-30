{ inputs, system }:

let
    overlays = f: p: {
        builders = {
            mkHome = { pkgs ? f, extraHomeConfig ? { } }:
                import ../outputs/hm.nix { inherit extraHomeConfig inputs pkgs system; };
        };
    };
in
[
    overlays
    inputs.nixgl.overlay
]