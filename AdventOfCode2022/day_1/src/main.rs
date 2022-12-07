use std::fs;
use std::collections::VecDeque;

fn main() {
    let filename = "input_1";
    part_1(&filename);
    part_2(&filename);
}

fn part_1(filename: &str) {
    let mut max = 0;
    let mut counter = 0;
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let split = contents.split("\n");
    for number in split {
        if number.eq("\n") || number.is_empty() {
            if counter > max {
                max = counter;
            }
            counter = 0;
        } else {
            counter += number.parse::<i32>().expect("Could not convert int");
        }
    }

    println!("{max}");
}

fn part_2(filename: &str) {
    let mut max: VecDeque<i32> = VecDeque::from([0, 0, 0]);
    let mut counter = 0;
    let mut total = 0;
    let mut iter;

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    let split = contents.split("\n");

    for number in split {
        if number.eq("\n") || number.is_empty() {
            iter = max.iter();
            for num in iter {
                println!("{}", *num);
                if *num < counter {
                    println!("{counter} > {}", *num);
                    max.insert(max.iter().position(|&x| x == *num).unwrap(), counter);
                    total += counter;
                    if max.len() > 3{
                        let min = max.pop_back().unwrap();
                        total -= min;
                    }
                    break;
                }
            }
            counter = 0;
        } else {
            counter += number.parse::<i32>().expect("Could not convert int");
        }
    }

    println!("{total}");

}
