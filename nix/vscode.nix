{ pkgs, ... }:

{
  programs.vscode = {
    enable = true;


    userSettings = {
      "window.zoomLevel" = -1;

      # Fix `no_std` can't find crate for `test`
      # See: https://github.com/rust-lang/vscode-rust/issues/729
      "rust.target" = "thumbv7em-none-eabihf";
      "rust.all_targets" = false;
      "rust-analyzer.cargo.target" = "thumbv7em-none-eabihf";
      "rust-analyzer.checkOnSave.allTargets" = false;
    };

    extensions = with pkgs.vscode-extensions; [
      bbenoist.nix
      eamodio.gitlens
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
