use std::fs;

struct Entry {
    name: String,
    path: String,
    size: u64,
    is_dir: bool,
}

impl Entry {
    fn new(name: String, path: String, size: u64, is_dir: bool) -> Entry {
        Entry {
            name,
            path,
            size,
            is_dir,
        }
    }

    fn display(&self) {
        println!(
            "{} {} {} {}\n",
            self.name, self.path, self.size, self.is_dir
        );
    }
}

fn get_entries(path: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let size = fs::metadata(&path).unwrap().len();
        let is_dir = fs::metadata(&path).unwrap().is_dir();
        entries.push(Entry::new(
            name,
            path.to_str().unwrap().to_string(),
            size,
            is_dir,
        ));
    }
    entries
}

fn main() {
    for entry in get_entries("/home/monarch/") {
        print!("{}\n", entry.name);
    }
}
