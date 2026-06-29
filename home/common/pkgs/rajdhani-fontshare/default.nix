{ lib, stdenv, fetchurl }:

stdenv.mkDerivation rec {
  pname = "rajdhani-fontshare";
  version = "2.0";

  srcs = [
    # Downloaded from official Google Fonts repo because Fontshare versions
    # had corrupted metadata (family: "false" for Regular and Bold).
    (fetchurl {
      url = "https://raw.githubusercontent.com/google/fonts/main/ofl/rajdhani/Rajdhani-Light.ttf";
      sha256 = "1hqk29j1rbhpskzpnzl8xad0ck2rh5zwy3vqqmhq2lv9mw9ry3hl";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/google/fonts/main/ofl/rajdhani/Rajdhani-Regular.ttf";
      sha256 = "164v0f76ii7cricjzyc2qmb4c7mcgg2jwl39wnk530iim0lc47vf";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/google/fonts/main/ofl/rajdhani/Rajdhani-Medium.ttf";
      sha256 = "1hpaj5jqvf4pdg18cnkzzaifczdmx9i1ffy5ba3y61n2wk7pvzqj";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/google/fonts/main/ofl/rajdhani/Rajdhani-SemiBold.ttf";
      sha256 = "1nimy9dq4w02l2fbx2xprw6qcazxx5ym6nmhzscmjrna31dd5fwl";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/google/fonts/main/ofl/rajdhani/Rajdhani-Bold.ttf";
      sha256 = "1zba4aii129c3bdcn5ajp90rh5wnazvhn3clfyb4x8c66bfp0539";
    })
  ];

  unpackPhase = "true";

  installPhase = ''
    install -Dm644 ${builtins.elemAt srcs 0} $out/share/fonts/truetype/Rajdhani-Light.ttf
    install -Dm644 ${builtins.elemAt srcs 1} $out/share/fonts/truetype/Rajdhani-Regular.ttf
    install -Dm644 ${builtins.elemAt srcs 2} $out/share/fonts/truetype/Rajdhani-Medium.ttf
    install -Dm644 ${builtins.elemAt srcs 3} $out/share/fonts/truetype/Rajdhani-SemiBold.ttf
    install -Dm644 ${builtins.elemAt srcs 4} $out/share/fonts/truetype/Rajdhani-Bold.ttf
  '';

  meta = with lib; {
    description = "Rajdhani font family from Fontshare";
    homepage = "https://www.fontshare.com/fonts/rajdhani";
    license = licenses.ofl;
    platforms = platforms.all;
  };
}
