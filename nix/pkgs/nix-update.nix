{ pkgs, fetchpatch }:
pkgs.nix-update.overrideAttrs (drv: {
  patches = (drv.patches or [ ]) ++ [
    # `--no-src`: init
    #
    # See: https://github.com/Mic92/nix-update/pull/436
    (fetchpatch {
      url = "https://github.com/Mic92/nix-update/commit/9a5f2939953e9f730231f21a97d8027360c75446.patch";
      hash = "sha256-O2GLHP+d3cA9HwvwYPAexxqA/rSOYkm09FdivVMdjL8=";
    })
  ];
})
