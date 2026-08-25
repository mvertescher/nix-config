# The door to the golden matrix.
#
# `passthru.tests` in ../default.nix *defines* the cases, but nothing
# could reach them: this repo's flake deliberately exports no
# configurations to hang them off, and the package used to be reachable
# only through `callPackage` with three arguments supplied by hand. So
# every agent and every session that needed to run a golden wrote its own
# throwaway instantiation somewhere under /tmp. Three variants of the
# same twelve lines were written in one night. This is that file,
# committed.
#
# The package is now `pkgs.cyberpunk-ui`, from this repo's overlay
# (../../../../lib/overlays.nix), so this file no longer instantiates
# anything -- which is the point. A matrix rendering its *own* build of
# the crate would be a second instance free to drift from the one the
# desktop ships, and the whole value of the goldens is that they are
# evidence about the shipped thing. Same overlay list, same pkgs
# constructor, therefore the same derivation as `home/common/gui` and
# `home/themes/lib/era.nix` name.
#
# Takes this repo's overlaid `pkgs` -- `(getFlake ...).out.pkgs`, the
# escape hatch the flake documents for exactly this -- and returns:
#
#   tests   the nested set as ../default.nix declares it,
#           tests.<screen>.<era>, tests.bar.<era>, tests.visual
#   cases   the same derivations flattened to "store.kitsch" = <drv>,
#           so a runner can iterate without knowing the shape
#
# `pkgs` has no default on purpose. `import <nixpkgs> {}` has no
# `cyberpunk-ui` and no `craneLib`, so it would evaluate and then fail
# on a missing attribute somewhere less obvious than here. Use
# ../scripts/run_test_matrix.sh; it wires this up correctly and is the
# supported entry point.
#
# ---------------------------------------------------------------------
# If you are about to write your own instantiation anyway, read this.
#
# Fetch this repo with `git+file:`, never `path:`. The path fetcher
# copies a directory into the store wholesale and does *not* honour
# .gitignore -- that is git+file:'s behaviour, not its own -- and a
# working checkout of this crate carries a multi-gigabyte cargo
# `target/`. Because the store path is content-addressed, every distinct
# tree state mints another copy, and a tree under active editing changes
# between every run. On 2026-08-24 that put 285 source paths on one
# disk, the largest 24 GB each, and filled 1.8 TB to 100%. The same tree
# through the git fetcher is a few megabytes: it sees tracked files only.
#
# The corollary of "tracked files only" is that a NEW file is invisible
# until it is at least `git add -N`'d -- the same rule the flake itself
# follows. A run that ignores the case you just added is almost always
# this.
{ pkgs }:

let
  inherit (pkgs) lib;

  crate = pkgs.cyberpunk-ui;

  # Flatten by walking rather than by restating the era and screen
  # lists. Those live in ../default.nix; a copy here would be a second
  # place to forget to update, and the matrix's whole point is that its
  # membership is derived, not asserted.
  flatten =
    prefix: set:
    lib.concatMapAttrs (
      name: value:
      let
        key = if prefix == "" then name else "${prefix}.${name}";
      in
      if lib.isDerivation value then
        { ${key} = value; }
      else if lib.isAttrs value then
        flatten key value
      else
        { }
    ) set;
in
{
  inherit (crate) tests;
  cases = flatten "" crate.tests;
}
