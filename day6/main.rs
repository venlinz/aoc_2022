use std::{fs, process};
use std::time::Instant;

fn main() {
    let file_path;
    if 2 == 1 {
        file_path = "./example.txt";
    } else {
        file_path = "./input.txt";
    }
    let input = fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("ERROR: unable to read the file {file_path}, {e}");
        process::exit(1);
    });
    let input = input.trim().to_string();

    println!("part1: {}", find_first_distinct_set_end(&input, 4));
    println!("part2: {}", find_first_distinct_set_end(&input, 14));
}

fn find_first_distinct_set_end(input: &String, distinct_set_length: usize) -> usize {
    let start = Instant::now();
    let mut packets = Vec::new();
    let mut counter = 0;
    for packet in input.chars() {
        counter += 1;
        packets.push(packet);

        if packets.len() == distinct_set_length && is_random_sequence(packets.clone()) {
            packets.remove(0);
        }

        if packets.len() == distinct_set_length {
            break;
        }
    }
    let duration = start.elapsed();
    println!("duration: {:?}", duration);
    return counter;
}

fn is_random_sequence(mut packets: Vec<char>) -> bool {
    let inital_len = packets.len();
    packets.sort();
    packets.dedup();
    if inital_len > packets.len() {
        return true;
    }
    return false;
}
