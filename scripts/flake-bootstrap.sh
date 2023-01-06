#!/usr/bin/env sh

set -ex

# See https://nix-community.github.io/home-manager/index.html#ch-nix-flakes for more info

UNAMESTR=$(uname)
if [[ "$UNAMESTR" == "Linux" ]]; then
CONFIG="mvertescher@linux"
elif [[ "$UNAMESTR" == "Darwin" ]]; then
CONFIG="mvertescher@macos"
else
exit 1
fi

# Install home-manager and apply the configuration
# Use "nix --show-trace ..." to see errors
nix --show-trace build --no-link ./#homeConfigurations.${CONFIG}.activationPackage
"$(nix path-info ./#homeConfigurations.${CONFIG}.activationPackage)"/activate

