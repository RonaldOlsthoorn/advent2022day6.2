use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::os::linux::raw::stat;


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
}

impl UFolder {
    pub fn size(&self) -> usize {
        let mut s: usize = 0;
        for uFile in &self.files {
            s +=  uFile.borrow().size();
        }
        s
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
    total_size: usize
}

const MAX_SIZE: usize = 100000;

impl Filter{

    fn walk(mut self, root: Rc<RefCell<UnixFile>>){

        let mut cache_children = Vec::new();

        while !cache_children.is_empty() {

            let mut popped_child:Rc<RefCell<UnixFile>> = cache_children.pop().unwrap();

            {
                let size = (*popped_child).borrow().size();

                if size < MAX_SIZE {
                    self.total_size += size;
                }
            }

            match &*(popped_child.borrow()){
                UnixFile::FILE(file) => {}
                UnixFile::FOLDER(folder) => {
                    for child in &folder.files{
                        cache_children.push(child.clone());
                    }
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
        let mut words: Vec<&str> = command_string.split_whitespace().collect::<Vec<&str>>();
        if words.len() > 1 {
            return Command::CD(words[0].to_owned());
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

        if !state_idle && line.chars().nth(0).unwrap() == '$' {

            let mut f = &*(*current).borrow_mut();

            match f {
                UnixFile::FOLDER(mut folder) => {
                    folder.files.append(&mut cache_files);
                },
                UnixFile::FILE(file) => {}
            }


            state_idle = false;
        }

        if state_idle {
            let command = Command::from_string(line);
            match  command {
                Command::CD(String) => {
                    
                },
                Command::LS => {
                    state_idle = false;
                }
            }
        } else{
            cache_files.push(Rc::new(RefCell::new(
                UnixFile::from_string(line)
            )));
        }
    }
}