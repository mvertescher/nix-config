# The browser's *identity*: that it exists, which profile it uses, which
# extensions are installed, and the preferences that are not a matter of
# taste. Its *appearance* belongs to the active theme, which contributes
# `userChrome` and palette-derived settings on top of this.
#
# That split is the point of this file. Before it, both
# `home/themes/cybr/firefox` and `home/themes/lib/era.nix` declared
# `programs.firefox.enable`, `profiles.default.id = 0` and an overlapping
# set of preferences -- two copies, free to drift. Worse, only cybr
# declared `policies.ExtensionSettings`, so **Sidebery was installed
# under the vendored theme and absent under all four generated eras**.
# An extension that appears and disappears with a colour scheme is the
# same defect class as a daemon started from `exec-once`: something that
# is not a theme concern, made to depend on the theme anyway.
#
# So: extensions and prefs here, chrome in the theme. A theme may still
# override any preference below -- home-manager merges the attrsets, and
# nothing here is `mkForce`.
{ ... }:

let
  # Gecko ids, from each extension's own manifest. An id names both the
  # policy entry that installs the extension and the
  # `browser-extension-data/<id>` directory its storage lives in, so a
  # wrong one fails silently in two places at once.
  sideberyId = "{3c078156-979c-498b-8990-85f7987dd929}";
  onePasswordId = "{d634138d-c276-4fc8-924b-40a0ea21d284}";

  # `normal_installed` rather than `force_installed`: the extension is
  # installed and cannot be removed by accident, but it can still be
  # disabled from about:addons. `force_installed` also blocks disabling,
  # which is the wrong trade for a password manager you may want to turn
  # off in a throwaway window.
  fromAmo = slug: {
    installation_mode = "normal_installed";
    install_url = "https://addons.mozilla.org/firefox/downloads/latest/${slug}/latest.xpi";
  };
in
{
  programs.firefox = {
    enable = true;

    policies.ExtensionSettings = {
      ${sideberyId} = fromAmo "sidebery";

      # Browser unlock and autofill need the desktop app as well as this
      # extension; `custom.onePassword` in system/onepassword.nix is the
      # other half, including the NixOS-specific wrapper name the app has
      # to be told to trust.
      ${onePasswordId} = fromAmo "1password-x-password-manager";
    };

    profiles.default = {
      id = 0;

      settings = {
        # Required by every theme's `userChrome`; harmless without one.
        "toolkit.legacyUserProfileCustomizations.stylesheets" = true;

        "browser.startup.homepage" = "https://github.com";

        # The password manager is 1Password. Firefox's built-in store
        # competing with it means two prompts on every login form and two
        # places a credential can rot.
        "signon.rememberSignons" = false;
      };
    };
  };
}
