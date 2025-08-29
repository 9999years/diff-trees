final: prev: {
  diff-treesPackages = final.lib.packagesFromDirectoryRecursive {
    inherit (final) callPackage newScope;
    directory = ../pkgs;
  };

  diff-trees = final.diff-treesPackages.diff-trees;
}
