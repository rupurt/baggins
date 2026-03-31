{
  description = "baggins - medical billing agent platform";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    keel.url = "github:rupurt/keel";
  };

  outputs = { self, nixpkgs, flake-utils, keel }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.just
          pkgs.cargo-nextest
          keel.packages.${system}.keel
          pkgs.git
          pkgs.pkg-config
          pkgs.protobuf
          pkgs.rustc
          pkgs.rustfmt
          pkgs.openssl
        ];

        shellHook = ''
          echo "Entering baggins dev shell"
          echo "Rust toolchain: $(rustc --version)"
          echo "Ready for Rust microservices on transit/Keel workflows."
        '';
      };
    });
}
