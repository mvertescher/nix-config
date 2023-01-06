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

2. Make sure your local Nix configuration includes the following features:

```
$ cat ~/.config/nix/nix.conf
experimental-features = nix-command flakes
```

3. Run the flake bootstrap script in this repository:

```
./scripts/flake-bootstrap.sh
```

4. Build and activate the configuration!

```
home-manager switch --flake './#mvertescher'
```

## Useful resources

- [Misterio's Nix config](https://git.sr.ht/~misterio/nix-config)
