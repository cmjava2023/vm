{
  packageName,
  pkgs,
  mkShell,
  fenixRustToolchain,
  bashInteractive,
  rust-analyzer,
  reuse,
  just,
  eclint,
  commitlint,
}:
mkShell {
  inputsFrom = [pkgs.${packageName}];

  packages = [
    fenixRustToolchain

    bashInteractive
    rust-analyzer

    reuse
    just
    eclint
    commitlint
  ];

  shellHook = ''
    unset SOURCE_DATE_EPOCH
    just --list --list-heading $'just <task>:\n'
  '';
}
