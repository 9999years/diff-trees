{
  localSystem ? builtins.currentSystem,
}:
let
  sources = import ./npins;
  pkgs = import sources.nixpkgs {
    inherit localSystem;
    overlays = [
      (import ./nix/overlays/local-pkgs.nix)
    ];
  };
in
pkgs.diff-trees-pkgs.diff-trees.overrideAttrs (prev: {
  passthru = (prev.passthru or { }) // {
    inherit pkgs;

    inherit (pkgs.diff-trees-pkgs)
      diff-trees
      checks
      shell
      ;
  };
})
