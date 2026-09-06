# This repo's own packages, so there is exactly one instance of each.
#
# They used to be `callPackage`d at each use site: cp-eras-ui in both
# `home/common/gui/default.nix` and `home/themes/lib/era.nix`, the
# fonts in those two plus `themes/neomil`. Two definitions free to
# drift apart is the shape of the Firefox duplication that left
# Sidebery installed under the vendored theme and absent under all four
# generated eras.
#
# A file of its own, and not a binding inside `overlays.nix`, because
# it must not need the flake's `inputs`: the cp-eras-ui dev shell
# (`home/common/pkgs/cp-eras-ui/shell.nix`) is entered with
# `nix develop -f`, outside the flake, and stages these fonts into the
# crate's `fonts/` for `include_bytes!`. Going through the flake from
# there costs a `builtins.getFlake` of this whole repo -- measured at
# a minute per entry -- so the shell applies this overlay to
# `<nixpkgs>` directly, and the bytes a `cargo build` embeds are the
# bytes the nix build embeds.
#
# Additive, which `PLAN.md` requires of anything the work wrapper
# consumes -- its call sites cannot be updated from here. Every name
# below is absent from nixpkgs, so nothing that already resolves
# changes meaning (checked by eval, not assumed).
#
# `orbitron` is *not* absent, which is why the in-tree font is not
# called that. nixpkgs ships `orbitron-2011-05-25`, a different and
# much older release whose files are named "Orbitron Light.ttf" rather
# than "Orbitron-Light.ttf"; taking the name would silently swap the
# font under any consumer of the nixpkgs one. The in-tree build tracks
# googlefonts/orbitron-vf, so it is `orbitron-vf` here.
#
# `cp-eras-ui` itself needs `craneLib`, which the flake's inputs overlay
# supplies; on bare `<nixpkgs>` plus this file it aborts at call time.
# The dev shell wants only the fonts, so that is fine.
final: prev: {
    orbitron-vf = final.callPackage ../home/common/pkgs/orbitron { };
    rajdhani-fontshare = final.callPackage ../home/common/pkgs/rajdhani-fontshare { };
    noto-cjk-subset = final.callPackage ../home/common/pkgs/noto-cjk-subset { };
    cp-eras-ui = final.callPackage ../home/common/pkgs/cp-eras-ui {
        orbitron = final.orbitron-vf;
    };
    repo-rs = final.callPackage ../home/common/pkgs/repo-rs.nix { };
    mpris-status = final.callPackage ../home/common/pkgs/mpris-status { };
}
