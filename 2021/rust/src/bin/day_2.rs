#[test]
fn test_find_sub_positions() {
    let input: Vec<&str> = include_str!("./day_2_input.txt").split('\n').collect();
    assert_eq!(find_sub_position_part_one(&input), 1938402);
    assert_eq!(find_sub_position_part_two(&input), 1947878632);
}

fn find_sub_position_part_one(input: &[&str]) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    input.iter().for_each(|command_str| {
        if command_str.contains("forward") {
            horizontal += command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if command_str.contains("down") {
            depth += command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if command_str.contains("up") {
            let depth_change = command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            // cant go higher (figuratively but lower literally) than sea level
            if depth > depth_change {
                depth -= depth_change
            } else {
                depth = 0
            }
        };
    });

    depth * horizontal
}

fn find_sub_position_part_two(input: &[&str]) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    input.iter().for_each(|command_str| {
        if command_str.contains("forward") {
            let forward = command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            horizontal += forward;
            depth += aim * forward;
        } else if command_str.contains("down") {
            aim += command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if command_str.contains("up") {
            aim -= command_str
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        };
    });

    depth * horizontal
}

fn main() {
    let input: Vec<&str> = include_str!("./day_2_input.txt").split('\n').collect();
    println!("part one {}", find_sub_position_part_one(&input));
    println!("part two {}", find_sub_position_part_two(&input))
}
