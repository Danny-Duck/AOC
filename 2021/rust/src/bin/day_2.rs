#[test]
fn test_find_sub_positions() {
    let input: Vec<&str> = include_str!("./day_2_input.txt").split('\n').collect();
    assert_eq!(find_sub_position_part_one(&input), 1938402);
    assert_eq!(find_sub_position_part_two(&input), 1947878632);
}

struct Position {
    depth: usize,
    horizontal: usize,
}

impl Position {
    fn final_position(&self) -> usize {
        self.horizontal * self.depth
    }
    fn increase_depth(&mut self, amount: usize) {
        self.depth += amount
    }
    fn decrease_depth(&mut self, amount: usize) {
        // cant go higher (figuratively but literally lower literally) than sea level
        if self.depth > amount {
            self.depth -= amount
        } else {
            self.depth = 0;
        }
    }
    fn move_forward(&mut self, amount: usize) {
        self.horizontal += amount
    }
}

fn find_sub_position_part_one(input: &[&str]) -> usize {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    input.iter().for_each(|command_str| {
        if command_str.is_empty() {
            return;
        };

        let Command { command, amount } = Command::new(command_str);

        match command {
            "forward" => position.move_forward(amount),
            "down" => position.increase_depth(amount),
            "up" => position.decrease_depth(amount),
            _ => panic!("Didn't understand that command \"{}\"", command_str),
        }
    });

    position.final_position()
}

struct SpecificPosition {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl SpecificPosition {
    fn final_position(&self) -> usize {
        self.horizontal * self.depth
    }
    fn forward(&mut self, amount: usize) {
        self.horizontal += amount;
        self.depth += self.aim * amount;
    }
    fn increase_aim(&mut self, amount: usize) {
        self.aim += amount
    }
    fn decrease_aim(&mut self, amount: usize) {
        self.aim += amount
    }
}

struct Command<'a> {
    command: &'a str,
    amount: usize,
}

impl<'a> Command<'a> {
    fn new(command_str: &str) -> Command {
        let mut command_split = command_str.split_whitespace();
        Command {
            command: command_split.next().unwrap(),
            amount: command_split.last().unwrap().parse::<usize>().unwrap(),
        }
    }
}

fn find_sub_position_part_two(input: &[&str]) -> usize {
    let mut position = SpecificPosition {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    input.iter().for_each(|command_str| {
        if command_str.is_empty() {
            return;
        };

        let Command { command, amount } = Command::new(command_str);

        match command {
            "forward" => position.forward(amount),
            "down" => position.increase_aim(amount),
            "up" => position.decrease_aim(amount),
            _ => panic!("Didn't understand that command \"{}\"", command_str),
        }
    });

    position.final_position()
}

fn main() {
    let input: Vec<&str> = include_str!("./day_2_input.txt").split('\n').collect();
    println!("part one {}", find_sub_position_part_one(&input));
    println!("part two {}", find_sub_position_part_two(&input))
}
