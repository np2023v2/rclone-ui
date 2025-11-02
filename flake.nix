{
  description = "Rclone UI - A TUI for rclone cloud file management";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          sqlite
          rclone
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "🦀 Rclone UI development environment"
            echo "Rust version: $(rustc --version)"
            echo "Rclone version: $(rclone version | head -1)"
            echo ""
            echo "Available commands:"
            echo "  cargo build       - Build the project"
            echo "  cargo test        - Run tests"
            echo "  cargo run         - Run the application"
            echo "  cargo clippy      - Run linter"
            echo "  cargo fmt         - Format code"
            echo ""
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
          DATABASE_URL = "todo.db";
        };

        # Package definition
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rclone-ui";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          meta = with pkgs.lib; {
            description = "A TUI for rclone cloud file management";
            homepage = "https://github.com/np2023v2/rclone-ui";
            license = with licenses; [ mit asl20 ];
            maintainers = [ ];
          };
        };

        # App definition for easy running
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/rclone-ui";
        };
      }
    );
}
