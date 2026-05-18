{ config, ... }:

{
  programs.starship = {
    enable = true;
    settings = let
      c = config.lib.stylix.colors;
      no0 = "#${c.base00}";
      no1 = "#${c.base01}";
      no2 = "#${c.base02}";
      wh1 = "#${c.base03}";
      me0 = "#${c.base04}";
      re0 = "#${c.base08}";
      wh0 = "#${c.base06}";
      pi0 = "#${c.base0F}";

      og0 = "#${c.base09}";
      ye0 = "#${c.base0A}";
      gr0 = "#${c.base0B}";
      cy0 = "#${c.base0C}";
      bl0 = "#${c.base0D}";
      vi0 = "#${c.base0E}";

      re2 = "#${c.base02}";
      bl2 = "#${c.base01}";
      cy2 = "#${c.base01}";
      pu2 = "#${c.base01}";
      ye2 = "#${c.base01}";
      or2 = "#${c.base01}";
      me2 = "#${c.base01}";

      or0 = og0;
      pu0 = vi0;
    in {
      scan_timeout = 100;

      format = "$username[](fg:${no1} bg:${re2})$shell[](fg:${re2} bg:${re0})$directory[](fg:${re0})$git_branch$git_status$c$elixir$elm$golang$gradle$haskell$java$julia$nim$rust$scala$docker_context$time$line_break$character";

      username = {
        show_always = true;
        style_user = "bg:${no1} fg:${re0}";
        format = "[ $user ]($style)";
        disabled = false;
      };

      shell = {
        style = "bg:${re2} fg:${re0}";
        format = "[ $indicator ]($style)";
        disabled = false;
      };

      directory = {
        style = "fg:${re2} bg:${re0}";
        format = "[ $path ](bold $style)";
        truncation_length = 3;
        truncation_symbol = "…/";
      };

      line_break = {
        disabled = false;
      };

      jobs = {
        disabled = true;
      };

      character = {
        success_symbol = "[❯ ](fg:${re0})";
        error_symbol = "[](fg:${re0})";
        vicmd_symbol = "[󰆤](fg:${ye0})";
        format = "$symbol";
      };

      time = {
        disabled = false;
        time_format = "%R";
        style = "fg:${cy0} bg:${cy2}";
        format = "[](fg:${cy2})[ $time ]($style)[](fg:${cy2})";
      };

      custom = {
        time_arrow = {
          disabled = false;
          command = "echo -n \"\"";
          when = "true";
          style = "fg:${cy2}";
          format = "[ $output]($style)";
        };
        transient_time = {
          disabled = false;
          command = "date \"+%H:%M\"";
          when = "true";
          style = "fg:${cy0} bg:${cy2}";
          format = "[ $output ]($style)";
        };
      };

      git_branch = {
        symbol = "";
        style = "bg:${pu2} fg:${pu0}";
        format = "[](fg:${pu2})[ $symbol $branch ]($style)[](fg:${pu2})";
      };

      git_status = {
        style = "bg:${pu0} fg:${pu2}";
        format = "[](fg:${pu0})[ $all_status$ahead_behind ]($style)[](fg:${pu0})";
      };

      c = {
        symbol = "";
        style = "bg:${bl2} fg:${bl0}";
        format = "[](fg:${pu2} bg:${bl2})[ $symbol ($version) ]($style)";
      };

      cpp = {
        symbol = "";
        style = "bg:${bl2} fg:${bl0}";
        format = "[ $symbol ($version) ]($style)";
      };

      elm = {
        symbol = "";
        style = "bg:${bl2} fg:${bl0}";
        format = "[ $symbol ($version) ]($style)";
      };

      golang = {
        symbol = "";
        style = "bg:${bl2} fg:${bl0}";
        format = "[ $symbol ($version) ]($style)";
      };

      gradle = {
        style = "bg:${cy2} fg:${cy0}";
        format = "[](fg:${bl2} bg:${cy2})[ $symbol ($version) ]($style)";
      };

      julia = {
        symbol = "";
        style = "bg:${cy2} fg:${cy0}";
        format = "[ $symbol ($version) ]($style)";
      };

      java = {
        symbol = "";
        style = "bg:${or2} fg:${or0}";
        format = "[](fg:${cy2} bg:${or2})[ $symbol ($version) ]($style)";
      };

      rust = {
        symbol = "";
        style = "bg:${or2} fg:${or0}";
        format = "[ $symbol ($version) ]($style)";
      };

      python = {
        symbol = "";
        style = "bg:${ye2} fg:${ye0}";
        format = "[](fg:${or2} bg:${ye2})[ $symbol ($version) ]($style)";
      };

      nim = {
        symbol = "󰆥";
        style = "bg:${ye2} fg:${ye0}";
        format = "[ $symbol ($version) ]($style)";
      };

      haskell = {
        symbol = "";
        style = "bg:${pu2} fg:${pu0}";
        format = "[](fg:${ye2} bg:${pu2})[ $symbol ($version) ]($style)";
      };

      elixir = {
        symbol = "";
        style = "bg:${pu2} fg:${pu0}";
        format = "[ $symbol ($version) ]($style)";
      };

      scala = {
        symbol = "";
        style = "bg:${re2} fg:${re0}";
        format = "[](fg:${pu2} bg:${re2})[ $symbol ($version) ]($style)";
      };

      docker_context = {
        symbol = "";
        style = "bg:${bl2} fg:${bl0}";
        format = "[](fg:${re2} bg:${bl2})[ $symbol $context ]($style)";
      };
    };
  };
}
