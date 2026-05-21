{ lib, stdenv
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "form";
  version = "0.0.0";

  src = fetchFromGitHub {
    owner = "djmcgill";
    repo = pname;
    rev = "fcb397a39d633ba7fbda057483e0587cef05f25d";
    sha256 = "1jsh0qwpl2n77jdbzq4xxa7jbra5lj2k80aifpwgnlpfx8hmi11z";
  };

  cargoSha256 = "14s4fpzs4p3b9zssqfp0n02dz68cri7c04vm5pnh2pm6hxn3vfms";
  
  doCheck = false;

  meta = with lib; {
    description = "A Tool for splitting apart a large file with multiple modules into the idiomatic rust directory structure.";
    homepage = "https://github.com/djmcgill/form";
    license = licenses.mit;
  };
}
