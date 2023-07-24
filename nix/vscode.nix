{ pkgs, ... }:

{
  programs.vscode = {
    enable = true;

    userSettings = {
      "[nix]"."editor.tabSize" = 2;
      "files.trimTrailingWhitespace" = true;
      "window.zoomLevel" = -1;

      # "rust-analyzer.diagnostics.disabled" = ["unresolved-proc-macro"];
      # "rust-analyzer.procMacro.attributes.enable" = true;
      # "rust-analyzer.check.extraArgs" = ["--target-dir" "target_ra"];

      # Fix `no_std` can't find crate for `test`
      # See: https://github.com/rust-lang/vscode-rust/issues/729
      # "rust.target" = "thumbv7em-none-eabihf";
      # "rust.all_targets" = false;
      # "rust-analyzer.cargo.target" = "thumbv7em-none-eabihf";
      # "rust-analyzer.checkOnSave.allTargets" = false;
    };

    extensions = with pkgs.vscode-extensions; [
      # bazelbuild.vscode-bazel
      bbenoist.nix
      eamodio.gitlens
      golang.go
      matklad.rust-analyzer
      ms-azuretools.vscode-docker
      ms-python.python
      ms-vscode-remote.remote-ssh
      serayuzgur.crates
      streetsidesoftware.code-spell-checker
      vscodevim.vim
    ] ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "better-toml";
      publisher = "bungcip";
      version = "0.3.2";
      sha256 = "g+LfgjAnSuSj/nSmlPdB0t29kqTmegZB5B1cYzP8kCI=";
    }] ++ pkgs.lib.optionals (!pkgs.stdenv.isDarwin) [
      ms-vscode.cpptools
    ];
  };
}
