{
  description = "mdb-search - (International | The) Movie Database search tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        # `nix develop`
        devShells.default = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            rustup
            # rustPlatform.cargoSetupHook
            pkg-config

            xorg.libxcb
            wayland
            openssl
          ];

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };

        # `nix build`
        packages.default = rustPlatform.buildRustPackage {
          name = "mdb-search";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            rustPlatform.cargoSetupHook
            pkg-config

            xorg.libxcb
            openssl
            wayland
          ];
          buildInputs = with pkgs; [
            openssl
          ];

          RUST_BACKTRACE = 1;
        };
      }
    );
}
