# GPU profile: nvidia.
#
# The Hyprland session environment an nvidia GPU needs, kept out of the
# shared hyprland module because these three variables are wrong -- not
# merely useless -- on anything else. `LIBVA_DRIVER_NAME=nvidia` points
# VA-API at a driver an AMD or Intel box does not have, and
# `GBM_BACKEND=nvidia-drm` plus `__GLX_VENDOR_LIBRARY_NAME=nvidia` aim the
# GBM and GLX vendor loaders at the same missing implementation, which can
# take the session down rather than just degrade it.
#
# This is a *vendor* axis and is orthogonal to desktop-vs-laptop: a host
# imports one profile from each axis that applies to it. Do not fold the
# two together. The first non-nvidia graphical host adds a sibling here
# rather than a branch in the shared module.
#
# `XDG_SESSION_TYPE,wayland` deliberately stayed behind in
# ../../common/hyprland/default.nix: the session is wayland whoever draws
# it.
{ ... }:

{
  wayland.windowManager.hyprland.settings.env = [
    "LIBVA_DRIVER_NAME,nvidia"
    "GBM_BACKEND,nvidia-drm"
    "__GLX_VENDOR_LIBRARY_NAME,nvidia"
  ];
}
