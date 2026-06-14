{ lib, stdenv, fetchurl }:

stdenv.mkDerivation rec {
  pname = "rajdhani-fontshare";
  version = "2.0";

  srcs = [
    (fetchurl {
      url = "https://cdn.fontshare.com/wf/LD4GN5DAY3TSLWUL3GLS46MEB6VPUTR3/Q7KOJ6SEN53YLOGH553ZT4C4CRM26FS5/KEUOY5K2E2AMUOGQQJXBQIRNTOH7KQHK.ttf";
      sha256 = "0awy199gipwjx085h8031nkv211bdg8ai8n25a6c52ni6bd6l6b1";
    })
    (fetchurl {
      url = "https://cdn.fontshare.com/wf/TORRJZXM2VMPF273F2CW63EE7L3YA4M6/BFYFLXKSXG7BLHOLOWBOHF5NM7G6JOGB/LDKU4RGE4SF5XSV27OQVKWRBND7BGDBM.ttf";
      sha256 = "1lz12s88yxnf4yf8dimrihhhqpwnz8b2xpqlsb5gwfjfxx9qqm5r";
    })
    (fetchurl {
      url = "https://cdn.fontshare.com/wf/NA4IAGUSLF4EHK37TDKERPZ7NDGSDIRO/4XGPZHVJIUBBZRT537KB4JKL6RNS3XIB/VX4XSJAXNZJ4VVJS4IPP5XNZGYR2DJJL.ttf";
      sha256 = "1b6kjdv9pmg2z9b0z8gz8m94blr7fr4iwqiw55an64j0q6mp9s84";
    })
    (fetchurl {
      url = "https://cdn.fontshare.com/wf/3YMFAGHETBCEBNTBSRMIDXZ6E24ARXRZ/ECW7GXGVRFMXAJVR23A2AEV3VB6POCLM/O6HA6YRXLOW7WGNAXUQYSMKUL6HRPNEV.ttf";
      sha256 = "10nm926l3g45r7qkv6n0qdsdwjpfn5fff5bpv41d55pmhaf3q72i";
    })
    (fetchurl {
      url = "https://cdn.fontshare.com/wf/HWEVYDWNEA25ABA6YXCAQYCSVDGT2JQF/AURV42SMU5UVKKBWBHHSOHVPHFYA5SHQ/3HBRNX5OMPHHFNBDA65YLCJ5FPUVDT52.ttf";
      sha256 = "1vwz3kbxv2hasfdcls0h60y62zq1lr6kr0lim93162c4a316ikvh";
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
