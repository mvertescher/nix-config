# Xcode from the Nix store, set up with no GUI and no prompts.
#
# Opt-in per host (`custom.xcode.enable`). The package is one of
# nixpkgs' `requireFile` Xcodes: nix pins the unpacked Xcode.app by hash
# but cannot fetch it, because Apple serves the .xip only to a signed-in
# developer account. So the one manual step is getting the .xip into the
# store once (the wrapper's script does that); everything after it is
# here and runs on every deploy and every boot, idempotently:
#
#   activation (fast, root): /Applications/Xcode.app -> the store path,
#     and xcode-select pointed at it.
#   launchd daemon org.nixos.xcode-setup (slow, root, after activation):
#     license accepted, first-launch packages installed, the listed
#     simulator platforms downloaded, developer mode on so debugging
#     does not raise an authorization dialog.
#
# The slow half is a daemon rather than activation because a deploy's
# activation has a timeout (deploy-rs: 240 s) and first launch plus a
# multi-GB platform download would blow it, and a timed-out activation is
# rolled back. Its log is /var/log/xcode-setup.log.
#
# Enabling this accepts the Xcode and Apple SDKs license agreement on the
# machine's behalf, which is what `xcodebuild -license accept` does.

{ config, lib, pkgs, ... }:

let
  cfg = config.custom.xcode;
  app = "/Applications/Xcode.app";
  developer = "${app}/Contents/Developer";

  setup = pkgs.writeShellScript "xcode-setup" ''
    set -u
    export PATH=/usr/bin:/bin:/usr/sbin:/sbin
    # The store path, so a new Xcode changes this script, which changes
    # the daemon's plist, which makes activation reload it and rerun.
    echo "== $(date) xcode-setup for ${cfg.package}"

    if [ "$(xcode-select -p 2>/dev/null)" != "${developer}" ]; then
      echo "xcode-select is not ${developer}; activation has not linked Xcode yet"
      exit 1
    fi

    xcodebuild -license check >/dev/null 2>&1 || xcodebuild -license accept
    xcodebuild -checkFirstLaunchStatus || xcodebuild -runFirstLaunch

    ${lib.concatMapStrings (p: ''
      if xcrun simctl list runtimes 2>/dev/null | grep -q '^${p} '; then
        echo "${p} platform already installed"
      else
        xcodebuild -downloadPlatform ${p}
      fi
    '') cfg.platforms}

    DevToolsSecurity -enable
    dseditgroup -o checkmember -m ${config.system.primaryUser} _developer >/dev/null 2>&1 \
      || dseditgroup -o edit -a ${config.system.primaryUser} -t user _developer
    echo "== done"
  '';
in
{
  options.custom.xcode = {
    enable = lib.mkEnableOption "Xcode from the Nix store, set up unattended";

    package = lib.mkOption {
      type = lib.types.package;
      example = lib.literalExpression ''
        pkgs.darwin.xcode_26_6_Apple_silicon
        # or a version nixpkgs does not list yet, hash from the add script:
        pkgs.darwin.requireXcode "27.0_Apple_silicon" "sha256-..."
      '';
      description = ''
        The Xcode.app to use: a nixpkgs requireFile Xcode. Building it
        fails until its .xip has been added to the store, so enable this
        only once that is done.
      '';
    };

    platforms = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ "iOS" ];
      description = "Simulator platforms to download (`xcodebuild -downloadPlatform`).";
    };
  };

  config = lib.mkIf cfg.enable {
    system.activationScripts.postActivation.text = ''
      echo "setting up Xcode..." >&2
      if [ -e ${app} ] && [ ! -L ${app} ]; then
        echo "warning: ${app} is a real directory, not ours; leaving it and xcode-select alone" >&2
      else
        ln -sfn ${cfg.package} ${app}
        if [ "$(/usr/bin/xcode-select -p 2>/dev/null)" != "${developer}" ]; then
          /usr/bin/xcode-select -s ${developer}
        fi
      fi
    '';

    launchd.daemons.xcode-setup.serviceConfig = {
      Label = "org.nixos.xcode-setup";
      ProgramArguments = [ "${setup}" ];
      RunAtLoad = true;
      StandardOutPath = "/var/log/xcode-setup.log";
      StandardErrorPath = "/var/log/xcode-setup.log";
    };
  };
}
