let
  sources = import ./npins;
  pkgs = import sources.nixpkgs {
    overlays = [
      (import ./nix/overlays/local-pkgs.nix)
    ];
  };
in
pkgs.diff-treesPackages.diff-trees.overrideAttrs (prev: {
  passthru = (prev.passthru or { }) // {
    inherit pkgs;
  };
})
