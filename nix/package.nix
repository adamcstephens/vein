{
  lib,
  rustPlatform,
  cacert,
}:

rustPlatform.buildRustPackage {
  pname = "vein";
  version = (lib.importTOML ../Cargo.toml).package.version;

  src =
    with lib.fileset;
    toSource {
      root = ../.;
      fileset = unions [
        ../Cargo.toml
        ../Cargo.lock
        ../src
      ];
    };

  cargoLock.lockFile = ../Cargo.lock;

  env.SSL_CERT_FILE = "${cacert}/etc/ssl/certs/ca-bundle.crt";

  meta.mainProgram = "vein";
}
