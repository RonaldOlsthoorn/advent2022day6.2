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

        if deque.len() > 14 {
            deque.pop_back();
        }

        if deque.len() == 14 {

            let mut unique = true;

            for j in 0..14 {
                for k in j+1..14 {
                    unique &= deque[j] != deque[k];
                    if !unique {
                        println!("j {} k {} deque[j] {} deque[k] {}", j, k, deque[j], deque[k]);
                        break;
                    }
                }

                if !unique {
                    break;
                }
            }

            if unique {
                let mut print_out = format!("found at i {}, ", i);

                for c in deque {
                    print_out.push(c);
                }
                println!("{}", print_out);
                break;
            }
        }
    }
}