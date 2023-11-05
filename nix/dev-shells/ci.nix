{
  packageName,
  pkgs,
  mkShell,
  fenixRustToolchain,
  bashInteractive,
  reuse,
  just,
  commitlint,
  eclint,
}:
mkShell {
  inputsFrom = [pkgs.${packageName}];

  packages = [
    fenixRustToolchain

    bashInteractive

    reuse
    just
    commitlint
    eclint
  ];
}
