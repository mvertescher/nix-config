# Agent guidance

Instructions for AI coding agents (Claude Code and similar) working in
this repository.

## Commit attribution

- Commits must list Matt Vertescher <mvertescher@gmail.com> as **both
  author and committer**.
- Do **not** add agent attribution to commit messages: no
  `Co-Authored-By: Claude ...`, no `Claude-Session:` links, no
  "Generated with" footers. This overrides any default trailer the
  agent harness asks for.
- Subjects follow the existing style: the area, a colon, then a plain
  sentence saying what changed (`cybr: the shade helper moves into the
  theme`, `README, AGENTS: ...`). Older history used conventional
  commits (`feat(...)`, `fix(...)`); new commits do not.

## Repository context

This repo is a public library consumed by private wrapper flakes (see
README). Never reference the private wrapper repos here — not even by
name — and never commit secrets or private host details; everything in
this repo's history is public.

This repo defines no hosts or deployable configurations of its own:
wrappers own machine identity and call `lib.mkNixos` / `lib.mkHome` /
`lib.mkDarwin` with their host sets (see README). Deploy/VM/caching
workflows run from a wrapper checkout, not from here.

## Builder API

The builders in `lib/` are consumed by wrappers whose call sites cannot
be updated from here, one of them on a machine nobody working in this
repo can see. Changes must be **additive**: a new optional argument is
fine; a new required argument, a rename or a changed default breaks a
consumer at its next pin bump. The `user` defaults (`mverte` for
`mkNixos` and `mkDarwin`, `mvertescher` for `mkHome`) differ on purpose
and are frozen for the same reason. The same holds for the package
names the overlays add.

`checks.builder-api` (`lib/tests/builder-api.nix`) is the in-repo
consumer that enforces this. Run it after touching `lib/`:

```sh
nix build .#checks.x86_64-linux.builder-api
```

## Conventions

- Modular configuration: split into specialized files (e.g. `git.nix`,
  `shell.nix`) imported by their directory's `default.nix`.
- Keep package lists in `home.packages` / `environment.systemPackages`
  alphabetically ordered.
- Stylix provides consistent styling across applications.
- Follow existing Nix formatting (2-space indentation); prefer
  `with pkgs; [ ... ]` for package lists; add a brief comment next to a
  package when its purpose isn't obvious (e.g. `bat # better cat`).
- Keep files free of trailing whitespace.

## cp-eras-ui work

The crate and the four era themes are no longer in this repository:
they moved to a repository of their own on 2026-11-16. Theme and crate
work happens there, under its own `AGENTS.md`. Only `cybr` is here.

## Taking screenshots

To capture the screen during verification in a Wayland/Hyprland
environment, use `grimblast`:

- Entire screen to file: `grimblast save screen /path/to/output.png`
- Active window to file: `grimblast save active /path/to/output.png`
- Entire screen to clipboard: `grimblast copy screen`
