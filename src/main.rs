use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet, VecDeque};


fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut emptyLineNumber = 0;

    for (i, line) in reader.lines().enumerate() {
        if line.unwrap().is_empty() {
            emptyLineNumber = i;
        }
    }

    let reader = BufReader::new(File::open("input.txt").unwrap());
    let lines: Vec<String>= reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let line: &String = &lines[0];

    let mut deque = VecDeque::new();

    for (i, new_character) in line.chars().enumerate() {


        deque.push_front(new_character);

        if deque.len() > 4 {
            deque.pop_back();
        }

        if deque.len() == 4 {
            //println!("deque {}{}{}{}", deque[0], deque[1], deque[2], deque[3]);

            if deque[0] != deque[1] && deque[0] != deque[2] && deque[0] != deque[3] && deque[1] != deque[2] && deque[1] != deque[3] && deque[2] != deque[3]{
                println!("found at i {}, {}{}{}{}", i, deque[0], deque[1], deque[2], deque[3]);
                break;
            }
        }
    }
}