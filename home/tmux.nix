{ pkgs, ... }:

{
    programs.tmux = {
        enable = true;

        shell = "${pkgs.nushell}/bin/nu";
        aggressiveResize = true;
        baseIndex = 1;
        keyMode = "vi";
        shortcut = "a";
        sensibleOnTop = true;

        plugins = with pkgs; [
            tmuxPlugins.cpu
            tmuxPlugins.yank
        ];

        extraConfig = ''
            # Reload ~/.tmux.conf using PREFIX r
            bind r source-file ~/.tmux.conf \; display " Reloaded!"
            # Use zsh
            # set -g default-command "${pkgs.zsh}/bin/zsh"
            # Disable mouse support for easy system mouse copy/paste
            # set-option -g mouse on

            # easy-to-remember split pane commands
            bind | split-window -h -c "#{pane_current_path}"
            bind - split-window -v -c "#{pane_current_path}"
            bind c new-window -c "#{pane_current_path}"
            # Moving between panes with vim movement keys
            bind h select-pane -L
            bind j select-pane -D
            bind k select-pane -U
            bind l select-pane -R
            # Vim like copy-mode
            bind-key -T copy-mode-vi 'v' send -X begin-selection
            bind-key -T copy-mode-vi 'y' send -X copy-selection-and-cancel
            # Status Bar
            # Center status bar window list for clarity
            set -g status-justify centre

            # Set status bar colors
            set-option -g status-bg colour235 # base02
            set-option -g status-fg yellow # yellow
            # Configure the left status bar
            set -g status-left-length 70
            set -g status-left "#[fg=colour153]: #h : #[fg=colour123]#(curl icanhazip.com) #[fg=colour220]#(ifconfig en0 | grep 'inet ' | awk '{print \"en0 \" $2}') #(ifconfig en1 | grep 'inet ' | awk '{print \"en1 \" $2}') #[fg=red]#(ifconfig tun0 | grep 'inet ' | awk '{print \"vpn \" $2}') "
            # Configure the right status bar
            # show session name, window & pane number, date and time
            set -g status-right-length 60
            set -g status-right "#[fg=colour250]#S #I:#P #[fg=colour87]:: %l:%M %p ::"
        '';
    };
}
