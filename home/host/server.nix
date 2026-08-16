# server specific home manager configuration
#
# Headless host: imports CLI tooling and stylix terminal theming only,
# skipping common/home.nix's GUI packages and cybr's Hyprland/Wayland
# programs (firefox, hyprlock, waybar, swaync), which don't apply here.

{ pkgs, config, lib, ... }:

let
  theme = import ../themes/cybr/colors/cybrcolors.nix;
in
{
  imports = [
    ../common/cli
    ../themes/cybr/starship.nix
  ];

  programs.home-manager.enable = true;

  # notifications about home-manager news
  news.display = "silent";

  stylix = {
    enable = true;
    base16Scheme = theme;
  };

  programs.vivid.enable = true;
  stylix.targets.vivid.enable = true;

  # Clone this repo into ~/nix-config on first activation, so a freshly
  # provisioned host is immediately ready for `./switch server`. Clones
  # over HTTPS (no credentials needed on a new box) but pushes over SSH.
  # Skipped silently if offline or the directory already exists.
  home.activation.cloneNixConfig = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    if [ ! -e "$HOME/nix-config" ]; then
      run ${pkgs.git}/bin/git clone https://github.com/mvertescher/nix-config.git "$HOME/nix-config" \
        && run ${pkgs.git}/bin/git -C "$HOME/nix-config" remote set-url --push origin git@github.com:mvertescher/nix-config.git \
        || verboseEcho "nix-config clone failed (offline?), skipping"
    fi
  '';

  home.stateVersion = "25.05";
}
