# entropism prompt: a line of text, not a dashboard.
#
# cybr's starship is powerline segments in six colours; this is the
# opposite. No separators, no glyph chevrons, no per-language icons --
# just who, where, and whether the last thing failed, in fg/dim/alert.
{
  config,
  lib,
  ...
}:

let
  cfg = config.themes.entropism;
  c = cfg.resolvedColors;
in
{
  config = lib.mkIf cfg.enable {
    programs.starship = {
      enable = true;

      settings = {
        scan_timeout = 100;
        add_newline = false;

        format = lib.concatStrings [
          "$username"
          "$hostname"
          "$directory"
          "$git_branch"
          "$git_status"
          "$cmd_duration"
          "$line_break"
          "$character"
        ];

        username = {
          show_always = true;
          style_user = "fg:${c.dim}";
          style_root = "fg:${c.alert}";
          format = "[$user]($style)";
        };

        # The one place the tape accent earns its keep: which machine you
        # are actually typing into.
        hostname = {
          ssh_only = false;
          style = "fg:${c.tape}";
          format = "[@$hostname]($style) ";
        };

        directory = {
          style = "fg:${c.fg}";
          format = "[$path]($style)[$read_only]($read_only_style) ";
          read_only = " ro";
          read_only_style = "fg:${c.alert}";
          truncation_length = 4;
          truncate_to_repo = false;
        };

        git_branch = {
          style = "fg:${c.dim}";
          format = "[$branch]($style) ";
          symbol = "";
        };

        git_status = {
          style = "fg:${c.alert}";
          format = "[$all_status$ahead_behind]($style) ";
          conflicted = "!";
          ahead = ">";
          behind = "<";
          diverged = "<>";
          untracked = "?";
          stashed = "$";
          modified = "*";
          staged = "+";
          renamed = "»";
          deleted = "x";
        };

        cmd_duration = {
          min_time = 2000;
          style = "fg:${c.dim}";
          format = "[$duration]($style) ";
        };

        # Failure is the only state that gets a colour of its own.
        character = {
          success_symbol = "[>](fg:${c.fg})";
          error_symbol = "[>](fg:${c.alert})";
          vimcmd_symbol = "[<](fg:${c.dim})";
        };

        # Everything else stays out of the way. Language/tool modules are
        # noise on a salvaged terminal.
        package.disabled = true;
        nodejs.disabled = true;
        rust.disabled = true;
        python.disabled = true;
        golang.disabled = true;
        java.disabled = true;
        docker_context.disabled = true;
        nix_shell = {
          disabled = false;
          style = "fg:${c.dim}";
          format = "[nix]($style) ";
          symbol = "";
        };
      };
    };
  };
}
