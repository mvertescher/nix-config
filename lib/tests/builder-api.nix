# Does the builder API still accept a minimal call?
#
# `PLAN.md` states the constraint this exists to enforce: a second
# consumer (the work machine) uses `lib.mkHome` from its own wrapper, and
# **its call sites cannot be updated from this repo**. So changes here
# have to be additive -- a new *optional* argument is fine, a new
# required one silently breaks a consumer that nobody here can see, and
# it breaks it at their next pin bump rather than in this repo's CI.
#
# Nothing else in this repo can catch that. There are no
# `nixosConfigurations` here by design, so a wrapper's build is the only
# thing that exercises the builders -- and every wrapper is private.
#
# The fixtures below are deliberately *minimal*: they pass only what the
# documented signature says is required. If a builder grows a new
# mandatory argument, or renames one, or loses a default, these stop
# evaluating. That is the whole test. They are not hosts -- they are the
# smallest thing each builder will accept, and they exist so the API has
# a consumer inside this repo.
{
  lib,
  runCommand,
  mkNixos,
  mkHome,
}:

let
  # The floor for any NixOS eval: a root filesystem, a boot loader and a
  # state version. Anything above this floor belongs to a wrapper.
  minimalHost = {
    fileSystems."/" = {
      device = "/dev/null";
      fsType = "ext4";
    };
    boot.loader.grub.devices = [ "nodev" ];
    system.stateVersion = "24.05";
  };

  nixos = mkNixos {
    hosts.fixture = {
      modules = [ minimalHost ];

      # `homeModules` is documented as optional and is optional
      # *syntactically*, but `mkNixos` always wires home-manager in for
      # the host's user, and that user has no `home.stateVersion` unless
      # something supplies one. So a host set with no `homeModules` at
      # all does not evaluate. Found by this fixture on its first run --
      # which is the fixture earning its place, since every real consumer
      # passes some and would never hit it.
      homeModules = [ { home.stateVersion = "24.05"; } ];
    };
  };

  home = mkHome {
    hosts.fixture.modules = [ { home.stateVersion = "24.05"; } ];
  };

  # What to force, and what deliberately not to.
  #
  # The obvious assertion is each configuration's `drvPath`, and it is
  # the wrong one: instantiating a full toplevel drags in
  # import-from-derivation and starts *building* Rust crates, which is
  # wildly disproportionate for checking a function signature and would
  # not fit a CI runner's disk. A signature break -- a renamed argument,
  # a newly required one, a lost default -- fails at the `mkNixos { ... }`
  # call itself, so forcing anything past it is enough to catch it.
  #
  # These are cheap module-system reads that prove each builder accepted
  # the host set and produced an evaluated configuration. **This does not
  # prove a configuration builds** -- that is a wrapper's job, and
  # nix-config-private's `./check` does it for the real hosts.
  facts = {
    # mapAttrs over the host set: proves the name reached the config, so
    # a builder that returned an empty set fails here rather than
    # silently deploying nothing.
    nixosHostName = nixos.fixture.config.networking.hostName;
    # The documented default, and load-bearing for a wrapper that omits
    # `user`.
    nixosUser = nixos.fixture.config.users.users.mverte.name;
    homeUser = home.fixture.config.home.username;
    homeDir = home.fixture.config.home.homeDirectory;
  };

  expected = {
    nixosHostName = "fixture";
    nixosUser = "mverte";
    homeUser = "mvertescher";
    homeDir = "/home/mvertescher";
  };

  wrong = lib.filterAttrs (n: v: v != facts.${n}) expected;
in
runCommand "builder-api-check"
  {
    report = lib.generators.toPretty { } facts;
    wrong = lib.generators.toPretty { } wrong;
  }
  ''
    printf 'builders accepted a minimal host set:\n%s\n' "$report" | tee "$out"

    if [ "$wrong" != "{ }" ]; then
      printf '\nunexpected values (left = expected):\n%s\n' "$wrong" >&2
      exit 1
    fi
  ''
