{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  pname = "moxide";
  version = "0.3.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
