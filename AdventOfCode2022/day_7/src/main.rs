use std::collections::VecDeque;
use std::fs;

fn main() {
    let file = "input";
    part_1(file);
}

fn part_1(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut totSum = 0;
    let mut curSum = 0;
    let mut depth = 0;

    for line in contents.split("\n") {
        let lineb = line.as_bytes();

        if lineb[0] == b'$' {
            if lineb[2] == b'c' {
                if lineb[5..7] == *"..".as_bytes() {
                    depth -= 1

                }
                else if !(lineb[5] == b'/') {
                    depth += 1;
                }
            } else if lineb[0].is_ascii_digit() {

            }
        }
    }
}

fn part_2(filename: &str) {}

