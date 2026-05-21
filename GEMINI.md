# Nix Config - Project Instructions

This repository manages a NixOS and Home Manager configuration using Nix Flakes.

## Project Structure

- `flake.nix`: Entry point for the flake, defining inputs and system/home configurations.
- `home/`: Home Manager configurations.
    - `common/`: Shared configurations and reusable settings.
        - `cli/`: CLI-related tools and configurations.
        - `gui/`: Graphical User Interface packages (Alacritty, VSCode).
        - `hyprland/`: Desktop layout and key binds.
        - `pkgs/`: Custom package definitions or overrides.
    - `themes/`: Visual styles and customization resources.
        - `cybr/`: Cybrcolors Base16 theme configurations, wallpapers, status bar (Waybar), lockscreens (Hyprlock), notifications (SwayNC), and browser styles (Firefox).
    - `host/`: Host-specific machine profiles (desktop, laptop, server, terra).
- `system/`: NixOS system-level configurations.
    - `host/`: Machine-specific hardware and system configurations.
    - `wm/`: Window manager configurations (e.g., Hyprland).
- `lib/`: Helper functions and overlays.
- `outputs/`: Logic for generating Home Manager and NixOS configurations.
- `scripts/`: Utility scripts for VM management and caching.

## Architecture & Conventions

- **Modular Configuration**: Configurations are split into specialized files (e.g., `git.nix`, `shell.nix`) and imported into `default.nix` within their respective directories.
- **Home Manager**: Used for user-level configuration and package management.
- **Alphabetical Ordering**: Maintain alphabetical order for package lists in `home.packages` and `environment.systemPackages` to keep files organized.
- **Stylix**: The project uses Stylix for consistent styling across applications.

## Workflows

- **Updating Packages**: Add new packages to the relevant module in `home/` (e.g., `home/cli/default.nix` for general CLI tools).
- **Switching Configurations**: Use the `./switch` script to apply changes (currently supports `terra`).
    - Example: `./switch terra`
- **Testing**: Before committing, run `nix flake check` to ensure there are no evaluation errors.

## Style Guide

- Follow existing Nix formatting (mostly 2-space indentation).
- Prefer `with pkgs; [ ... ]` for package lists within modules.
- Add brief comments next to packages to explain their purpose if not immediately obvious (e.g., `bat # better cat`).

## AI Agent Instructions

### Taking Screenshots
If an AI coding agent needs to take a screenshot or capture the screen during verification in a Wayland/Hyprland environment, use the `grimblast` utility from the command line:
- **Capture entire screen to file**: `grimblast save screen /path/to/output.png`
- **Capture active window to file**: `grimblast save active /path/to/output.png`
- **Capture entire screen to clipboard**: `grimblast copy screen`
