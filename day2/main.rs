use std::fs;
use std::time::Instant;
use std::time::Duration;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut duration1 = Duration::new(0,0);
    let mut duration2 = Duration::new(0,0);
    for _ in 0..1000 {
        let start1 = Instant::now();
        let input1 = input.clone();
        part1(input1);
        duration1 += start1.elapsed();
        // print_type(&duration1);
        let start2 = Instant::now();
        let input2 = input.clone();
        part2(input2);
        duration2 += start2.elapsed();
    }
    println!("duration: {:?}", duration1);
    println!("duration: {:?}", duration2);
}

fn part1(input: String) {
    let my_defeat = vec![2, 3, 1];

    const MY_CHOICE_OFFSET: usize = 87;
    const OPP_CHOICE_OFFSET: usize = 64;
    let mut score: usize = 0;
    for line in input.lines() {
        let choices: Vec<&str> = line.split_whitespace().collect();
        let opponent_choice = choices.get(0).unwrap();
        let my_choice = choices.get(1).unwrap();
        let oc = opponent_choice.chars().next().unwrap() as usize - OPP_CHOICE_OFFSET;
        let mc = my_choice.chars().next().unwrap() as usize - MY_CHOICE_OFFSET;

        score += mc;
        if mc == oc {
            score += 3;
        } else if *my_defeat.get(mc - 1).unwrap() != oc {
            score += 6;
        }
    }

    // println!("score part1: {score}");
}

fn part2(input: String) {
    const MY_CHOICE_OFFSET: usize = 87;
    const OPP_CHOICE_OFFSET: usize = 64;

    const LOSE: usize = 1;
    const DRAW: usize = 2;

    let mut score: usize = 0;
    for line in input.lines() {
        let choices: Vec<&str> = line.split_whitespace().collect();
        let opponent_choice = choices.get(0).unwrap();
        let my_choice = choices.get(1).unwrap();
        let oc = opponent_choice.chars().next().unwrap() as usize - OPP_CHOICE_OFFSET;
        let mc = my_choice.chars().next().unwrap() as usize - MY_CHOICE_OFFSET;

        if mc == DRAW {
            score += oc;
            score += 3;
        } else if mc == LOSE {
            score += if oc == 1 {3} else {oc - 1};
        } else {
            score += if oc == 3 {1} else {oc + 1};
            score += 6;
        }
    }

    // println!("score part 2: {score}");
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
