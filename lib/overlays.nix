{ inputs }:

let
    overlays = f: p: {
        craneLib = inputs.crane.mkLib p;
        # Frozen shim: the home-manager-only wrapper calls
        # `pkgs.builders.mkHome { extraHomeConfig }` and its call sites
        # can't be updated from here — signature changes must be
        # additive. NixOS wrappers use `nix-config.lib.mkNixos` instead.
        builders = {
            mkHome = { pkgs ? p, extraHomeConfig ? { } }:
                import ./mkHome.nix { inherit extraHomeConfig inputs pkgs; };
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
