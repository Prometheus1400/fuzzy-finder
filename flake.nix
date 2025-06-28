{
  description = "Rust fuzzy finder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        myRustPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "my-rust-app";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # You can override this to `false` to skip tests
          doCheck = true;
        };
      in {
        packages.default = myRustPackage;

        checks = {
          inherit myRustPackage;
        };

        devShell = pkgs.mkShell {
            packages = [
                (pkgs.rust-bin.stable.latest.default.override {
                 extensions = [ "rust-src" ];
                 })
            ];
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.openssl
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
