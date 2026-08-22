# entropism launcher: a list, drawn once.
#
# One column, square, 1px border, no icons and no selection highlight
# beyond a flat border-toned row. Everything comes from the resolved
# roles so a colour override reaches the launcher too.
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.themes.entropism;
  c = cfg.resolvedColors;
in
{
  config = lib.mkIf cfg.enable {
    home.packages = [ pkgs.rofi ];

    xdg.configFile."rofi/config.rasi".text = ''
      /* Generated from themes/entropism roles. */
      configuration {
        modi: "drun,run";
        show-icons: false;
        display-drun: "run";
        drun-display-format: "{name}";
        me-select-entry: "";
        me-accept-entry: [ MousePrimary ];
      }
      @theme "entropism"
    '';

    xdg.configFile."rofi/entropism.rasi".text = ''
      /* Generated from themes/entropism roles. No literals here. */
      * {
        background-color: transparent;
        text-color:       ${c.fg};
        font:             "${cfg.uiFont.name} 12";
      }

      window {
        background-color: ${c.panel};
        border:           1px;
        border-color:     ${c.border};
        border-radius:    0;
        width:            40%;
        padding:          0;
      }

      mainbox {
        children: [ inputbar, listview ];
        padding:  0;
      }

      inputbar {
        background-color:   ${c.bg};
        border:             0 0 1px 0;
        border-color:       ${c.border};
        padding:            6px 8px;
        children:           [ prompt, entry ];
      }

      prompt {
        text-color: ${c.dim};
        padding:    0 6px 0 0;
      }

      entry {
        placeholder:            "";
        placeholder-color:      ${c.dim};
      }

      listview {
        lines:          12;
        columns:        1;
        scrollbar:      false;
        fixed-height:   false;
        padding:        4px 0;
      }

      element {
        padding:       4px 10px;
        border-radius: 0;
      }

      element normal.normal {
        text-color: ${c.fg};
      }

      /* Selection is a flat band, not a glow. */
      element selected.normal {
        background-color: ${c.border};
        text-color:       ${c.fg};
      }

      element urgent.normal,
      element selected.urgent {
        text-color: ${c.alert};
      }
    '';
  };
}
