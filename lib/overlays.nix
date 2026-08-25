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
    };

    # This repo's own packages, so there is exactly one instance of each.
    #
    # They used to be `callPackage`d at each use site: cyberpunk-ui in
    # both `home/common/gui/default.nix` and `home/themes/lib/era.nix`,
    # the fonts in those two plus `themes/neomil`. Two definitions free
    # to drift apart is the shape of the Firefox duplication that left
    # Sidebery installed under the vendored theme and absent under all
    # four generated eras.
    #
    # Additive, which `PLAN.md` requires of anything the work wrapper
    # consumes -- its call sites cannot be updated from here. All three
    # names below are absent from nixpkgs, so nothing that already
    # resolves changes meaning.
    #
    # `orbitron` is *not* absent, which is why the in-tree font is not
    # called that. nixpkgs ships `orbitron-2011-05-25`, a different and
    # much older release whose files are named "Orbitron Light.ttf"
    # rather than "Orbitron-Light.ttf"; taking the name would silently
    # swap the font under any consumer of the nixpkgs one. The in-tree
    # build tracks googlefonts/orbitron-vf, so it is `orbitron-vf` here.
    inTreePkgs = final: prev: {
        orbitron-vf = final.callPackage ../home/common/pkgs/orbitron { };
        rajdhani-fontshare = final.callPackage ../home/common/pkgs/rajdhani-fontshare { };
        cyberpunk-ui = final.callPackage ../home/common/pkgs/cyberpunk-ui {
            orbitron = final.orbitron-vf;
        };
    };
in
[
    overlays
    inputs.nixgl.overlay
    inputs.rust-overlay.overlays.default
    llmAgentsOverlay
    inTreePkgs
]
