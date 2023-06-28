use std::collections::HashSet;

struct RuckSack {
    contents_score: u32,
}

impl RuckSack {
    fn new(ruck_sack_contents: &str) -> RuckSack {
        let (first, second) = ruck_sack_contents.split_at(ruck_sack_contents.chars().count() / 2);
        let mut uniq_letters_set = HashSet::new();

        for cha in first.chars() {
            if second.contains(cha) {
                uniq_letters_set.insert(cha);
            }
        }

        RuckSack {
            contents_score: uniq_letters_set.iter().fold(0, |count, letter| {
                if letter.is_ascii_uppercase() {
                    count + *letter as u32 - 38
                } else {
                    count + *letter as u32 - 96
                }
            }),
        }
    }
}

fn first(ruck_sack_str_list: &[&str]) -> u32 {
    ruck_sack_str_list
        .iter()
        .map(|ruck_sack_str| RuckSack::new(ruck_sack_str))
        .fold(0u32, |count, ruck_sack| count + ruck_sack.contents_score)
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

    let ruck_sack_list: Vec<&str> = include_str!("../../../day3.txt").split('\n').collect();

    let first_score = first(&ruck_sack_list);
    let second_score = second(&ruck_sack_list);

    println!("first score: {}", first_score);
    println!("second score: {}", second_score);
}
