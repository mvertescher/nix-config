{ lib, ... }:

{
  programs.alacritty = {
    enable = true;
    settings = {
      terminal.shell.program = "nu";
      window.opacity = lib.mkForce 0.10;

      # font = {
      #   normal = {
      #     family = "DroidSansM Nerd Font";
      #     style = "Medium";
      #   };
      #   size = config.programs.alacritty.fontsize;
      # };
      selection.save_to_clipboard = true;
    };
  };
}
