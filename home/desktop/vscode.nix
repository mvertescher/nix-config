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
      bazelbuild.vscode-bazel
    ]
    ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "vscode-nushell-lang";
      publisher = "TheNuProjectContributors";
      version = "1.10.0";
      sha256 = "AfClskNZwQIC67VrM8XKxF6nIbXPp576CRmls0WCKwU=";
    }]
    ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "android-system-tools";
      publisher = "flimberger";
      version = "0.0.2";
      sha256 = "9IDMvtfJ/F+Y+vBUG8EjIhofxOZE3zSMRDM3NjtbODg=";
    }]
    ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [{
      name = "selinuxhelper";
      publisher = "SELinuxHelper";
      version = "1.1.1";
      sha256 = "evaYv48qfrhf+Y1pzk+X4Hba/xSiOY0IAv/b8Jq0BH8=";
    }]
    ++ pkgs.lib.optionals (!pkgs.stdenv.isDarwin) [
      ms-vscode.cpptools
    ];
  };
}
