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
      serayuzgur.crates
      vscodevim.vim
    ] ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "better-toml";
      publisher = "bungcip";
      version = "0.3.2";
      sha256 = "g+LfgjAnSuSj/nSmlPdB0t29kqTmegZB5B1cYzP8kCI=";
    }];
  };
}
