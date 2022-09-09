# nix-config

> My Nix/NixOS configuration

## Non NixOS

If you're on a Unix-like system that's not NixOS, these instructions should
hopefully work.

1. Install Nix and activate the local shell:

```
sh <(curl -L https://nixos.org/nix/install) --no-daemon
. ~/.nix-profile/etc/profile.d/nix.sh
```

2. Run the flake bootstrap script in this repository:

```
./scripts/flake-bootstrap.sh
```

3. Build and activate the configuration!

```
home-manager switch --flake './#mvertescher'
```
