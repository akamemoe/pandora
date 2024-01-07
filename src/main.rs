use std::{ffi::OsStr, fs, path::Path};
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

fn main() {
    println!("Process started");
    let args = std::env::args().collect::<Vec<String>>();
    let dirs = &args[1..];
    // println!("args = {:?}",dirs);

    for dir in dirs {
        let mut n = 1;
        for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
            let entry = entry.unwrap();
            println!("processing {:?}", entry.path());
            if entry.path().is_file() {
                renamer(&mut n, entry.path())
            }
        }
    }
}

fn renamer(n: &mut i32, path: &Path) -> () {
    let ext = path.extension().unwrap_or(OsStr::new(""));
    let ext = ext.to_str().unwrap().to_lowercase();
    let ext = ext.as_str();
    if IMAGE_EXTENSIONS.contains(&ext) {
        let img = image::open(path).unwrap();

        let name = format!("{:04}.{}x{}.{}", *n, img.width(), img.height(), ext);
        println!(
            "Original name {} -> {}",
            path.file_name().unwrap().to_str().unwrap(),
            name
        );
        let destpath = path.parent().unwrap().join(name);
        if let Err(e) = fs::rename(path, destpath) {
            eprintln!("Error:{}", e);
        }
        *n = *n + 1;
    }
}
