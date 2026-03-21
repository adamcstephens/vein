{
  lib,
  rustPlatform,
  cacert,
  installShellFiles,
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

  postInstall = ''
    installShellCompletion --cmd vein \
      --bash <($out/bin/vein completions bash) \
      --fish <($out/bin/vein completions fish) \
      --zsh <($out/bin/vein completions zsh)
  '';

  nativeBuildInputs = [ installShellFiles ];

  meta.mainProgram = "vein";
}
