{ pkgs, ... }:

let
    gdb-dashboard = pkgs.fetchFromGitHub {
        owner = "cyrus-and";
        repo = "gdb-dashboard";
        rev = "v0.12.0";
        sha256 = "jAEnrS6wxv/pHD+pbOPfAljvHNlZNyf9AGOmMZgqPvw=";
    };
in {
    home.file.".gdbinit".text = builtins.readFile(gdb-dashboard + "/.gdbinit");
}
