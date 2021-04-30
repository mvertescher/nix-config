{ pkgs, ... }:

{
  programs.zsh = {
    enable = true;
    shellAliases = {
      ".." = "cd ..";
      c = "bat";
      hms = "home-manager switch";
      l = "exa";
      ll = "exa -al";
      rb = "sudo nixos-rebuild switch";
      s = "stg";

      # Git aliases
      gcdxf = "git clean -dxf && git submodule foreach --recursive git clean -dxf";
      gits = "git status";
      gsu = "git submodule update --init --recursive";

      # Temporary fix to get around OpenGL issues
      term = "LD_PRELOAD=/lib/x86_64-linux-gnu/libnss_cache.so.2 nixGLIntel alacritty";
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
