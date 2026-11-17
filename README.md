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
      # Home-manager only (e.g. running Nix on a non-NixOS host). The
      # wrapper owns machine identity — username, home directory,
      # monitors — in its host module, which imports this repo's shared
      # home/ modules by path:
      homeConfigurations = nix-config.lib.mkHome {
        hosts = {
          myhost = {
            # user = "mvertescher";              # optional (the default)
            # system = "aarch64-linux";          # optional, per host
            modules = [ ./hosts/myhost/home.nix ];
          };
        };
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

      # macOS via nix-darwin. Same shape; the shared stack is
      # darwin/configuration.nix. Machine kinds such as
      # darwin/mac-mini.nix are opt-in from the host's modules. `user`
      # must be the account macOS Setup Assistant already created.
      darwinConfigurations = nix-config.lib.mkDarwin {
        hosts = {
          mymac = {
            # system = "x86_64-darwin";          # optional; default aarch64-darwin
            user = "me";
            modules = [ ./hosts/mymac ];         # e.g. imports public/darwin/mac-mini.nix
            homeModules = [ ./hosts/mymac/home.nix ];
          };
        };
      };
    };
}
```

Notes:

- **`?submodules=1` is required** on flake refs so Nix can see the
  submodule: `home-manager switch --flake '.?submodules=1#<host>'`,
  `nixos-rebuild switch --flake '.?submodules=1#<host>'`.
- **Identity defaults** (`mkHome`): `home.username` and
  `home.homeDirectory` are defaulted from the host's `user` at
  `mkDefault` priority — home directories vary per machine, so a host
  module's plain definition overrides without `mkForce`.
- **Host layout convention**: keep each host under
  `hosts/<name>/` with its `hardware-configuration.nix` — that's where
  a wrapper's provisioning tool writes the generated one.
- Shared modules are importable from the wrapper by path, e.g.
  `public/system/wm/hyprland.nix` or `public/home/common/cli`, from a
  host's own modules.
- The `path:` input evaluates the submodule's working tree as-is, so
  local edits here apply on the next rebuild without committing. Bump
  the submodule pin like any other: commit in `public/`, then
  `git add public` in the wrapper.
- Nixpkgs and friends are pinned by this repo's `flake.lock`; wrappers
  inherit those pins through the input.

## Themes

One theme lives here, in `home/themes/cybr`: Cyberpunk 2077
Neomilitarism, red and cyan on black, slanted separators, glow. It
styles the desktop beyond what stylix does on its own (the bar,
launcher, notification daemon, lock screen and hyprland's own
decoration), ships upstream cybrcore assets and a fixed 33-colour
palette in `colors/`, and is selected by importing it:

```nix
{
  imports = [ ../../public/home/themes/cybr ];
}
```

The generated Cyberpunk 2077 era themes (entropism, kitsch, neo-kitsch,
neomil) and the `cp-eras-ui` crate that draws their screens were here
until November 2026. They moved to a repository of their own, which a
wrapper adds as a flake input alongside this one. What stays here for
them is the shared Hyprland module, whose look is all `mkDefault` so a
theme can set plain values over it, and the `custom.greetd` option for
the era login screen, which expects the wrapper to supply
`pkgs.cp-eras-ui` through an overlay.

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
