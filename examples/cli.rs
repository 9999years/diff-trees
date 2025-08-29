use std::path::Path;

use diff_trees::Diff;
use diff_trees::DisplayDiffOpts;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("usage: diff-trees OLD_PATH NEW_PATH");
        return;
    }
    let old = &args[0];
    let new = &args[1];

    let diff = Diff::new(Path::new(old), Path::new(new)).unwrap();

    println!("{}", diff.display(DisplayDiffOpts::new().color(true)));
}
