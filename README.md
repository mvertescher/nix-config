# nix-config

> My Nix/NixOS configuration, structured as a library.

This repo holds shared configuration — home-manager modules (`home/`),
NixOS modules (`system/`), nix-darwin modules (`darwin/`) — and the
builders that assemble them into configurations. It defines no hosts.
It is consumed by private wrapper flakes that layer machine-specific
and private config on top; the wrappers depend on this repo, never the
other way around.

## What is here

| Path | What |
| --- | --- |
| `lib/` | The builders `mkNixos`, `mkHome` and `mkDarwin`, the overlays, the shared SSH key list, and the builder API test. |
| `system/` | The NixOS base every host gets (`configuration.nix`), plus opt-in modules: Hyprland and its greeter (`wm/hyprland.nix`), fonts, 1Password, GitHub-sourced authorized keys, and the installer ISO. |
| `darwin/` | The macOS base (`configuration.nix`), plus opt-in modules: `mac-mini.nix`, `xcode.nix`, `ios-signing.nix`. |
| `home/common/` | Home-manager modules: `cli` (git, shell, tmux, vim, helix, ...), `gui` (alacritty, firefox, vscode, ...), `hyprland`. |
| `home/profiles/` | Opt-in host profiles: `desktop.nix` or `laptop.nix` (idle and suspend behaviour), `gpu/nvidia.nix`. |
| `home/themes/` | The `cybr` theme (see below). |

The overlays add a few packages to `pkgs` for every host: the coding
agents (`claude-code`, `codex`, `grok`, `antigravity-cli`) from
llm-agents.nix, `nixgl`, the rust-overlay toolchains, `craneLib`, and
this repo's own `mpris-status`.

## Integrating from a wrapper flake

Add this repo as a git submodule and consume it as a `git+file:` flake
input:

```sh
git submodule add https://github.com/mvertescher/nix-config.git public
```

```nix
# flake.nix
{
  inputs.nix-config.url = "git+file:./public";

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
            # extraOverlays = [ ];               # optional, this host only
            modules = [ ./hosts/myhost/home.nix ];
          };
        };
        # extraOverlays = [ ];                   # optional, all hosts
      };

      # NixOS: this repo defines no hosts — the wrapper owns machine
      # identity (hardware config, disks, per-host modules) and this
      # library wraps each host with the shared stack (base system
      # config, home-manager, stylix).
      nixosConfigurations = nix-config.lib.mkNixos {
        hosts = {
          myhost = {
            # system = "aarch64-linux";          # optional, per host
            # user = "mverte";                   # optional (the default)
            # extraOverlays = [ ];               # optional, this host only
            modules = [ ./hosts/myhost ];        # incl. hardware-configuration.nix
            homeModules = [ ./hosts/myhost/home.nix ];
          };
        };
        extraSystemConfig = ./system;            # optional, all hosts
        # extraOverlays = [ ];                   # optional, all hosts
      };

      # macOS via nix-darwin. Same shape; the shared stack is
      # darwin/configuration.nix. Machine kinds such as
      # darwin/mac-mini.nix are opt-in from the host's modules. `user`
      # must be the account macOS Setup Assistant already created.
      # Takes extraSystemConfig and extraOverlays like mkNixos.
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
- **`extraOverlays`** adds packages this repo does not carry. All three
  builders take it for the whole host set, and each host can add its
  own on top.
- **Default users differ on purpose**: `mkNixos` and `mkDarwin` default
  `user` to `mverte`, `mkHome` to `mvertescher`. Existing consumers
  depend on each, so neither changes.
- **This repo reaches a build by two routes, and they see different
  trees.** Modules a host imports by path (`public/home/...`) come from
  the wrapper's own source, so they are the submodule's working tree:
  edits to tracked files apply on the next rebuild without committing,
  but a new file is invisible until it is at least `git add -N`'d.
  Everything that comes through the flake input (the builders, `pkgs`
  and the overlays, the base system config, the nixpkgs pin) is the
  revision in the wrapper's `flake.lock` and nothing else.
- **Bumping the pin is therefore two steps**: commit and push in
  `public/`, then in the wrapper run `nix flake update nix-config` and
  commit the submodule pointer and `flake.lock` together. A pointer bump
  without the lock update deploys none of the second route.
- **Use `git+file:`, not `path:`**, for the input. `path:./public` has
  failed to bring the submodule into the fetched flake source and
  resolved stale pins without an error.
- Nixpkgs and friends are pinned by this repo's `flake.lock`; wrappers
  inherit those pins through the input.

## Themes

One theme lives here, in `home/themes/cybr`: red and cyan on black,
slanted separators, glow. It styles the desktop beyond what stylix
does on its own (the bar,
launcher, notification daemon, lock screen and hyprland's own
decoration), ships upstream cybrcore assets and a fixed 33-colour
palette in `colors/`, and is selected by importing it:

```nix
{
  imports = [ ../../public/home/themes/cybr ];
}
```

The four generated era themes and the `cp-eras-ui` crate that draws
their screens were here until November 2026. They moved to a repository of their own, which a
wrapper adds as a flake input alongside this one. What stays here for
them is the shared Hyprland module, whose look is all `mkDefault` so a
theme can set plain values over it, and the `custom.greetd` option for
the era login screen (`custom.greetd.greeter = "cp-eras-ui"`), which
expects the wrapper to supply `pkgs.cp-eras-ui` through `extraOverlays`
and fails evaluation with a message saying so if it is missing.

## Checks and the installer ISO

```sh
nix build .#checks.x86_64-linux.builder-api   # the builders still accept a minimal call
nix build .#installer-iso                     # NixOS installer with the shared SSH keys baked in
```

`builder-api` is the only consumer of the builders inside this repo,
so it is what catches an API break before a wrapper's next pin bump.
Changes to the builders must be additive: a new optional argument is
fine, a new required one or a changed default is not. Both run in CI
(`.github/workflows/`), the ISO only when its inputs change.

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
