{ inputs }:

let
    overlays = f: p: {
        craneLib = inputs.crane.mkLib p;
        builders = {
            mkHome = { pkgs ? p, extraHomeConfig ? { } }:
                import ../outputs/hm.nix { inherit extraHomeConfig inputs pkgs; };
            mkNixos = { pkgs ? f, extraSystemConfig ? { } }:
                import ../outputs/os.nix { inherit extraSystemConfig inputs pkgs; };
        };
    };

    llmAgentsOverlay = final: prev: {
        claude-code = inputs.llm-agents.packages.${prev.stdenv.hostPlatform.system}.claude-code;
    };
in
[
    overlays
    inputs.nixgl.overlay
    inputs.rust-overlay.overlays.default
    llmAgentsOverlay
]
