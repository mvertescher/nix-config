{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, openssl
, libgit2
, zlib
}:

rustPlatform.buildRustPackage rec {
  pname = "repo-rs";
  version = "0.2.3";

  src = fetchFromGitHub {
    owner = "sunbeamdotpt";
    repo = "repo-rs";
    rev = "41f96e67f985722c4074fd192c3b48ae8c80f922";
    hash = "sha256-rxJEEYOVAGKJpvmW+1xI+a/7k/VH8UYzkwWYqnIjTHg=";
  };

  cargoHash = "sha256-4an6nedHUdfH6U78tSFujueIgylvdtv8QPuvPbLJNV0=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl libgit2 zlib ];

  doCheck = false;

  postInstall = ''
    mv $out/bin/repo $out/bin/repo-rs
  '';

  meta = with lib; {
    description = "A fast but experimental/buggy Rust implementation of the Android repo tool";
    homepage = "https://github.com/sunbeamdotpt/repo-rs";
    license = with licenses; [ mit asl20 ];
  };
}
