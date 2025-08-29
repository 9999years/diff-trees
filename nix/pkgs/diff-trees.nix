{
  lib,
  rustPlatform,
  src,
  cargo-toml,
}:

rustPlatform.buildRustPackage {
  pname = "diff-trees";
  version = cargo-toml.package.version;

  inherit src;

  cargoHash = "sha256-SR6EFOnwOkP56jyaiIWVnWNJE+8++XvqLa3bPKq0Wuo=";

  meta = {
    description = "directory tree diffs";
    homepage = "https://github.com/9999years/diff-trees";
    license = lib.licenses.mit;
    maintainers = [
      lib.maintainers._9999years
    ];
  };
}
