{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-compat, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        aoc24 = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = builtins.path { path = ./.; name = "aoc24"; };

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "aoclib-rs-0.0.10" = "sha256-icaKVUEcqV7GXa+5nBF+gzJZ0TAZyFMw1x8txmOFLO4=";
            };
          };
        };
        aoc24-shell = pkgs.mkShell {
          inputsFrom = [ aoc24 ];
          packages = with pkgs; [
            clippy
            rustfmt
          ];
        };
      in
      {
        packages = {
          inherit aoc24;
          default = aoc24;
        };
        devShells = {
          inherit aoc24-shell;
          default = aoc24-shell;
        };
      }
    );
}
