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

  cargoHash = "sha256-bPyc59c7WmlQeHMW/wnRz5u0cC+abHQHY1L847e7e9U=";

  meta = {
    description = "directory tree diffs";
    homepage = "https://github.com/9999years/diff-trees";
    license = lib.licenses.mit;
    maintainers = [
      lib.maintainers._9999years
    ];
  };
}
