final: prev: {
  diff-trees-pkgs = final.lib.packagesFromDirectoryRecursive {
    inherit (final) callPackage newScope;
    directory = ../pkgs;
  };

  diff-trees = final.diff-trees-pkgs.diff-trees;
}
