# What a host needs to know about the eras' own screens without
# importing a theme module.
#
# The era's screens -- dashboard, mailbox, mail, store, login -- are
# 1600x900 windows drawn in the traces' frame, and each names itself by
# its binary (`shell::app_id` in cp-eras-ui). A host that gives them a
# workspace sends them there fullscreen, so a screen is looked at the
# way its reference was: alone, edge to edge, over the era's ground
# rather than in a tiled cell beside a shell.
#
# This used to be a window rule written by themes/lib/era.nix from
# `config.custom.workspaceApp`, an option the themes do not own. The
# theme now only says which windows are its screens; the host's
# workspace module decides what to do with them
# (home/common/hyprland/workspace-app.nix, `classes`).
{
  # A Hyprland class regex.
  windowClass = "^(cp-eras-ui-(dashboard|mailbox|mail|store|login))$";
}
