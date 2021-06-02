use std::fs;
use std::io::*;
use std::path::Path;

struct UFile {
    name: fs::DirEntry,
    content: Option<Vec<Box<UFile>>>,
    is_dir: bool,
}

impl UFile {
    pub fn new(name: fs::DirEntry) -> Self {
        UFile {
            name,
            content: None,
            is_dir: false,
        }
    }
}

/*
fn ufiles_get(path: fs::ReadDir) -> UFile {
    UFile::new()
}
*/
fn files_get(paths: fs::ReadDir) -> Option<Vec<fs::DirEntry>> {
    let mut res: Vec<fs::DirEntry> = Vec::new();
    for path in paths {
        match path {
            Ok(val) => res.push(val),
            Err(_) => continue,
        };
    }
    Some(res)
}

fn files_print(files: &Vec<fs::DirEntry>) {
    for file in files {
        println!("{:#?}", file.path().display());
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    let len_withoutcrlf = buffer.trim_end().len();
    buffer.truncate(len_withoutcrlf);
    let dir = Path::new(&buffer);
    let paths = fs::read_dir(dir)?; // try avoiding ?, turn into match

    let x: Vec<fs::DirEntry> = match files_get(paths) {
        Some(val) => val,
        None => vec![],
    };

    files_print(&x);

    Ok(())
}
