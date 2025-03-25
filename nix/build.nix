{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  pname = "moxide";
  version = "0.2.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
