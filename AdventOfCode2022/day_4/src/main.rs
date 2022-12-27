use std::fs;

fn main() {
    let file = "input";
    part_1(file);
    part_2(file);
}

fn part_1(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let lines = contents.split("\n");
    let mut sum = 0;

    let mut vals: Vec<&str> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            let pairs: Vec<&str> = line.split(",").collect();
            for pair in pairs {
                vals.append(&mut pair.split("-").collect::<Vec<&str>>());
            }

            print!(
                "{}-{},{}-{}",
                vals.get(0).unwrap(),
                vals.get(1).unwrap(),
                vals.get(2).unwrap(),
                vals.get(3).unwrap()
            );
            if (vals.get(0).unwrap().parse::<i32>().unwrap()
                <= vals.get(2).unwrap().parse::<i32>().unwrap())
                && (vals.get(1).unwrap().parse::<i32>().unwrap()
                    >= vals.get(3).expect("3").parse::<i32>().unwrap())
            {
                sum += 1;
                println!(": 1");
            } else if (vals.get(0).unwrap().parse::<i32>().unwrap()
                >= vals.get(2).unwrap().parse::<i32>().unwrap())
                && (vals.get(1).unwrap().parse::<i32>().unwrap()
                    <= vals.get(3).unwrap().parse::<i32>().unwrap())
            {
                sum += 1;
                println!(": 2");
            } else {
                println!(": 0");
            }
        }
        vals.clear();
    }

    println!("{sum}");
}

fn part_2(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let lines = contents.split("\n");
    let mut sum = 0;

    let mut vals: Vec<&str> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            let pairs: Vec<&str> = line.split(",").collect();
            for pair in pairs {
                vals.append(&mut pair.split("-").collect::<Vec<&str>>());
            }

            print!(
                "{}-{},{}-{}",
                vals.get(0).unwrap(),
                vals.get(1).unwrap(),
                vals.get(2).unwrap(),
                vals.get(3).unwrap()
            );
            if (vals.get(0).unwrap().parse::<i32>().unwrap()
                < vals.get(2).unwrap().parse::<i32>().unwrap())
                && (vals.get(1).unwrap().parse::<i32>().unwrap()
                    < vals.get(2).expect("3").parse::<i32>().unwrap())
            {
                println!(": 1");
            } else if (vals.get(0).unwrap().parse::<i32>().unwrap()
                > vals.get(2).unwrap().parse::<i32>().unwrap())
                && (vals.get(0).unwrap().parse::<i32>().unwrap()
                    > vals.get(3).unwrap().parse::<i32>().unwrap())
            {
                println!(": 2");
            } else {
                sum += 1;
                println!(": 0");
            }
        }
        vals.clear();
    }

    println!("{sum}");
}
