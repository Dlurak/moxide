{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    (utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        moxide = import ./nix/build.nix {inherit pkgs;};
      in {
        defaultPackage = moxide;
        packages.moxide = moxide;
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            rustPackages.clippy
            bacon
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    ))
    // {
      homeManagerModules.moxide = import ./nix/hm-module.nix;
    };
}
