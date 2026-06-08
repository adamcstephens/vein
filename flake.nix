{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.beans
              pkgs.git-cliff
              pkgs.gojq
              pkgs.just
              pkgs.process-compose
              pkgs.vikunja

              pkgs.cargo
              pkgs.cargo-outdated
              pkgs.clippy
              pkgs.rustc
              pkgs.rust-analyzer
              pkgs.rustfmt
            ];

            shellHook = ''
              export VIKUNJA_SRC=${pkgs.vikunja.src}
            '';
          };

          packages = rec {
            default = vein;
            vein = pkgs.callPackage ./nix/package.nix { };
          };
        };
    };
}
