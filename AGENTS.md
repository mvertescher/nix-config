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
- Follow the existing conventional-commit style (`feat:`, `fix:`,
  `docs:`, ...), scoped where it helps (`fix(server): ...`).

## Repository context

This repo is a public library consumed by private wrapper flakes (see
README). Never reference the private wrapper repos here — not even by
name — and never commit secrets or private host details; everything in
this repo's history is public.

This repo defines no hosts or deployable configurations of its own:
wrappers own machine identity and call `lib.mkNixos` /
`pkgs.builders.mkHome` (see README). Deploy/VM/caching workflows run
from a wrapper checkout, not from here.

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

## Taking screenshots

To capture the screen during verification in a Wayland/Hyprland
environment, use `grimblast`:

- Entire screen to file: `grimblast save screen /path/to/output.png`
- Active window to file: `grimblast save active /path/to/output.png`
- Entire screen to clipboard: `grimblast copy screen`
