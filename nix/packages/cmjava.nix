{
  packageName,
  nix-filter,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = packageName;
  version = "0.1.0";

  src = nix-filter {
    root = ../../.;
    include = [
      "src"
      "Cargo.toml"
      "Cargo.lock"
    ];
  };

  cargoLock.lockFile = ../../Cargo.lock;

}
