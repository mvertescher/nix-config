{ pkgs, lib, ... }:

{
  programs.git = {
    package = pkgs.gitFull;
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

      # New repositories (including submodules cloned into worktrees) use
      # the files ref backend: git 2.55 defaults to reftable, which nix's
      # libgit2 cannot read, so git+file flakes fail on such checkouts.
      init.defaultRefFormat = "files";
    };

    lfs.enable = true;
  };

  home.packages = with pkgs; [ stgit ];
}
