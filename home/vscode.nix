{ pkgs, ... }:

{
  programs.vscode = {
    enable = true;

    profiles.default.userSettings = {
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

    profiles.default.extensions = with pkgs.vscode-extensions; [
      eamodio.gitlens
      golang.go
      jnoortheen.nix-ide
      ms-azuretools.vscode-docker
      ms-vscode-remote.remote-ssh
      rust-lang.rust-analyzer
      streetsidesoftware.code-spell-checker
      tamasfe.even-better-toml
      vscodevim.vim

      # ms-python.python
      # serayuzgur.crates
    ]
    ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "vscode-nushell-lang";
      publisher = "TheNuProjectContributors";
      version = "1.10.0";
      sha256 = "AfClskNZwQIC67VrM8XKxF6nIbXPp576CRmls0WCKwU=";
    }]
    # ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
    #  name = "better-toml";
    #   publisher = "bungcip";
    #   version = "0.3.2";
    #   sha256 = "g+LfgjAnSuSj/nSmlPdB0t29kqTmegZB5B1cYzP8kCI=";
    # }]
    ++ pkgs.lib.optionals (!pkgs.stdenv.isDarwin) [
      ms-vscode.cpptools
    ];
  };
}
