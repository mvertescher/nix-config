{ pkgs, lib, ... }:

let
  # Configure preferred terminal here ("alacritty" or "kitty")
  terminal = "alacritty";

  mkScratchpadCmd = class: cmd:
    if terminal == "kitty" then
      "kitty --class ${class} ${cmd}"
    else if terminal == "alacritty" then
      "alacritty --class ${class} -e ${cmd}"
    else
      throw "Unsupported terminal: ${terminal}";

  modulesTemplate = builtins.readFile ./cybr-waybar/modules.jsonc;

  templatedModules = builtins.replaceStrings
    [
      "'kitty --class scratchpad-btop btop'"
      "'kitty --class scratchpad-nvtop nvtop'"
      "'kitty --class scratchpad-large fish -c upall'"
    ]
    [
      "'${mkScratchpadCmd "scratchpad-btop" "btop"}'"
      "'${mkScratchpadCmd "scratchpad-nvtop" "nvtop"}'"
      "'${mkScratchpadCmd "scratchpad-large" "fish -c upall"}'"
    ]
    modulesTemplate;
in
{
  programs.waybar = {
    enable = true;
  };

  xdg.configFile."waybar/config.jsonc".source = ./cybr-waybar/config.jsonc;
  xdg.configFile."waybar/style.css".source = lib.mkForce ./cybr-waybar/style.css;
  xdg.configFile."waybar/output-switcher.sh".source = ./cybr-waybar/output-switcher.sh;
  xdg.configFile."waybar/scripts".source = ./cybr-waybar/scripts;
  xdg.configFile."waybar/svg".source = ./cybr-waybar/svg;
  xdg.configFile."waybar/modules.jsonc".text = templatedModules;

  fonts.fontconfig.enable = true;

  home.packages = with pkgs; [
    nerd-fonts.geist-mono
  ];
}
