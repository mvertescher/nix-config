{ pkgs, ... }:

{
  programs.zsh = {
    enable = true;
    shellAliases = {
      ".." = "cd ..";
      c = "bat";
      gits = "git status";
      hms = "home-manager switch";
      l = "exa";
      ll = "exa -al";
      rb = "sudo nixos-rebuild switch";
      s = "stg";
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
