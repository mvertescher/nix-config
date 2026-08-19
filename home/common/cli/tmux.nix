{ pkgs, lib, config, ... }:

let
  c = config.lib.stylix.colors or { };
  useStylix = config.stylix.enable or false && c ? base01;

  # Main Palette dynamically derived from Stylix!
  no0 = if useStylix then "#${c.base00}" else "colour232"; # Black
  no1 = if useStylix then "#${c.base01}" else "colour235"; # Dark grey
  no2 = if useStylix then "#${c.base02}" else "colour238"; # Greyish black
  wh0 = if useStylix then "#${c.base06}" else "colour250"; # White
  re0 = if useStylix then "#${c.base08}" else "colour196"; # Red
  gr0 = if useStylix then "#${c.base0B}" else "colour46";  # Green
  cy0 = if useStylix then "#${c.base0C}" else "colour51";  # Cyan

  bg = no1;
  fg = re0;
in
{
  options = {
    programs.tmux.statusBarExtraConfig = lib.mkOption {
      type = lib.types.lines;
      default = ''
        # Status Bar
        # Center status bar window list for clarity
        set -g status-justify centre

        # Set status bar colors
        set-option -g status-bg "${bg}"
        set-option -g status-fg "${fg}"

        # Configure active and inactive window formats with sharp green arrows!
        set-window-option -g window-status-format "#[fg=${wh0},bg=${bg}] #I:#W "
        set-window-option -g window-status-current-format "#[fg=${bg},bg=${gr0}]#[fg=${no0},bg=${gr0},bold] #I:#W #[fg=${gr0},bg=${bg}]"

        # Configure the left status bar (Hostname & IP segment blocks)
        set -g status-left-length 70
        set -g status-left "#[bg=${re0},fg=${no0},bold] #h #[bg=${no2},fg=${re0},nobold]#[bg=${no2},fg=${re0}] #(curl icanhazip.com) #[bg=${bg},fg=${no2}] "

        # Configure the right status bar (Session & Time segment blocks)
        set -g status-right-length 60
        set -g status-right "#[fg=${no2}]#[bg=${no2},fg=${re0}] session: #S #[fg=${re0},bg=${no2}]#[bg=${re0},fg=${no0},bold] %l:%M %p "
      '';
      description = "Extra tmux config for status bar.";
    };
  };

  config = {
    programs.tmux = {
      enable = true;

      shell = "${pkgs.nushell}/bin/nu";
      aggressiveResize = true;
      baseIndex = 1;
      keyMode = "vi";
      shortcut = "a";
      sensibleOnTop = true;
      clock24 = true;
      mouse = false;
      customPaneNavigationAndResize = true;

      plugins = with pkgs; [
        tmuxPlugins.cpu
        tmuxPlugins.yank
      ];

      extraConfig = ''
        # Reload ~/.tmux.conf using PREFIX r
        # bind r source-file ~/.tmux.conf \; display " Reloaded!"
        # Use zsh
        # set -g default-command "${pkgs.zsh}/bin/zsh"
        # Disable mouse support for easy system mouse copy/paste
        # set-option -g mouse on

        # easy-to-remember split pane commands
        bind | split-window -h -c "#{pane_current_path}"
        bind - split-window -v -c "#{pane_current_path}"
        bind c new-window -c "#{pane_current_path}"
        # Vim like copy-mode
        bind-key -T copy-mode-vi 'v' send -X begin-selection
        bind-key -T copy-mode-vi 'y' send -X copy-selection-and-cancel

        ${config.programs.tmux.statusBarExtraConfig}
      '';
    };

    home.activation.reloadTmux = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
      # Check if tmux server is running before trying to reload
      if ${pkgs.tmux}/bin/tmux info &>/dev/null; then
        echo "Reloading tmux configuration for active sessions..."
        ${pkgs.tmux}/bin/tmux source-file ~/.config/tmux/tmux.conf || true
      fi
    '';
  };
}
