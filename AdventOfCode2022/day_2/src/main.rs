use std::fs;

const SCORES: [[i32; 3]; 3] = [[4, 1, 7], [8, 5, 2], [3, 9, 6]];
const SCORES_P2: [[i32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
const MY_HAND: [&str; 3] = ["A", "B", "C"];
const OPP_HAND: [&str; 3] = ["X", "Y", "Z"];

fn main() {
    let filename = "input";
    part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let mut score = 0;
    let contents = fs::read_to_string(&filename).expect("Should have been able to read the file");

    for line in contents.split("\n") {
        if !line.is_empty(){
        let letters: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let score_test = SCORES
            .get(
                OPP_HAND
                    .iter()
                    .position(|&x| x == *letters.get(1).expect("letters[0] is None"))
                    .expect("Could not get your hand"),
            )
            .expect("Could not get indexed array")
            .get(
                MY_HAND
                    .iter()
                    .position(|&x| x == *letters.get(0).expect("letters[1] is None"))
                    .expect("Could not get opponent's hand"),
            )
            .expect("Could not get integer");
        score += score_test;
        println!("{score_test}");
        }
    }
    println!("{score}");
}

fn part_2(filename: &str) {
    let mut score = 0;
    let contents = fs::read_to_string(&filename).expect("Should have been able to read the file");

    for line in contents.split("\n") {
        if !line.is_empty(){
        let letters: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let score_test = SCORES_P2
            .get(
                MY_HAND
                    .iter()
                    .position(|&x| x == *letters.get(0).expect("letters[0] is None"))
                    .expect("Could not get your hand"),
            )
            .expect("Could not get indexed array")
            .get(
                OPP_HAND
                    .iter()
                    .position(|&x| x == *letters.get(1).expect("letters[1] is None"))
                    .expect("Could not get opponent's hand"),
            )
            .expect("Could not get integer");
        score += score_test;
        }
    }
    println!("{score}");
}
