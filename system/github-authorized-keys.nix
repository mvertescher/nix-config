# GitHub-backed SSH authorized keys.
#
# sshd asks github.com/<user>.keys at login time, in addition to the
# static keys in lib/ssh-keys.nix. Adding a machine is then
# `gh ssh-key add ~/.ssh/id_ed25519.pub` with no rebuild and no
# redeploy; the static list stays as the fallback that keeps login
# working while GitHub is unreachable.
#
# This lives in the public repo because the installer ISO (see
# ./installer.nix) is built from this flake alone and cannot import
# anything private; the private server host imports this same file by
# path. It used to be copy-pasted in both places.
#
# FAIL-CLOSED, deliberately. sshd's AuthorizedKeysCommand contract is
# stricter than it looks: auth2-pubkey.c parses the command's stdout
# first, but then does
#
#     if (exited_cleanly(pid, "AuthorizedKeysCommand", ...) != 0)
#             goto out;      /* found_key stays 0 */
#     found_key = ok;
#
# so a non-zero exit (or a signal) discards the result even when a
# matching key was printed. `curl -f` turns any HTTP error, DNS
# failure, TLS failure or timeout into a non-zero exit, and `set -e`
# propagates it, so an outage yields "no keys" and never "any key".
# Keep it that way: nothing in here may swallow curl's status.
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.custom.githubAuthorizedKeys;

  # GitHub account names and Unix account names both go into a shell
  # script - one into a URL, one into a `case` pattern. The assertions
  # below keep them to a charset that is inert in both.
  nameRe = "[A-Za-z0-9._-]+";

  # -q so a stray ~/.curlrc under the command user's home cannot
  # redirect the lookup; --proto '=https' so only TLS is ever spoken.
  # No -L: a redirect should fail the fetch, not be followed to
  # wherever it points.
  fetch = ghUser: ''
    ${pkgs.curl}/bin/curl -q -sf --proto '=https' \
      --max-time ${toString cfg.timeout} \
      https://github.com/${ghUser}.keys
  '';

  # sshd passes %u, so $1 is the local account being logged into. The
  # command runs for *every* account, so an empty localUsers means
  # "answer for anyone" - which is what the installer ISO wants (root
  # and nixos both), and what a real host should not have.
  guard = lib.optionalString (cfg.localUsers != [ ]) ''
    case "''${1-}" in
      ${lib.concatStringsSep "|" cfg.localUsers}) ;;
      *) exit 0 ;;
    esac
  '';

  script = pkgs.writeShellScript "github-authorized-keys" ''
    set -eu
    ${guard}
    ${lib.concatMapStringsSep "\n" fetch cfg.githubUsers}
  '';
in
{
  options.custom.githubAuthorizedKeys = {
    enable = lib.mkEnableOption "SSH authorized keys fetched from GitHub at login time";

    githubUsers = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "mvertescher" ];
      description = ''
        GitHub accounts whose published keys are trusted. Every key on
        every listed account is accepted, so this is exactly as
        trustworthy as those accounts' 2FA.

        Only user accounts work: GitHub serves `/<user>.keys`
        unauthenticated but has no equivalent for an organisation, so
        an org has to be expanded into its members here.

        Fetched in order and concatenated. If any one fetch fails the
        script exits non-zero and sshd discards the whole result - the
        same all-or-nothing behaviour a single fetch has.
      '';
    };

    localUsers = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "mverte" ];
      description = ''
        Local accounts the lookup answers for. Empty means every
        account, including root - only appropriate for a
        single-purpose image like the installer ISO. On a real host
        list the accounts explicitly, so that a service account or a
        future root login cannot be entered with a GitHub key.
      '';
    };

    timeout = lib.mkOption {
      type = lib.types.ints.positive;
      default = 5;
      description = ''
        Seconds curl may spend on each fetch. This sits in the login
        path, so it is a latency budget as much as a network one: on
        timeout the fetch fails and only the static
        `authorizedKeys.keys` remain.
      '';
    };

    commandUser = lib.mkOption {
      type = lib.types.str;
      default = "nobody";
      description = ''
        Account sshd runs the lookup as. Wants to be an unprivileged
        account with no other role on the host.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.githubUsers != [ ];
        message = "custom.githubAuthorizedKeys is enabled but no githubUsers are listed";
      }
      {
        assertion = lib.all (u: builtins.match nameRe u != null) cfg.githubUsers;
        message = "custom.githubAuthorizedKeys.githubUsers entries must match ${nameRe}";
      }
      {
        assertion = lib.all (u: builtins.match nameRe u != null) cfg.localUsers;
        message = "custom.githubAuthorizedKeys.localUsers entries must match ${nameRe}";
      }
    ];

    services.openssh = {
      authorizedKeysCommand = "${script} %u";
      authorizedKeysCommandUser = cfg.commandUser;
    };
  };
}
