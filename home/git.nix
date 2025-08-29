{ pkgs, ... }:

{
  programs.git = {
    package = pkgs.gitAndTools.gitFull;
    enable = true;

    userName = "Matt Vertescher";
    userEmail = "mvertescher@gmail.com";

    lfs.enable = true;

    extraConfig = {
      core = {
        editor = "vim";
        whitespace = "trailing-space,space-before-tab";
      };

      # Fast forward only
      pull.ff = "only";
    };
  };

  home.packages = with pkgs; [ gitAndTools.stgit ];
}
