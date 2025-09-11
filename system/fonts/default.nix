{ pkgs, ... }:

{
  fonts.packages = with pkgs; [
    nerd-fonts.droid-sans-mono
    nerd-fonts._0xproto
  ];
}