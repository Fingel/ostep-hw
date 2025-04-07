use std::{
    env::args,
    os::{linux::fs::MetadataExt, unix::fs::PermissionsExt},
    path::Path,
};

fn print_stats(path: &Path) {
    println!("Size: {} bytes", path.metadata().unwrap().st_size());
    println!("Owner: {}", path.metadata().unwrap().st_uid());
    println!("Permissions: {:?}", path.metadata().unwrap().permissions());
    println!("{}", "-".repeat(20));
}
fn main() {
    let verbose = args().nth(1).unwrap_or("".to_string()) == "-l";
    let p = args().nth(2).unwrap_or(".".to_string()).to_string();
    let path = Path::new(&p);
    if !path.exists() {
        eprintln!("No such file or directory");
        std::process::exit(1);
    }
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
        if verbose {
            print_stats(&entry.path());
        }
    }
}
