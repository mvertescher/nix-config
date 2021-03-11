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

    extraConfig = ''
        set colorcolumn=80
        set cursorline
        set spell
        set list

        colorscheme nord

        set hlsearch
        highlight ExtraWhitespace ctermbg=darkgreen guibg=darkgreen

        " Tab width to two spaces
        set tabstop=2 shiftwidth=2 expandtab

        " Remove trailing whitespace on write
        " autocmd BufWritePre * :%s/\s\+$//e

        " Map control-p to skim in normal mode
        map <C-p> :SK<CR>

        " Open GitMessenger commit popup
        map <C-w>m :GitMessenger<CR>
    '';
  };
}
