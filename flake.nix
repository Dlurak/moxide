{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forEachSystem = function:
      nixpkgs.lib.genAttrs ["aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux"] (
        system: function nixpkgs.legacyPackages.${system}
      );
  in {
    homeManagerModules.moxide = import ./nix/hm-module.nix;
    packages = forEachSystem (pkgs: rec {
      moxide = import ./nix/build.nix {inherit pkgs;};
      default = moxide;
    });
    devShell = forEachSystem (pkgs:
      pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          rust-analyzer
          rustPackages.clippy
          bacon
        ];
        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      });
  };
}
