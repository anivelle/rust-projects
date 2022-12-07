use std::fs;

fn main() {
    let filename = "input";
    part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut sum: i32 = 0;

    for line in lines {
        if !line.is_empty() {
            let line_b = line.as_bytes();
            let length = line_b.len();
            let line_1 = &line_b[..length / 2];
            let line_2 = &line_b[(length / 2)..length];

            assert_eq!(line_1.len(), line_2.len());
            for byte in line_2 {
                if line_1.contains(byte) {
                    //println!("{}: {} ", char::from(*byte), find_value(*byte));
                    sum = sum
                        .checked_add(find_value(*byte).into())
                        .expect("Could not add");
                    break;
                }
            }
        }
    }
    println!("{sum}");
}

fn part_2(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut buf: Vec<u8> = Vec::new();
    let mut sum: i32 = 0;
    let mut linebuf: Vec<&[u8]> = Vec::new();

    for line in lines {
        if linebuf.len() < 2 {
            linebuf.push(line.as_bytes());
        } else {
            for byte in *linebuf.get(0).expect("Couldn't get array 1") {
                if linebuf.get(1).expect("Couldn't get array").contains(byte) {
                    buf.push(*byte);
                }
            }
            for byte in line.as_bytes() {
                if buf.contains(byte) {
                    println!("{}: {}", char::from(*byte), find_value(*byte));
                    sum = sum
                        .checked_add(find_value(*byte).into())
                        .expect("Could not add");
                }
            }
            buf.clear();
            linebuf.clear();
        }
    }
    println!("{sum}");
}
fn find_value(input: u8) -> u8 {
    return if input.is_ascii_uppercase() {
        input - 38
    } else {
        input - 96
    };
}
