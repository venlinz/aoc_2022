use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap(); 
    let mut max: usize = 0;
    let mut cur_elf: usize = 0;
    let mut top3: [usize; 3] = [0, 0, 0];
    for line in input.split("\n") {
        if line.trim().len() == 0 {
            if max < cur_elf {
                max = cur_elf;
            }
            determine_top3(&mut top3, cur_elf);
            cur_elf = 0;
        } else {
            cur_elf += line.parse::<usize>().unwrap();
        }
    }
    println!("max: {max}");
    let mut top3_combined: usize = 0;
    for (i, elem) in top3.iter().enumerate() {
        println!("{i}: {}", elem);
        top3_combined += elem;
    }
    println!("top3 combined snacks' calories: {top3_combined}");
}

fn determine_top3(top3: &mut [usize; 3], cur_num: usize) {
    if top3[2] >= cur_num {
        return;
    } else if top3[0] < cur_num {
        let t: usize = top3[1];
        top3[1] = top3[0];
        top3[0] = cur_num;
        top3[2] = t;
    } else if top3[1] < cur_num {
        top3[2] = top3[1];
        top3[1] = cur_num;
    } else {
        top3[2] = cur_num;
    }
}
