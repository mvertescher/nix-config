{ pkgs, lib, config, ... }:

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

  # Rust replacement for upstream's mediaplayer.py, which needed python3 plus
  # PyGObject and the Playerctl GIR typelib. Talks D-Bus directly, so the
  # music module has no interpreter dependency at runtime.
  cybr-media = pkgs.callPackage ./cybr-media { };

  modulesTemplate = builtins.readFile ./cybr-waybar/modules.jsonc;

  templatedModules = builtins.replaceStrings
    [
      "'kitty --class scratchpad-btop btop'"
      "'kitty --class scratchpad-nvtop nvtop'"
      "'kitty --class scratchpad-large nu -c upall'"
      "~/.config/waybar/scripts/mediaplayer.py"
    ]
    [
      "'${mkScratchpadCmd "scratchpad-btop" "btop"}'"
      "'${mkScratchpadCmd "scratchpad-nvtop" "nvtop"}'"
      "'${mkScratchpadCmd "scratchpad-large" "nu -c upall"}'"
      (lib.getExe cybr-media)
    ]
    modulesTemplate;
in
{
  # Launch the bar alongside the theme's other components (see swaync.nix,
  # hyprpaper.nix): programs.waybar only installs and configures it, and the
  # packaged waybar.service is inert because hyprland runs with
  # systemd.enable = false.
  wayland.windowManager.hyprland.settings = {
    exec-once = [
      "waybar"
    ];
  };

  programs.waybar = {
    enable = true;
  };

  xdg.configFile."waybar/colors.css".text = ''
    /* Generated dynamically from Stylix active palette */
    @define-color no0 #${config.lib.stylix.colors.base00};
    @define-color no1 #${config.lib.stylix.colors.base01};
    @define-color no2 #${config.lib.stylix.colors.base02};

    @define-color re0 #${config.lib.stylix.colors.base08};
    @define-color re1 #631F21;
    @define-color re2 #331215;

    @define-color gr0 #${config.lib.stylix.colors.base0B};
    @define-color gr1 #15633F;
    @define-color gr2 #0C3423;

    @define-color ye0 #${config.lib.stylix.colors.base0A};
    @define-color ye1 #635618;
    @define-color ye2 #332D10;

    @define-color bl0 #${config.lib.stylix.colors.base0D};
    @define-color bl1 #152966;
    @define-color bl2 #0C1737;

    @define-color vi0 #${config.lib.stylix.colors.base0E};
    @define-color vi1 #421666;
    @define-color vi2 #230D37;

    @define-color cy0 #${config.lib.stylix.colors.base0C};
    @define-color cy1 #124E56;
    @define-color cy2 #0B292F;

    @define-color wh0 #${config.lib.stylix.colors.base05};
    @define-color wh1 #${config.lib.stylix.colors.base03};
    @define-color wh2 #1E2025;

    @define-color me0 #${config.lib.stylix.colors.base04};
    @define-color me1 #212638;
    @define-color me2 #0D1120;

    @define-color or0 #${config.lib.stylix.colors.base09};
    @define-color or1 #63290E;
    @define-color or2 #33170B;

    @define-color pi0 #${config.lib.stylix.colors.base0F};
    @define-color pi1 #63164C;
    @define-color pi2 #330D2A;
  '';

  xdg.configFile."waybar/config.jsonc".source = ./cybr-waybar/config.jsonc;
  xdg.configFile."waybar/style.css".source = lib.mkForce ./cybr-waybar/style.css;
  xdg.configFile."waybar/output-switcher.sh".source = ./cybr-waybar/output-switcher.sh;
  xdg.configFile."waybar/scripts".source = ./cybr-waybar/scripts;
  xdg.configFile."waybar/svg".source = ./cybr-waybar/svg;
  xdg.configFile."waybar/modules.jsonc".text = templatedModules;

  fonts.fontconfig.enable = true;

  home.packages = [
    cybr-media
    pkgs.nerd-fonts.geist-mono
  ];
}
