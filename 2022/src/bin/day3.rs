use std::collections::HashSet;

fn first(ruck_sack_list: &[&str]) -> u32 {
    let ruck_sack_pairs: Vec<(&str, &str)> = ruck_sack_list
        .iter()
        .map(|ruck_sack| return ruck_sack.split_at(ruck_sack.chars().count() / 2))
        .collect();

    let mut score = 0;

    for ruck_sack_pair in ruck_sack_pairs {
        let (first, second) = ruck_sack_pair;

        let mut uniq_letters = HashSet::new();

        for cha in first.chars() {
            if second.contains(cha) {
                uniq_letters.insert(cha);
            }
        }

        for &letter in uniq_letters.iter() {
            if letter.is_ascii_uppercase() {
                score += letter as u32 - 38
            } else {
                score += letter as u32 - 96
            }
        }
    }

    score
}

fn second(ruck_sack_list: &[&str]) -> u32 {
    let mut score = 0;
    for ruck_sack_chunk in ruck_sack_list.chunks(3) {
        if ruck_sack_chunk.iter().any(|chunk| chunk.is_empty()) {
            continue;
        };

        let mut uniq_letters = HashSet::new();
        let first = ruck_sack_chunk[0];
        let second = ruck_sack_chunk[1];
        let third = ruck_sack_chunk[2];

        for cha in first.chars() {
            if second.contains(cha) && third.contains(cha) {
                uniq_letters.insert(cha);
            }
        }

        for &letter in uniq_letters.iter() {
            if letter.is_ascii_uppercase() {
                score += letter as u32 - 38
            } else {
                score += letter as u32 - 96
            }
        }
    }

    score
}

fn main() {
    //     let sample = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    //
    // ";
    // let ruck_sack_list: Vec<&str> = sample.split('\n').collect();

    let ruck_sack_list: Vec<&str> = include_str!("./day3.txt").split('\n').collect();

    let first_score = first(&ruck_sack_list);
    let second_score = second(&ruck_sack_list);

    println!("first score: {}", first_score);
    println!("second score: {}", second_score);
}
