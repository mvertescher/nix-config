# Per-host `pkgs` for the two builders — the half of `mkNixos` and
# `mkHome` that is genuinely identical, written once.
#
# Both builders accept `extraOverlays` for the whole host set and honour
# a per-host `extraOverlays` on top of it. The common case is that no
# host adds any, and then every host should share a single nixpkgs
# instantiation instead of re-importing it per host; that is what
# `shared` is for, and it is the part that was easiest to get subtly
# different in two places.
#
# Extracting it has a second point beyond not repeating twelve lines.
# The two `let` blocks read as identical at a glance, which is exactly
# how one misses that their `user` defaults are *not* the same —
# "mverte" for `mkNixos`, "mvertescher" for `mkHome`. That difference is
# deliberate, load-bearing for the work wrapper and frozen (see
# `PLAN.md`); with the shared half gone it is the only thing left in
# either block, which is where the reader's attention belongs.
{
  inputs,
  overlays,
  extraOverlays ? [ ],
}:

let
  mkPkgs =
    hostOverlays:
    import ./pkgs.nix {
      inherit inputs;
      overlays = overlays ++ extraOverlays ++ hostOverlays;
    };

  shared = mkPkgs [ ];
in

host:
let
  hostOverlays = host.extraOverlays or [ ];
in
(if hostOverlays == [ ] then shared else mkPkgs hostOverlays)
  (host.system or "x86_64-linux")
