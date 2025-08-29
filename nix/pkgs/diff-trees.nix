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

  cargoHash = "sha256-sRgkUHY1wWH98gZIsXa0eK5k8U8eQkktXxlpz54N9RU=";

  meta = {
    description = "directory tree diffs";
    homepage = "https://github.com/9999years/diff-trees";
    license = lib.licenses.mit;
    maintainers = [
      lib.maintainers._9999years
    ];
  };
}
