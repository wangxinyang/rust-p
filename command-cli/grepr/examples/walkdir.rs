use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
    for entry in WalkDir::new("src").follow_links(true) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}
