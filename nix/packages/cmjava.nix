{
  flake-self,
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
      "build.rs"
    ];
  };

  cargoLock.lockFile = ../../Cargo.lock;

  VERGEN_IDEMPOTENT = "1";
  VERGEN_GIT_SHA = if flake-self ? "rev" then flake-self.rev else flake-self.dirtyRev;
  VERGEN_GIT_BRANCH = if flake-self ? "ref" then flake-self.ref else "";
  VERGEN_GIT_COMMIT_TIMESTAMP = flake-self.lastModifiedDate;
}
