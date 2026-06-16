# Entropism UI Tool Kit

The goal of this project is to create an `entropism-ui` tool kit that is nearly identical to the original source images.
Later, this will expand to create derivative panel designs to cover common use cases.

## Developer & Agent Rules

- **Git State**: When making changes to this folder, always leave all changes staged in git (e.g., `git add`).
- **Verification**: Always verify any changes with `scripts/run_headless_screenshot.sh` and perform at least a visual verification of the results.
- **Development Workflow**: Use `cargo` for development and testing of `entropism-ui` as it is significantly faster than rebuilding with Nix. Prefer using the new Nix CLI (`nix develop -f shell.nix -c cargo <cmd>`) to run cargo commands within the correct environment containing all GUI/system library dependencies and fonts.


