{ pkgs, lib, ... }:

{
  programs.git = {
    # package = pkgs.gitAndTools.gitFull;
    enable = true;

    settings = {
      user = {
        name = "Matt Vertescher";
        email = lib.mkDefault "mvertescher@gmail.com";
      };

      core = {
        editor = "vim";
        whitespace = "trailing-space,space-before-tab";
      };

      # Fast forward only
      pull.ff = "only";
    };

    lfs.enable = true;
  };

  home.packages = with pkgs; [ stgit ];
}
