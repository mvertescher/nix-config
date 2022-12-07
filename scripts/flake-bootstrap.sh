#!/usr/bin/env sh

set -ex

# See https://nix-community.github.io/home-manager/index.html#ch-nix-flakes for more info

# Install home-manager and apply the configuration
# Use "nix --show-trace ..." to see errors
nix --show-trace build --no-link ./#homeConfigurations.mvertescher.activationPackage
"$(nix path-info ./#homeConfigurations.mvertescher.activationPackage)"/activate

