{ pkgs, lib, config, ... }:

let
  shades = import ../../../lib/shades.nix { inherit lib; };

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

  # Every family below is base16 accent + two derived shades. The mid and
  # dark variants used to be literals lifted from cybr's 33-colour
  # palette.nix, which meant they stayed cybr-red no matter which scheme
  # stylix had loaded. Deriving them keeps the bar consistent with the
  # active theme.
  xdg.configFile."waybar/colors.css".text =
    let
      inherit (shades.forColors config.lib.stylix.colors) mid dark;
      family = name: slot: ''
        @define-color ${name}0 #${config.lib.stylix.colors.${slot}};
        @define-color ${name}1 #${mid slot};
        @define-color ${name}2 #${dark slot};
      '';
    in
    ''
      /* Generated dynamically from Stylix active palette */
      @define-color no0 #${config.lib.stylix.colors.base00};
      @define-color no1 #${config.lib.stylix.colors.base01};
      @define-color no2 #${config.lib.stylix.colors.base02};

      ${family "re" "base08"}
      ${family "gr" "base0B"}
      ${family "ye" "base0A"}
      ${family "bl" "base0D"}
      ${family "vi" "base0E"}
      ${family "cy" "base0C"}
      ${family "or" "base09"}
      ${family "pi" "base0F"}
      /* wh/me are foreground ramps rather than accents: base16 already
         carries the mid step (base03/base04), so only the dark end is
         derived. */
      @define-color wh0 #${config.lib.stylix.colors.base05};
      @define-color wh1 #${config.lib.stylix.colors.base03};
      @define-color wh2 #${dark "base05"};

      @define-color me0 #${config.lib.stylix.colors.base04};
      @define-color me1 #${mid "base04"};
      @define-color me2 #${dark "base04"};
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
