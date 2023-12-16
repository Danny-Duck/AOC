use std::{fs, str::Chars};

fn part_1(input: &str) -> u32 {
    let lines: Vec<u32> = input
        .split('\n')
        .filter_map(|line| {
            let numbers_in_line: Vec<&str> = line.matches(char::is_numeric).collect();
            if numbers_in_line.is_empty() {
                return None;
            };
            if numbers_in_line.len() == 1 {
                println!("{}\n", numbers_in_line[0]);
                return Some(numbers_in_line[0].parse().unwrap());
            }
            let mut string_pair = numbers_in_line.first().unwrap().to_string();
            string_pair.push_str(numbers_in_line.last().unwrap());
            Some(string_pair.parse().unwrap())
        })
        .collect();
    lines.iter().sum()
}

fn find_numbers(remaining: &Vec<char>, mut buffer: Vec<char>, is_reverse: bool) -> u32 {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    if remaining.is_empty() {
        println!("hit");
        return 0;
    }

    let current_index = remaining.first().unwrap();

    if current_index.is_numeric() {
        println!("hit {}", current_index.to_string().parse::<u32>().unwrap());
        return current_index.to_string().parse::<u32>().unwrap();
    }

    if is_reverse {
        buffer.insert(0, *current_index);
    } else {
        buffer.push(*current_index);
    }

    // check if buffer contains any of the words
    let temp: String = buffer.iter().copied().collect();
    println!("temp: {}", temp);

    for (index, number_word) in words.iter().enumerate() {
        if temp.contains(number_word) {
            println!("number {}", index);
            return index as u32;
        }
    }

    // if so return the first number found
    // else call self again
    println!("{:?} {:?} {}", buffer, &remaining[1..].to_vec(), is_reverse);
    find_numbers(&remaining[1..].to_vec(), buffer, is_reverse)
}

fn part_2(input: &str) -> u32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    // let lines = ["three41fivetzzfvmlsfive5two"];

    lines
        .iter()
        .map(|line| {
            println!("line: {}", line);
            let mut line_chars = line.chars().collect::<Vec<char>>();
            let first = find_numbers(&line_chars, vec![], false);
            println!("first: {}", first);

            println!("{:?}", line_chars);
            line_chars.reverse();
            println!("{:?}", line_chars);
            let last = find_numbers(&line_chars, vec![], true);
            println!("last: {}", last);

            // println!("{:?}: {}{}", line_chars, first, last);
            println!("{first}{last}");
            let res = format!("{first}{last}").trim().parse::<u32>().unwrap();
            println!("res {res}");
            res
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("../../../day1-data.txt").unwrap();
    // println!("{:?}", part_1(&input));
    // println!("{:?}", part_2(&input));
    // part_2(&input);
    // part_2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! multi_test {
        ($func:ident, $($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, $func(input));
            }
        )*
        }
    }

    multi_test!(
        part_1,
        p1_case_1: ("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", 77),
    );

    multi_test!(
        part_2,
        p2_case_1: ("three41fivetzzfvmlsfive5two", 32),
        p2_case_2: ("rrslpzr1", 1),
        p2_case_3: ("eighthree", 83),
    );
}
