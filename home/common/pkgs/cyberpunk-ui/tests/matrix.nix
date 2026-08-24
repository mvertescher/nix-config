# The door to the golden matrix.
#
# `passthru.tests` in ../default.nix *defines* the cases, but nothing
# could reach them: the package takes `callPackage` arguments (craneLib,
# orbitron, rajdhani-fontshare) and this repo's flake deliberately
# exports no configurations to hang them off. So every agent and every
# session that needed to run a golden wrote its own throwaway
# instantiation somewhere under /tmp. Three variants of the same twelve
# lines were written in one night. This is that file, committed.
#
# Takes this repo's overlaid `pkgs` -- `(getFlake ...).out.pkgs`, the
# escape hatch the flake documents for exactly this -- and returns:
#
#   tests   the nested set as ../default.nix declares it,
#           tests.<screen>.<era>, tests.bar.<era>, tests.visual
#   cases   the same derivations flattened to "store.kitsch" = <drv>,
#           so a runner can iterate without knowing the shape
#
# `pkgs` has no default on purpose. `import <nixpkgs> {}` would evaluate
# and then fail deep inside crane with a missing `craneLib`, which is a
# worse error than being told the argument is required. Use
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

  pkgsDir = ../..;

  orbitron = pkgs.callPackage (pkgsDir + "/orbitron") { };
  rajdhani-fontshare = pkgs.callPackage (pkgsDir + "/rajdhani-fontshare") { };

  crate = pkgs.callPackage ../. { inherit orbitron rajdhani-fontshare; };

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
