use std::ascii::escape_default;
use std::str;

use std::{fs, collections::VecDeque};

fn main() {
    let file = "input";
    part_1(file);
    part_2(file);
}

fn part_1(filename: &str) {

    let contents = fs::read_to_string(filename).expect("Could not read {filename}");
    let mut piles: Vec<VecDeque<&u8>>  = Vec::new();

    for _i in 0..9 {
        piles.push(VecDeque::new());
    }
    let mut piles_parsed = false;

    for line in contents.split("\n") {
        if line.is_empty() {
            piles_parsed = true;
            println!("Parsed containers");
        }
        else if piles_parsed {
            let instr = line.split(" ").collect::<Vec<&str>>();
            let pile1 = &mut piles[instr[3].parse::<usize>().unwrap() - 1];
            let mut pile_clone = VecDeque::new();
            for _i in 0..instr[1].parse::<i32>().unwrap() {
                //print!("{} ", escape_default(**pile1.back().unwrap()));
                pile_clone.push_back(pile1.pop_back().unwrap());
            }
            // for byte in &pile_clone {
            //     print!("{} ", escape_default(**byte));
            // }
            let pile2 = &mut piles[instr[5].parse::<usize>().unwrap() - 1];
            pile2.append(&mut pile_clone);
        } else {
            for i in (1..line.len()).step_by(4) {
                let pile: &mut VecDeque<&u8> = &mut piles[((i - 1) / 4) % 9];
                if line.as_bytes()[i].is_ascii_uppercase() {
                    pile.push_front(&line.as_bytes()[i]);
                }
            }
        }
        // println!();

        for pile in &piles {
            print!("{}", if !pile.back().is_none() { escape_default(**pile.back().unwrap()) } else { escape_default(" ".as_bytes()[0]) });
        }
        println!();
    }
}

fn part_2(filename: &str) {

    let contents = fs::read_to_string(filename).expect("Could not read {filename}");
    let mut piles: Vec<VecDeque<&u8>>  = Vec::new();

    for _i in 0..9 {
        piles.push(VecDeque::new());
    }
    let mut piles_parsed = false;

    for line in contents.split("\n") {
        if line.is_empty() {
            piles_parsed = true;
            println!("Parsed containers");
        }
        else if piles_parsed {
            let instr = line.split(" ").collect::<Vec<&str>>();
            let pile1 = &mut piles[instr[3].parse::<usize>().unwrap() - 1];
            let mut pile_clone = VecDeque::new();
            for _i in 0..instr[1].parse::<i32>().unwrap() {
                //print!("{} ", escape_default(**pile1.back().unwrap()));
                pile_clone.push_front(pile1.pop_back().unwrap());
            }
            // for byte in &pile_clone {
            //     print!("{} ", escape_default(**byte));
            // }
            let pile2 = &mut piles[instr[5].parse::<usize>().unwrap() - 1];
            pile2.append(&mut pile_clone);
        } else {
            for i in (1..line.len()).step_by(4) {
                let pile: &mut VecDeque<&u8> = &mut piles[((i - 1) / 4) % 9];
                if line.as_bytes()[i].is_ascii_uppercase() {
                    pile.push_front(&line.as_bytes()[i]);
                }
            }
        }
        println!();

        for pile in &piles {
            print!("{}", if !pile.back().is_none() { escape_default(**pile.back().unwrap()) } else { escape_default(" ".as_bytes()[0]) });
        }
        println!();
    }
}
