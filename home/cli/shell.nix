{ pkgs, ... }:

let
  commonShellAliases = {
    cat = "bat";
    l = "eza";
    ll = "eza -al";
    s = "stg";
    sse = "stg series";
    tree = "eza -T";

    # Git aliases
    gcdxf = "git clean -dxf; git submodule foreach --recursive git clean -dxf";
    gits = "git status";
    gsu = "git submodule update --init --recursive";

    # Nix aliases
    # hms = "home-manager switch --flake ~/nix-config/#mvertescher@linux";
    hm = "home-manager";
    ndc = "nix develop -c";
    ns = "nix-search";
    rb = "sudo nixos-rebuild switch";
  };
in {
  programs.zsh = {
    enable = true;
    # Set $SHELL appropriately
    envExtra = "export SHELL=${pkgs.zsh}/bin/zsh";
    shellAliases = {
      ".." = "cd ..";

      # Temporary fix to get around OpenGL issues
      # term = "LD_PRELOAD=/lib/x86_64-linux-gnu/libnss_cache.so.2 nixGLIntel alacritty";
      term = "nixGLIntel alacritty";
    };
  };

  programs.nushell = {
    enable = true;

    envFile = { text = ''
    ''; };

    configFile = { text = ''
      $env.config = {
        # filesize_metric: false
        show_banner: false
        # table_mode: rounded
        # use_ls_colors: true
      }
      $env.config.show_banner = false
    ''; };

    shellAliases = commonShellAliases // {
    };
  };

  programs.skim = { enable = true; };

  programs.starship = {
    enable = true;
    enableBashIntegration = false;

    settings = {
      # Don't print a new line at the start of the prompt
      add_newline = false;
      # This can be _very_ slow for larger repos, so disable
      git_status.disabled = true;
    };
  };
}
