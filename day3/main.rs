use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .unwrap();
    let mut sum = 0;
    let mut sum_part2 = 0;

    const LOWER_CASE: i32 = 96;
    const UPPER_CASE_OFFSET: i32 = 26;
    const UPPER_CASE: i32 = 64;
    const GROUP_SIZE: usize = 3;
    let mut first_elf: HashSet<char> = HashSet::new();
    let mut second_elf: HashSet<char> = HashSet::new();
    let mut third_elf: HashSet<char> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        let modder = i + 1;

        let mut line = line.to_string();
        let mut compartment2 = line.split_off(line.len()/2);
        let compartment1 = line;

        // part 1
        for c in compartment1.chars() {
            if modder % GROUP_SIZE == 1 {
                first_elf.insert(c);
            } 
            if modder % GROUP_SIZE == 2 {
                second_elf.insert(c);
            } 
            if modder % GROUP_SIZE == 0 {
                third_elf.insert(c);
            }
            if compartment2.contains(c) {
                if c.is_ascii_lowercase() {
                    let t = c as i32 - LOWER_CASE;
                    sum += t;
                } else {
                    let t = c as i32 - UPPER_CASE + UPPER_CASE_OFFSET;
                    sum += t;
                }
                compartment2 = compartment2.replace(c, " ");
            }
        }

        // part2
        for c in compartment2.chars() {
            if modder % GROUP_SIZE == 1 {
                first_elf.insert(c);
            } 
            if modder % GROUP_SIZE == 2 {
                second_elf.insert(c);
            } 
            if modder % GROUP_SIZE == 0 {
                third_elf.insert(c);
            }
        }

        if modder % GROUP_SIZE == 0 {
            for c in &first_elf {
                if !second_elf.contains(&c) {
                    continue;
                } 
                if !third_elf.contains(&c) {
                    continue;
                }
                if *c == ' ' {
                    continue;
                }
                if c.is_ascii_lowercase() {
                    let t = *c as i32 - LOWER_CASE;
                    sum_part2 += t;
                } else {
                    let t = *c as i32 - UPPER_CASE + UPPER_CASE_OFFSET;
                    sum_part2 += t;
                }
            }
            first_elf = HashSet::new();
            second_elf = HashSet::new();
            third_elf = HashSet::new();
        }
    }
    println!("sum: {sum}");
    println!("sum part2: {sum_part2}");
}
