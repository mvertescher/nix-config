# nix-config

> My Nix/NixOS configuration, structured as a library.

This repo holds shared configuration — home-manager modules
(`home/common`, `home/themes`), NixOS modules (`system/`), and the
builders that assemble them into configurations. It is consumed by
private wrapper flakes that layer machine-specific and private config
on top; the wrappers depend on this repo, never the other way around.

## Integrating from a wrapper flake

Add this repo as a git submodule and consume it as a `path:` flake
input:

```sh
git submodule add https://github.com/mvertescher/nix-config.git public
```

```nix
# flake.nix
{
  inputs.nix-config.url = "path:./public";

  outputs = { self, nix-config, ... }:
    {
      # Home-manager only (e.g. running Nix on a non-NixOS host):
      homeConfigurations = nix-config.out.pkgs.builders.mkHome {
        extraHomeConfig = ./home;
      };

      # NixOS: this repo defines no hosts — the wrapper owns machine
      # identity (hardware config, disks, per-host modules) and this
      # library wraps each host with the shared stack (base system
      # config, home-manager, stylix).
      nixosConfigurations = nix-config.lib.mkNixos {
        hosts = {
          myhost = {
            # system = "aarch64-linux";          # optional, per host
            modules = [ ./hosts/myhost ];        # incl. hardware-configuration.nix
            homeModules = [ ./hosts/myhost/home.nix ];
          };
        };
        extraSystemConfig = ./system;            # optional, all hosts
      };
    };
}
```

Notes:

- **`?submodules=1` is required** on flake refs so Nix can see the
  submodule: `home-manager switch --flake '.?submodules=1#<host>'`,
  `nixos-rebuild switch --flake '.?submodules=1#<host>'`.
- **`extraHomeConfig` discovery** (`mkHome`): pass a path; any file or
  directory under `<path>/host/` whose name matches a host (exactly, or
  as a suffix — `foo-laptop.nix` matches `laptop`) is merged into that
  host's home configuration. See `lib/private-config.nix`.
- **`mkNixos` host layout convention**: keep each host under
  `hosts/<name>/` with its `hardware-configuration.nix` — that's where
  `scripts/provision-server.sh` writes the generated one.
- Shared modules are importable from the wrapper by path, e.g.
  `public/system/wm/hyprland.nix` or `public/home/common/cli`, from a
  host's own modules.
- The `path:` input evaluates the submodule's working tree as-is, so
  local edits here apply on the next rebuild without committing. Bump
  the submodule pin like any other: commit in `public/`, then
  `git add public` in the wrapper.
- Nixpkgs and friends are pinned by this repo's `flake.lock`; wrappers
  inherit those pins through the input.

## Bootstrapping on non-NixOS

For a machine that runs Nix on another distro (home-manager only):

1. Install Nix and enable flakes:

   ```sh
   sh <(curl -L https://nixos.org/nix/install) --no-daemon
   . ~/.nix-profile/etc/profile.d/nix.sh
   mkdir -p ~/.config/nix
   echo 'experimental-features = nix-command flakes' >> ~/.config/nix/nix.conf
   ```

2. Clone the wrapper repo with submodules and activate:

   ```sh
   git clone --recurse-submodules <wrapper-repo> && cd <wrapper-repo>
   nix run home-manager -- switch --flake '.?submodules=1#<host>'
   ```

GL applications on non-NixOS may need the bundled nixGL overlay
(`pkgs.nixgl`) to find drivers.

## Useful resources

- [Misterio's Nix config](https://git.sr.ht/~misterio/nix-config)
- [gvolpe's Nix config](https://github.com/gvolpe/nix-config)
