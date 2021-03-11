{ pkgs, ... }:

{
  programs.vim = {
    enable = true;
    settings = {
      background = "dark";
      number = true;
      relativenumber = true;
    };

    plugins = with pkgs.vimPlugins; [
      fugitive
      git-messenger-vim
      nord-vim
      skim
      vim-airline
      vim-airline-themes
      vim-gitgutter
    ];

    extraConfig = "";
  };
}
