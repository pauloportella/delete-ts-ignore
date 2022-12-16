#[cfg(test)]
mod tests;

use std::env;
use std::fs;
use std::path::Path;
use std::io::Read;


fn delete_ts_ignore(dir: &Path) -> usize {
    let mut count = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                count += delete_ts_ignore(&path);
            } else if path.extension() == Some(std::ffi::OsStr::new("ts")) || path.extension() == Some(std::ffi::OsStr::new("tsx")) {
                let mut lines = String::new();
                fs::File::open(&path)
                    .unwrap()
                    .read_to_string(&mut lines)
                    .unwrap();
                let num_lines = lines.split('\n').count();
                let lines: Vec<&str> = lines.split('\n').collect();
                let lines: Vec<&str> = lines.into_iter().filter(|line| !line.contains("@ts-expect-error") && !line.contains("@ts-ignore")).collect();
                let new_contents = lines.join("\n");
                fs::write(path, new_contents).unwrap();
                count += num_lines - lines.len();
            }
        }
    }
    count
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: delete-ts-ignore --path <path>");
        return;
    }
    let path = &args[2];
    let count = delete_ts_ignore(Path::new(path));
    println!("Deleted {} lines.", count);
}
