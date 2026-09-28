# Code signing for iOS (and macOS) builds on a headless Mac, for any
# project on one Apple team.
#
# What is per team, and so configured here once: the distribution
# identity (a base64 .p12 and its password) and an App Store Connect API
# key. What is per app -- provisioning profiles -- is fetched with that
# key at build time (`xcodebuild -allowProvisioningUpdates` with the
# auth flags below, or a project's own script), so nothing here names an
# app. The options take *file paths*, so the secrets can come from
# sops-nix, agenix or anything that renders files; this module never
# sees their contents. The identity is stored base64 (one text secret,
# easy to keep in a YAML secrets file); the module decodes it to a raw
# .p12 at identityP12Path (default /run/ios-signing/identity.p12) on
# every activation and at boot, for tools that want a file.
#
# The keychain is the macOS-specific part. The login keychain stays
# locked on a Mac nobody has logged into at the screen, so builds do not
# use it. Instead:
#
#   with-signing-keychain <command> [args...]
#
# creates a throwaway keychain with a random password, imports the
# identity with codesign/productbuild allowed to use it without a
# prompt, puts it first in the user's search list, exports the API key
# as ASC_KEY_ID / ASC_ISSUER_ID / ASC_KEY_PATH plus XCODEBUILD_AUTH
# (the three -authenticationKey* flags and -allowProvisioningUpdates),
# runs the command, and deletes the keychain however the command exits.
# The same shape as a CI job, so a local build and a GitHub Actions
# build sign the same way, and no key sits in a keychain between builds.
#
#   with-signing-keychain sh -c 'xcodebuild ... archive $XCODEBUILD_AUTH'

{ config, lib, pkgs, ... }:

