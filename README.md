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

## Themes

Themes live in `home/themes/` and style the desktop beyond what stylix
does on its own — the bar, launcher, notification daemon, lock screen
and hyprland's own decoration.

- **`cybr`** — Cyberpunk 2077 Neomilitarism: red and cyan on black,
  slanted separators, glow. Selected by importing it; its palette is
  the fixed base16 file in `colors/`.
- **`entropism`** — the salvaged-hardware era: necessity over style.
  Monochrome degraded displays, square corners, 1px lines, no glow.
  Configurable, so it is selected by importing *and* enabling it.

A wrapper picks `entropism` from a host's home-manager modules:

```nix
{
  imports = [ ../../public/home/themes/entropism ];

  themes.entropism = {
    enable = true;

    # burn-in (default) | dead-pixel | salvage-phosphor
    variant = "dead-pixel";

    # Override any semantic role; unset roles keep the variant's value.
    colors.fg = "#c8d0c4";

    # Wallpaper treatment, generated from `bg`:
    # none (default) | scanlines | noise
    texture = "scanlines";

    # Defaults to Rajdhani, the typeface Cyberpunk 2077 sets its own
    # interface in. `pkgs.departure-mono` is the bitmap-adjacent
    # alternative.
    # uiFont = { package = pkgs.departure-mono; name = "Departure Mono"; };
  };
}
```

The theme's API is semantic roles rather than base16 slots — `bg`,
`panel`, `border`, `dim`, `fg`, `alert` and the optional `tape` label
accent (which follows `fg` unless the variant or the host sets it).
Every module downstream reads the resolved roles, so overriding one
role retints stylix, waybar, rofi, swaync and the hyprland borders
together.

The generated base16 scheme is deliberately monochrome: syntax
highlighting collapses onto `fg`/`dim`/`alert` rather than a rainbow.
That is the aesthetic, not a bug.

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
