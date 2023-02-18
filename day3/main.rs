use std::fs;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .unwrap();

    let start = Instant::now();
    part1(input.clone());
    part2(input);
    let duration = start.elapsed();
    println!("duration: {:?}", duration);
}

// Find the item type that appears in both compartments of each rucksack.
// What is the sum of the priorities of those item types?
fn part1(input: String) {
    let mut sum = 0;
    for rucksack in input.lines() {
        let item_priority = match find_common_item_in_compartments(rucksack.to_string()) {
            Some(x) => get_item_priority(x),
            None => 0,
        };
        sum += item_priority;
    }
    println!("new part1 sum: {sum}");
}

fn find_common_item_in_compartments(rucksack: String) -> Option<char> {
    let (compartment1, compartment2) = rucksack.split_at(rucksack.len()/2);
    // println!("{}, {}", compartment1, compartment2);
    for c in compartment1.chars() {
        if compartment2.contains(c) {
            return Some(c);
        }
    }
    None
}

fn get_item_priority(item: char) -> i32 {
    const LOWER_CASE: i32 = 96;
    const UPPER_CASE_OFFSET: i32 = 26;
    const UPPER_CASE: i32 = 64;
    if item.is_ascii_lowercase() {
        item as i32 - LOWER_CASE
    } else {
        item as i32 - UPPER_CASE + UPPER_CASE_OFFSET
    }
}

// Find the item type that corresponds to the badges of each three-Elf group.
// What is the sum of the priorities of those item types?
fn part2(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let common_among_group_of_3 = get_intersect_of_3(
            lines[i].chars().collect(),
            lines[i + 1].chars().collect(),
            lines[i + 2].chars().collect());
        sum += get_item_priority(common_among_group_of_3);
    }
    println!("sum part2: {sum}");
}

fn get_intersect_of_3(sack1: HashSet<char>, sack2: HashSet<char>, sack3: HashSet<char>) -> char {
    let intersect_s1_s2: HashSet<char> = sack1.intersection(&sack2).into_iter().cloned().collect::<HashSet<char>>();
    let intersect_of_all_3 = intersect_s1_s2.intersection(&sack3);
    return intersect_of_all_3.into_iter().next().unwrap().clone();
}
