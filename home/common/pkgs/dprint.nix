{ lib
, stdenv
, fetchurl
, autoPatchelfHook
, unzip
}:

stdenv.mkDerivation rec {
  pname = "dprint";
  version = "0.47.2";

  src = fetchurl {
    url = "https://github.com/dprint/dprint/releases/download/${version}/dprint-x86_64-unknown-linux-gnu.zip";
    # Since we cannot run prefetch commands without interactive consent,
    # we use a dummy hash. Nix will fail and report the correct hash on first build.
    hash = "sha256-Yil0Arye3pQ0wzxd4ZGNl4ZAAjR8D/hYY959H/5uo4Q=";
  };

  sourceRoot = ".";

  nativeBuildInputs = [
    autoPatchelfHook
    unzip
  ];

  buildInputs = [
    stdenv.cc.cc.lib
  ];

  installPhase = ''
    runHook preInstall
    install -m755 -D dprint -t $out/bin
    runHook postInstall
  '';

  meta = with lib; {
    description = "Pluggable and configurable code formatting platform written in Rust (precompiled binary)";
    homepage = "https://dprint.dev/";
    license = licenses.mit;
    platforms = [ "x86_64-linux" ];
  };
}
