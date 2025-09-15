{
  inputs = {
    fenix = {
        url = "github:nix-community/fenix";
    };
    flake-utils = {
        url = "github:numtide/flake-utils";
    };
    naersk = {
        url = "github:nix-community/naersk";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.wasm32-wasip1.latest.rust-std
          ];

        naersk' = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
        };

      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          release = true;
          CARGO_BUILD_TARGET = "wasm32-wasip1";
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo ];
        };
      }
    );
}
