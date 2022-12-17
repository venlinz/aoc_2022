use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut scores = HashMap::new();
    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let mut choice_me_to_opp = HashMap::new();
    choice_me_to_opp.insert("X", "A");
    choice_me_to_opp.insert("Y", "B");
    choice_me_to_opp.insert("Z", "C");

    let mut choice_opp_to_me = HashMap::new();
    choice_opp_to_me.insert("A", "X");
    choice_opp_to_me.insert("B", "Y");
    choice_opp_to_me.insert("C", "Z");

    let mut my_defeat = HashMap::new();
    my_defeat.insert("X", "B");
    my_defeat.insert("Y", "C");
    my_defeat.insert("Z", "A");

    let mut my_defeat_opp_pers = HashMap::new();
    my_defeat_opp_pers.insert("B", "X");
    my_defeat_opp_pers.insert("C", "Y");
    my_defeat_opp_pers.insert("A", "Z");

    let mut my_win = HashMap::new();
    my_win.insert("A", "Y");
    my_win.insert("B", "Z");
    my_win.insert("C", "X");


    const LOSE: &str = "X";
    const DRAW: &str = "Y";

    let mut my_score: usize = 0;
    let mut my_score_part2: usize = 0;
    for line in input.lines() {
        let choices: Vec<&str> = line.split_whitespace().collect();
        let opponent_choice = choices.get(0).unwrap();
        let my_choice = choices.get(1).unwrap();

        my_score += scores.get(my_choice).unwrap();
        if choice_me_to_opp.get(my_choice).unwrap() == opponent_choice {
            my_score += 3;
        } else if my_defeat.get(my_choice).unwrap() != opponent_choice {
            my_score += 6;
        }

        if my_choice == &DRAW {
            my_score_part2 += scores.get(choice_opp_to_me.
                get(opponent_choice).unwrap())
                .unwrap();
            my_score_part2 += 3;
        } else if my_choice == &LOSE {
            my_score_part2 += scores.get(my_defeat_opp_pers
                .get(opponent_choice).unwrap()).unwrap();
        } else {
            my_score_part2 += scores.get(my_win
                .get(opponent_choice).unwrap()).unwrap();
            my_score_part2 += 6;
        }
    }
    println!("my score: {my_score}");
    println!("my score part 2: {my_score_part2}");
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
