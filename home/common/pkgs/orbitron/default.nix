{ lib, stdenv, fetchurl }:

stdenv.mkDerivation rec {
  pname = "orbitron";
  version = "1.000"; # Based on standard versioning, or master ref

  srcs = [
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-Regular.ttf";
      sha256 = "1nb2v591i71b2c1y8kq7bhy0akpzrpzv20h8mmrvnw6qvglbbhpq";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-Medium.ttf";
      sha256 = "1lpqdmsrmqflpzp9kvih74kdx8s0zv6rcnr8j9xl3qw6sy9sp5mw";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-SemiBold.ttf";
      sha256 = "0gb3qlpgark68vahnfrv6lpr8fmb9pd7cb8dzidddmi9zwc9srn7";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-Bold.ttf";
      sha256 = "1sbvqiv8rin789479nnswwi3m4fipc5hjwikqmdr9vsmlzwpkjyj";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-ExtraBold.ttf";
      sha256 = "0hhn9sd1m97fmk6w8sdaj2ylky73a0gi23p7sj50j8hzwvq24h7q";
    })
    (fetchurl {
      url = "https://raw.githubusercontent.com/googlefonts/orbitron-vf/master/fonts/ttf/Orbitron-Black.ttf";
      sha256 = "1g61qn7lzxiglabk4xnd0yds6ajnwpkvfrm3krhq8c9gfnxmcifi";
    })
  ];

  unpackPhase = "true";

  installPhase = ''
    install -Dm644 ${builtins.elemAt srcs 0} $out/share/fonts/truetype/Orbitron-Regular.ttf
    install -Dm644 ${builtins.elemAt srcs 1} $out/share/fonts/truetype/Orbitron-Medium.ttf
    install -Dm644 ${builtins.elemAt srcs 2} $out/share/fonts/truetype/Orbitron-SemiBold.ttf
    install -Dm644 ${builtins.elemAt srcs 3} $out/share/fonts/truetype/Orbitron-Bold.ttf
    install -Dm644 ${builtins.elemAt srcs 4} $out/share/fonts/truetype/Orbitron-ExtraBold.ttf
    install -Dm644 ${builtins.elemAt srcs 5} $out/share/fonts/truetype/Orbitron-Black.ttf
  '';

  meta = with lib; {
    description = "Orbitron geometric sans-serif typeface";
    homepage = "https://github.com/googlefonts/orbitron-vf";
    license = licenses.ofl;
    platforms = platforms.all;
  };
}
