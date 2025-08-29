{ lib, stdenv
, rustPlatform
, fetchFromGitHub
, pkg-config
, openssl
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-local-registry";
  version = "0.2.2";

  src = fetchFromGitHub {
    owner = "dhovart";
    repo = pname;
    rev = "${version}";
    sha256 = "zstd/4tTytSyj84aEyYiNqpmId/wpvHINm6ihvkkw64=";
  };

  cargoSha256 = "Pd2W3M7D/iuQKwBaXamNR3aUIZUDDQ8il95whToPhIA=";

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ openssl ];

  doCheck = false;

  meta = with lib; {
    description = "A cargo subcommand to ease maintenance of local registries";
    homepage = "https://github.com/dhovart/cargo-local-registry";
    license = licenses.mit;
  };
}
