use std::fs;
use std::cmp;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap(); 

    let start = Instant::now();

    let mut count = 0;
    let mut overlap_cnt = 0;
    for pair in input.lines() {
        let elves: Vec<&str> = pair.split(",").collect();
        let first_elf_rng: Vec<i32> = elves[0].split("-")
            .map(|x| x.parse::<i32>().unwrap()).collect();
        let second_elf_rng: Vec<i32> = elves[1].split("-")
            .map(|x| x.parse::<i32>().unwrap()).collect();
        let first_elf_len = find_len(&first_elf_rng);
        let second_elf_len = find_len(&second_elf_rng);

        if has_overlap(&first_elf_rng, &second_elf_rng) {
            overlap_cnt += 1;
        }

        if first_elf_len >= second_elf_len {
            if first_elf_rng[0] <= second_elf_rng[0] && first_elf_rng[1] >= second_elf_rng[1] {
                count += 1;
            }
        } else {
            if second_elf_rng[0] <= first_elf_rng[0] && second_elf_rng[1] >= first_elf_rng[1] {
                count += 1;
            }
        }
    }
    let duration = start.elapsed();
    println!("duration: {:?}", duration);

    println!("count: {count}");
    println!("overlap_cnt: {overlap_cnt}");
}

fn find_overlap(rng1: &Vec<i32>, rng2: &Vec<i32>) -> i32 {
    cmp::max(0, cmp::min(rng1[1], rng2[1]) - cmp::max(rng1[0], rng2[0]) + 1)
}

fn has_overlap(rng1: &Vec<i32>, rng2: &Vec<i32>) -> bool {
    if find_overlap(rng1, rng2) == 0 { return false; } else { return true; }
}

fn find_len(rng: &Vec<i32>) -> i32 {
    rng[1] - rng[0]
}
