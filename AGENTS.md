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
