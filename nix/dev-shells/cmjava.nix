{
  cmjava,
  mkShell,
  fenixRustToolchain,
  bashInteractive,
  rust-analyzer,
  reuse,
  just,
}:

mkShell {

  inputsFrom = [ cmjava ];

  packages = [
    fenixRustToolchain

    bashInteractive
    rust-analyzer
    reuse
    just
  ];

  shellHook = ''
    unset SOURCE_DATE_EPOCH
  '';

}
