# This repo's own packages, so there is exactly one instance of each.
#
# A file of its own, and not a binding inside `overlays.nix`, because
# it needs none of the flake's `inputs` and can be applied to a bare
# `<nixpkgs>`.
#
# Additive, which `PLAN.md` requires of anything the work wrapper
# consumes -- its call sites cannot be updated from here. Every name
# below is absent from nixpkgs, so nothing that already resolves
# changes meaning.
#
# The cp-eras-ui crate and its three fonts (`orbitron-vf`,
# `rajdhani-fontshare`, `noto-cjk-subset`) were here until 2026-11-16.
# They live in the `ui` repository now, which is private, so this
# public flake cannot name it: a consumer that wants them adds that
# flake's overlay itself (mkNixos' per-host `extraOverlays`).
final: _prev: {
    # repo-rs removed: upstream https://github.com/sunbeamdotpt/repo-rs deleted
    mpris-status = final.callPackage ../home/common/pkgs/mpris-status { };
}
