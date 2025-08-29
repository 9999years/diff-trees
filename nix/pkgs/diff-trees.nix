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

  cargoHash = "sha256-54+ElptE4wL+8yVdxB/6scOYUxeCrkflbMWAC/5s+Uw=";

  meta = {
    description = "directory tree diffs";
    homepage = "https://github.com/9999years/diff-trees";
    license = lib.licenses.mit;
    maintainers = [
      lib.maintainers._9999years
    ];
  };
}
