{ lib, stdenv
, rustPlatform
, fetchFromGitHub
, pkg-config
, openssl
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-index";
  version = "0.2.4";

  src = fetchFromGitHub {
    owner = "ehuss";
    repo = pname;
    rev = "v${version}";
    sha256 = "0w5fbfrw0gcdmljs7fmm03vdn8flnyj60phawnpp4pi3kryc8in3";
  };

  cargoSha256 = "018vgwgs674g8p5dksn3hmsv23zk1r0dgmn4x4bgrkab26fbab3q";
  
  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ openssl ];

  doCheck = false;

  meta = with lib; {
    description = "A cargo subcommand to access and manipulate a registry index";
    homepage = "https://github.com/ehuss/cargo-index";
    license = licenses.mit;
  };
}
