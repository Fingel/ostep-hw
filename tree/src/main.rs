use std::fs::read_dir;
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, depth: u32) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!(
                "{:indent$}{}",
                "",
                path.file_name().unwrap().to_str().unwrap(),
                indent = depth as usize * 4
            );
            if path.is_dir() {
                visit_dirs(&path, depth + 1)?;
            }
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    visit_dirs(Path::new("."), 0)?;
    Ok(())
}
