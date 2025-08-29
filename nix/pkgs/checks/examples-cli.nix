{
  mkCheck,
  writeShellScriptBin,
}:

let
  old = writeShellScriptBin "old" "old";
  new = writeShellScriptBin "new" "new";
in

mkCheck {
  name = "examples-cli";

  checkPhase = ''
    cargo run --example cli -- ${old} ${new}
  '';
}
