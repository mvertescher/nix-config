# Discover per-host modules provided by a private wrapper flake.
# `extraConfig` is a path to the wrapper's home config root; any file or
# directory under <extraConfig>/host whose name matches `target` (exactly,
# or as a suffix, so `foo-laptop.nix` matches `laptop`) is returned as a
# module to merge in. Anything other than a path (e.g. the default `{ }`)
# yields no modules, so consumers without a wrapper are unaffected.
{ lib }:

extraConfig: target:
let
  hostDir = extraConfig + "/host";
in
if builtins.typeOf extraConfig == "path" && builtins.pathExists hostDir then
  let
    entries = builtins.readDir hostDir;
    matches = lib.filterAttrs
      (name: _:
        let
          cleanName = lib.removeSuffix ".nix" name;
        in
        cleanName == target || lib.hasSuffix target cleanName
      )
      entries;
  in
  lib.mapAttrsToList (name: _: hostDir + "/${name}") matches
else
  [ ]
