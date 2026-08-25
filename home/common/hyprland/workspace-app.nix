# Dedicate a workspace to a single always-available application.
#
# Hyprland's `on-created-empty` launches the command the first time you
# switch to that workspace while it is empty. That is a better fit than
# exec-once for a dashboard you want permanently available: it costs
# nothing until you actually look at it, and it comes back if you close
# the window.
#
# Note the workspace number. Hyprland's regular workspaces are
# 1-indexed; workspace 0 does not exist and dispatching to it silently
# succeeds while doing nothing, which is why the 0 key is bound to
# workspace 10 (see ./binds.nix).
{
  lib,
  config,
  ...
}:

let
  cfg = config.custom.workspaceApp;
in
{
  options.custom.workspaceApp = {
    enable = lib.mkEnableOption "an application pinned to its own workspace";

    workspace = lib.mkOption {
      type = lib.types.ints.positive;
      default = 10;
      description = ''
        Which workspace the application owns. Must be 1 or greater:
        hyprland has no workspace 0.
      '';
    };

    command = lib.mkOption {
      type = lib.types.str;
      example = "cp-eras-ui-dashboard";
      description = ''
        Command launched when the workspace is first visited while
        empty. Runs through the compositor, so it inherits a proper
        Wayland environment.
      '';
    };

    apps = lib.mkOption {
      type = lib.types.listOf (
        lib.types.submodule {
          options = {
            workspace = lib.mkOption {
              type = lib.types.ints.positive;
              description = "Which workspace this application owns.";
            };
            command = lib.mkOption {
              type = lib.types.str;
              description = "Command launched on first visit while empty.";
            };
          };
        }
      );
      default = [ ];
      example = lib.literalExpression ''
        [
          {
            workspace = 9;
            command = "alacritty -e claude";
          }
        ]
      '';
      description = ''
        Additional pinned applications. `workspace`/`command` above
        remain for the single-app case, which is how this started;
        anything past the first goes here rather than making every
        consumer write a list.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.command != "";
        message = "custom.workspaceApp.command must be set when the option is enabled";
      }
      {
        assertion = lib.all (a: a.command != "") cfg.apps;
        message = "every custom.workspaceApp.apps entry needs a command";
      }
      {
        # Two entries on one workspace means the second silently never
        # runs, which is a maddening thing to debug at the compositor.
        assertion =
          let
            all = [ cfg.workspace ] ++ map (a: a.workspace) cfg.apps;
          in
          lib.length (lib.unique all) == lib.length all;
        message = "custom.workspaceApp: two applications claim the same workspace";
      }
    ];

    wayland.windowManager.hyprland.settings.workspace =
      [ "${toString cfg.workspace}, on-created-empty:${cfg.command}" ]
      ++ map (a: "${toString a.workspace}, on-created-empty:${a.command}") cfg.apps;
  };
}
