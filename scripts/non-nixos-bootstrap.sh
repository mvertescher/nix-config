#!/usr/bin/env sh

set -ex

# Setup channel
nix-channel --add https://github.com/nix-community/home-manager/archive/release-20.09.tar.gz home-manager
nix-channel --update

# Install home-manager
nix-shell '<home-manager>' -A install

# Symlink this repo to the user config directory
rm -rf ~/.config/nixpkgs/home.nix
ln -s $PWD/nix/home.nix ~/.config/nixpkgs/home.nix
