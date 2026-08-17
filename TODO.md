# TODO

Loose ends from the 2026-08-17 unattended-provisioning work (Vultr "server" host):

- [ ] **Fix `scripts/provision-server.sh` ISO publish path**: Vultr silently
  refuses to fetch ISOs from GitHub release URLs (POST /v2/iso accepts, stays
  `pending` ~30s, then the record is deleted without ever contacting GitHub).
  The working approach was serving the ISO over plain HTTP from another Vultr
  instance and giving Vultr that URL. The script's GitHub-release path never
  triggers as long as `nix-config-installer.iso` exists in the Vultr account,
  but it will break for a fresh account or after deleting/rebuilding the ISO.
- [ ] **Rotate the Vultr API key** used during provisioning (it appeared in a
  Claude Code session transcript).
- [ ] **Change the default console password** on the server: `mverte`'s
  `initialPassword` is "mverte" (system/configuration.nix). SSH password auth
  is off, so exposure is console-only, but run `passwd` anyway — or set a
  hashed password declaratively.
- [ ] **Delete the redundant stock ISO** (`nixos-minimal-26.05...iso`) from the
  Vultr account; the custom `nix-config-installer.iso` replaces it.
- [ ] **Decide whether to keep `scripts/vcs-install-nix.sh`**: superseded by
  disko + nixos-anywhere (`scripts/provision-server.sh`).
- [ ] **Consider sharing the GitHub AuthorizedKeysCommand config** between the
  server host and the installer ISO (currently duplicated in
  system/host/server/default.nix and system/installer.nix).
