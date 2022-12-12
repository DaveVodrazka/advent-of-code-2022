use std::fs;

type Parent = Box<Folder>;

type FolderList = Vec<Box<Folder>>;

type FileList = Vec<Box<File>>;

#[derive(Debug)]
struct Folder {
    parent: Option<Parent>,
    files: Option<FileList>,
    sub_folders: Option<FolderList>,
}

impl From<Folder> for Option<Vec<Box<Folder>>> {
    fn from(folder: Folder) -> Self {
        Some(vec![Box::new(folder)])
    }
}

impl Folder {
    fn new() -> Self {
        Folder {
            parent: None,
            files: None,
            sub_folders: None,
        }
    }
    fn add_file(mut self: &mut Self, name: &str, size: &str) {
        let parsed_size: u32 = match size.parse() {
            Ok(v) => v,
            Err(_) => panic!("Failed to parse file size"),
        };
        let file = Box::new(File {
            name: String::from(name),
            size: parsed_size,
        });

        let updated_files: Option<FileList> = match self.files {
            Some(v) => Vec::from(v),
            None => File::from(*file),
        };
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl From<File> for Option<FileList> {
    fn from(file: File) -> Self {
        Some(vec![Box::new(file)])
    }
}

fn get_input_content() -> String {
    let file_path = "./puzzle7/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn append_file(folder: &Folder, name: &str, size: &str) {
    let parsed_size: u32 = match size.parse() {
        Ok(v) => v,
        Err(_) => panic!("Failed to parse file size"),
    };
    let file = File {
        name: String::from(name),
        size: parsed_size,
    };

    match folder.files {
        Some(v) => v.push(Box::new(file)),
        None => {}
    }
}

fn main() {
    let input = get_input_content();

    let mut root = Folder {
        parent: None,
        files: None,
        sub_folders: None,
    };

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "$" => {
                let command = parts[1];
                println!("command {command}");
            }
            "dir" => {
                let dirname = parts[1];
                println!("directory {dirname}");
            }
            _ => {
                let (size, filename) = (parts[0], parts[1]);
                println!("file {filename} with size {size}");
            }
        }
    }
}
