{ inputs, system }:

let
    overlays = f: p: {
        nixgl = inputs.nixgl;
        
        builders = {
            mkHome = { pkgs ? f }:
                import ../outputs/hm.nix { inherit inputs pkgs system; };
        };
    };
in 
[
    overlays
]