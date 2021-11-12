#!/usr/bin/env sh

set -ex

# Setup channel
# nix-channel --add https://github.com/nix-community/home-manager/archive/release-21.05.tar.gz home-manager
# nix-channel --add https://github.com/nix-community/home-manager/archive/master.tar.gz home-manager

# for now, use a patched branch to install using Nix 2.4
nix-channel --add https://github.com/moinessim/home-manager/archive/nix-2.4-nix-darwin-modules.tar.gz home-manager
nix-channel --update

# Install home-manager
nix-shell '<home-manager>' -A install

# Symlink this repo to the user config directory
rm -rf ~/.config/nixpkgs/home.nix
ln -s $PWD/nix/home.nix ~/.config/nixpkgs/home.nix
