# This repo's own packages, so there is exactly one instance of each.
#
# A file of its own, and not a binding inside `overlays.nix`, because
# it needs none of the flake's `inputs` and can be applied to a bare
# `<nixpkgs>`.
#
# Additive, which `AGENTS.md` ("Builder API") requires of anything the
# work wrapper consumes -- its call sites cannot be updated from here.
# Every name below is absent from nixpkgs, so nothing that already
# resolves changes meaning.
#
# A consumer that wants packages this repository does not carry adds
# its own overlay (the builders' `extraOverlays`).
final: _prev: {
    # repo-rs removed: upstream https://github.com/sunbeamdotpt/repo-rs deleted
    mpris-status = final.callPackage ../home/common/pkgs/mpris-status { };
}
