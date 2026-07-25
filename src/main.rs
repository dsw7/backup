mod run_backup;

use run_backup::run_backup;

fn main() {
    let src = String::from("/tmp/bar/");
    let dst = String::from("dsw@10.0.0.115:/tmp/bar");
    run_backup(&src, &dst);
}
