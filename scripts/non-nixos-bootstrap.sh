#!/usr/bin/env sh

set -ex

# Setup channel
nix-channel --add https://github.com/nix-community/home-manager/archive/release-22.05.tar.gz home-manager
# nix-channel --add https://github.com/nix-community/home-manager/archive/master.tar.gz home-manager

# for now, use a patched branch to install using Nix 2.4
# nix-channel --add https://github.com/moinessim/home-manager/archive/nix-2.4-nix-darwin-modules.tar.gz home-manager
nix-channel --update

# Make sure NIX_PATH is set
export NIX_PATH=$HOME/.nix-defexpr/channels:/nix/var/nix/profiles/per-user/root/channels${NIX_PATH:+:$NIX_PATH}

# Install home-manager
nix-shell '<home-manager>' -A install

# Symlink this repo to the user config directory
rm -rf ~/.config/nixpkgs/home.nix || true
ln -s $PWD/nix/home.nix ~/.config/nixpkgs/home.nix
