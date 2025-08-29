{
  mkShell,
  cargo,
  rustc,
  rust-analyzer,
  checks,
}:

mkShell {
  name = "diff-trees-shell";

  inputsFrom = [
    checks.treefmt
  ];

  packages = [
    cargo
    rustc
    rust-analyzer
  ];
}
