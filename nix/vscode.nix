{ pkgs, ... }:

{
  programs.vscode = {
    enable = true;
    extensions = with pkgs.vscode-extensions; [
      bbenoist.Nix
      matklad.rust-analyzer
      ms-azuretools.vscode-docker
      ms-python.python
      ms-vscode.cpptools
      vscodevim.vim
    ];
  };
}
