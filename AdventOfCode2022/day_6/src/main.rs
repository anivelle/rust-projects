use std::fs;
use std::collections::VecDeque;
use std::ascii::escape_default;
fn main() {
    let file = "input";
    part_1(file);
    part_2(file);

}

fn part_1(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut buf: VecDeque<u8> = VecDeque::new();
    let mut tester: Vec<u8> = Vec::new();

    for i in 0..contents.len() {
        let byte = contents.as_bytes()[i];
        for i in 0..buf.len() { print!("{} ", escape_default(buf[i])) }
        println!();

        buf.push_back(byte);
        if buf.len() > 4{
            buf.pop_front();
            for j in 0..buf.len() {
                if tester.clone().into_iter().position(|x| x == buf[j]).is_none() {
                    println!("Here");
                    if j == buf.len() - 1 {
                        print!("{} {}", i, j);
                        println!("{}", i + 1);
                    }
                    tester.push(buf[j]);
                } else {
                    break;
                }
            }
            if tester.len() == buf.len() {
                break;
            }
            tester.clear();
        }
    }
}

fn part_2(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut buf: VecDeque<u8> = VecDeque::new();
    let mut tester: Vec<u8> = Vec::new();

    for i in 0..contents.len() {
        let byte = contents.as_bytes()[i];
        for i in 0..buf.len() { print!("{} ", escape_default(buf[i])) }
        println!();

        buf.push_back(byte);
        if buf.len() > 14{
            buf.pop_front();
            for j in 0..buf.len() {
                if tester.clone().into_iter().position(|x| x == buf[j]).is_none() {
                    println!("Here");
                    if j == buf.len() - 1 {
                        println!("{}", i + 1);
                    }
                    tester.push(buf[j]);
                } else {
                    break;
                }
            }
            if tester.len() == buf.len() {
                break;
            }
            tester.clear();
        }
    }
}