let
  cfg = config.custom.iosSigning;

  # Apple's WWDR G3 intermediate, which issues today's Apple Development
  # and Apple Distribution certificates. macOS ships Apple Root CA but
  # not necessarily this intermediate -- malum had only the expired G1 --
  # and without it the identity imports yet `find-identity -v` reports
  # 0 valid identities and codesign cannot build a chain. Pinned by hash;
  # valid until 2030-02-20.
  wwdrG3 = pkgs.fetchurl {
    url = "https://www.apple.com/certificateauthority/AppleWWDRCAG3.cer";
    hash = "sha256-3PIYeMd/QZjktGFPA9aW2JxmxmAI1CROG5kWGqyRYB8=";
  };

  # The raw .p12, decoded from the base64 secret. Tools that want a .p12
  # file (loom's release script, `security import`) read this; nothing
  # else has to know the secret is stored as base64. /run is cleared at
  # boot, so it is re-derived then too (see the daemon below).
  rawDir = builtins.dirOf cfg.identityP12Path;
  decodeIdentity = pkgs.writeShellScript "ios-signing-identity" ''
    set -eu
    export PATH=/usr/bin:/bin:/usr/sbin:/sbin
    src=${lib.escapeShellArg cfg.identityP12Base64File}
    # At boot this races sops-install-secrets: wait for the secret.
    for _ in $(seq 1 60); do [ -r "$src" ] && break; sleep 2; done
    [ -r "$src" ] || { echo "ios-signing: $src never appeared" >&2; exit 1; }
    install -d -m 0700 -o ${cfg.user} ${lib.escapeShellArg rawDir}
    tmp=$(mktemp ${lib.escapeShellArg rawDir}/.identity.XXXXXX)
    base64 -d < "$src" > "$tmp"
    chown ${cfg.user} "$tmp"; chmod 0400 "$tmp"
    mv -f "$tmp" ${lib.escapeShellArg cfg.identityP12Path}
  '';

  helper = pkgs.writeShellScriptBin "with-signing-keychain" ''
    set -euo pipefail
    export PATH=/usr/bin:/bin:/usr/sbin:/sbin:$PATH

    if [ $# -eq 0 ]; then
      echo "usage: with-signing-keychain <command> [args...]" >&2
      exit 2
    fi
    for f in ${lib.escapeShellArgs [ cfg.identityP12Path cfg.identityPasswordFile cfg.ascKeyIdFile cfg.ascIssuerIdFile cfg.ascKeyP8File ]}; do
      [ -r "$f" ] || { echo "with-signing-keychain: cannot read $f (secret not deployed?)" >&2; exit 1; }
    done

    work=$(mktemp -d "''${TMPDIR:-/tmp}/signing.XXXXXX")
    keychain="$work/signing.keychain-db"
    kc_pass=$(openssl rand -hex 24)
    original=$(security list-keychains -d user | tr -d '"' | xargs)

    cleanup() {
      # shellcheck disable=SC2086
      security list-keychains -d user -s $original >/dev/null 2>&1 || true
      security delete-keychain "$keychain" >/dev/null 2>&1 || true
      rm -rf "$work"
    }
    trap cleanup EXIT INT TERM

    security create-keychain -p "$kc_pass" "$keychain"
    # No flags: no lock on sleep and no lock after a timeout, so a long
    # archive cannot lose the key halfway. It is deleted on exit anyway.
    security set-keychain-settings "$keychain"
    security unlock-keychain -p "$kc_pass" "$keychain"

    security import ${lib.escapeShellArg cfg.identityP12Path} -k "$keychain" \
      -P "$(cat ${lib.escapeShellArg cfg.identityPasswordFile})" -f pkcs12 \
      -T /usr/bin/codesign -T /usr/bin/productbuild -T /usr/bin/security >/dev/null
    security import ${wwdrG3} -k "$keychain" -t cert -f x509 >/dev/null
    # Without this partition list, codesign raises a GUI authorization
    # dialog the first time it touches the key -- fatal with no screen.
    security set-key-partition-list -S apple-tool:,apple:,codesign: \
      -s -k "$kc_pass" "$keychain" >/dev/null
    # shellcheck disable=SC2086
    security list-keychains -d user -s "$keychain" $original

    export ASC_KEY_ID ASC_ISSUER_ID ASC_KEY_PATH XCODEBUILD_AUTH SIGNING_KEYCHAIN
    ASC_KEY_ID=$(cat ${lib.escapeShellArg cfg.ascKeyIdFile})
    ASC_ISSUER_ID=$(cat ${lib.escapeShellArg cfg.ascIssuerIdFile})
    ASC_KEY_PATH=${lib.escapeShellArg cfg.ascKeyP8File}
    SIGNING_KEYCHAIN="$keychain"
    XCODEBUILD_AUTH="-allowProvisioningUpdates -authenticationKeyPath $ASC_KEY_PATH -authenticationKeyID $ASC_KEY_ID -authenticationKeyIssuerID $ASC_ISSUER_ID"

    "$@"
  '';

  pathOption = description: lib.mkOption {
    type = lib.types.str;
    inherit description;
  };
in
{
  options.custom.iosSigning = {
    enable = lib.mkEnableOption "the with-signing-keychain helper for iOS/macOS code signing";

    identityP12Base64File = pathOption "File holding the distribution identity as a base64-encoded .p12.";

    identityP12Path = lib.mkOption {
      type = lib.types.str;
      default = "/run/ios-signing/identity.p12";
      description = ''
        Where the decoded .p12 is written (mode 0400, owned by `user`),
        for tools that need a .p12 file. Derived from
        identityP12Base64File on every activation and at boot.
      '';
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = config.system.primaryUser;
      defaultText = lib.literalExpression "config.system.primaryUser";
      description = "The account that signs, and so owns the decoded identity.";
    };
    identityPasswordFile = pathOption "File holding the .p12's password, with no trailing newline.";
    ascKeyIdFile = pathOption "File holding the App Store Connect API key ID.";
    ascIssuerIdFile = pathOption "File holding the App Store Connect issuer ID (the team's).";
    ascKeyP8File = pathOption "File holding the App Store Connect API key (.p8, PEM).";
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ helper ];

    # After sops-nix, which installs secrets in postActivation at mkAfter
    # (1500): this runs at 2000, so the base64 secret is already current.
    system.activationScripts.postActivation.text = lib.mkOrder 2000 ''
      echo "decoding the signing identity..." >&2
      ${decodeIdentity} || echo "warning: signing identity not decoded" >&2
    '';

    # /run is cleared at boot, when sops-nix re-installs secrets from its
    # own launchd daemon rather than activation; decodeIdentity waits for
    # that secret to appear.
    launchd.daemons.ios-signing-identity.serviceConfig = {
      Label = "org.nixos.ios-signing-identity";
      ProgramArguments = [ "${decodeIdentity}" ];
      RunAtLoad = true;
      StandardErrorPath = "/var/log/ios-signing-identity.log";
    };
  };
}
