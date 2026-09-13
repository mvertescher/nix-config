# Every overlay this repo applies, in one list. The flake's own `pkgs`
# and both builders read it, so every host and every wrapper sees the
# same package set.
{ inputs }:

let
    overlays = f: p: {
        craneLib = inputs.crane.mkLib p;
    };

    llmAgentsOverlay = final: prev: {
        claude-code = inputs.llm-agents.packages.${prev.stdenv.hostPlatform.system}.claude-code;
        antigravity-cli = inputs.llm-agents.packages.${prev.stdenv.hostPlatform.system}.antigravity-cli;
        # nixpkgs has codex too, but the pin trails it by a handful of
        # releases; taken from llm-agents for the same reason
        # claude-code is, that these move faster than the pin does.
        codex = inputs.llm-agents.packages.${prev.stdenv.hostPlatform.system}.codex;
    };

    # This repo's own packages; see the file for why it is one.
    inTreePkgs = import ./in-tree.nix;
in
[
    overlays
    inputs.nixgl.overlay
    inputs.rust-overlay.overlays.default
    llmAgentsOverlay
    inTreePkgs
]
