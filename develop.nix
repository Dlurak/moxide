{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "rust-dev-shell";

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
  ];

  shellHook = ''
    echo "Welcome to the Rust development environment!"
    export CARGO_INCREMENTAL=1
    export RUST_BACKTRACE=1
  '';
}
