{
  cmjava,
  mkShell,
  fenixRustToolchain,
  bashInteractive,
  reuse,
  just,
  commitlint,
  eclint,
}:

mkShell {

  inputsFrom = [ cmjava ];

  packages = [
    fenixRustToolchain

    bashInteractive
    reuse
    just

    commitlint
    eclint
  ];

}
