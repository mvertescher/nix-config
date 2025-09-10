{ ... }:

{
  services.hyprpaper = {
    enable = true;
    settings = {
      ipc = true;
      splash = false;
      preload = "~/wallpapers/default.jpg";
      wallpaper = ",~/wallpapers/default.jpg";
    };
  };
}