use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use std::fs::File;
use std::io::{BufRead, BufReader};


enum UnixFile{
    FILE(UFile),
    FOLDER(UFolder)
}

struct UFile {
    name: String,
    parent: Weak<RefCell<UnixFile>>,
    size: usize,
}

struct UFolder {
    name: String,
    parent: Weak<RefCell<UnixFile>>,
    files: Vec<Rc<RefCell<UnixFile>>>,
}

impl UFile {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn to_string(&self, level: usize) {
        let mut res: String = "".to_string();

        for _ in 0..2*level {
            res.push(' ');
        }

        res.push_str(&*format!("{} {}", self.name, self.size));
        println!("{}", res);
    }
}

impl UFolder {
    pub fn size(&self) -> usize {
        let mut s: usize = 0;
        for u_file in &self.files {
            s +=  u_file.borrow().size();
        }
        s
    }

    pub fn to_string(&self, level: usize) {
        let mut res: String = "".to_string();

        for _ in 0..2*level {
            res.push(' ');
        }

        res.push_str(&*format!("+{} {}", self.name, self.size()));
        println!("{}", res);

        for u_file in &self.files {
            match &*u_file.borrow() {
                UnixFile::FOLDER(folder) => {folder.to_string(level + 1)},
                UnixFile::FILE(file) => {file.to_string(level + 1)}
            };
        }
    }
}

impl UnixFile {
    pub fn size(&self) -> usize {
        match &self {
            UnixFile::FILE(file) => file.size(),
            UnixFile::FOLDER(folder) => folder.size()
        }
    }

    pub fn from_string(ls_string: String) -> UnixFile {

        let splits = ls_string.split_whitespace();
        let words: Vec<&str> = splits.collect::<Vec<&str>>();

        if words[0] == "dir".to_string() {
            UnixFile::FOLDER(UFolder{
                name: words[1].to_owned(),
                files: vec![],
                parent: Weak::new()
            })
        } else{
            UnixFile::FILE(UFile{
                name: words[1].to_owned(),
                size: words[0].parse::<usize>().unwrap(),
                parent: Weak::new()
            })
        }
    }
}


struct Filter{
    free_size_needed: usize,
    size_to_delete: usize,
    folder_to_delete: String,
}

const MAX_SIZE: usize = 70000000;
const SIZE_TO_FREE: usize = 30000000;

impl Filter{

    fn walk(&mut self, root: Rc<RefCell<UnixFile>>){

        let mut cache_children = Vec::new();
        cache_children.push(root);

        while !cache_children.is_empty() {

            let popped_child:Rc<RefCell<UnixFile>> = cache_children.pop().unwrap();

            {
                let size = (*popped_child).borrow().size();

                if size >= self.free_size_needed && size <= self.size_to_delete {

                    if let UnixFile::FOLDER(f) = &*popped_child.borrow(){
                        println!("found small dir name {} size {}", f.name, size);
                        self.size_to_delete = size;
                        self.folder_to_delete = f.name.clone();
                    }

                }
            }

            if let UnixFile::FOLDER(folder) = &*(popped_child.borrow()) {
                for child in &folder.files {
                    cache_children.push(child.clone());
                }
            };
        }
    }
}

enum Command {
    CD(String),
    LS
}

impl Command {
    fn from_string(command_string: String) -> Command {
        let words: Vec<&str> = command_string.split_whitespace().collect::<Vec<&str>>();
        if words.len() > 2 {
            return Command::CD(words[2].to_owned());
        }
        return Command::LS;
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let root = Rc::new(RefCell::new(
        UnixFile::FOLDER(
            UFolder{
                name: String::from("root"),
                parent: Weak::new(),
                files: vec![],
            })));

    let mut current = root.clone();
    let mut state_idle = true;
    let mut cache_files: Vec<Rc<RefCell<UnixFile>>> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()){

        println!("{}", line);
        
        if !state_idle && line.chars().nth(0).unwrap() == '$' {

            if let UnixFile::FOLDER(folder) = &mut *(*current).borrow_mut() {
                    folder.files.clear();

                    println!("Add {} files to {}", cache_files.len(), folder.name);
                    folder.files.append(&mut cache_files);
                }

            state_idle = true;
        }

        if state_idle {
            let command = Command::from_string(line);
            match  command {
                Command::CD(path_input) => {

                    if path_input == "/".to_string() {
                        current = root.clone();
                    }
                    else if path_input == "..".to_string() {
                        if let UnixFile::FOLDER(folder) = &*current.clone().borrow() {
                            current = folder.parent.upgrade().unwrap();
                        }
                    } else {
                        if let UnixFile::FOLDER(folder) = &*current.clone().borrow() {

                            current = folder.files.iter().find(|r| {
                                if let UnixFile::FOLDER(inner_folder) = &*r.borrow() {
                                    inner_folder.name == path_input
                                } else{
                                    false
                                }
                            }
                            ).unwrap().clone();
                        }
                    }
                }
                Command::LS => {
                    state_idle = false;
                }
            }
        } else{
            let words: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

            if words[0] == "dir".to_string() {
                cache_files.push(
                    Rc::new(RefCell::new(UnixFile::FOLDER(UFolder{
                        name: words[1].to_owned(),
                        files: vec![],
                        parent: Rc::downgrade(&current)
                    }))));

            } else{
                cache_files.push(
                    Rc::new(RefCell::new(UnixFile::FILE(UFile{
                        name: words[1].to_owned(),
                        size: words[0].parse::<usize>().unwrap(),
                        parent: Rc::downgrade(&current)
                    }))));
            }
        }
    }

    if !state_idle {

        if let UnixFile::FOLDER(folder) = &mut *(*current).borrow_mut() {
            folder.files.clear();

            println!("Add {} files to {}", cache_files.len(), folder.name);
            folder.files.append(&mut cache_files);
        }
    }


    let current_free_size = MAX_SIZE - root.borrow().size();
    println!("root size {} current_free_size {}", root.borrow().size(), current_free_size);
    println!("size to free {}", SIZE_TO_FREE - current_free_size);

    let mut filter: Filter = Filter{
        free_size_needed: SIZE_TO_FREE - current_free_size,
        size_to_delete: root.borrow().size(),
        folder_to_delete: "root".to_string()};

    println!("Tree: ");
    if let UnixFile::FOLDER(f) = &*root.borrow(){
        f.to_string(0);
    }

    filter.walk(root);

    println!("to free name {} size: {}", filter.folder_to_delete, filter.size_to_delete);
}