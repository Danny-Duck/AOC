use regex::Regex;

#[derive(Debug)]
struct Instruction {
    crate_count: usize,
    initial_stack_index: usize,
    destination_stack_index: usize,
}

/// Parses a string in the format of `"move 3 from 9 to 6\nmove 4 from 1 to 8\n...` and returns a
/// `Vector` of `Instruction`s
///
/// # Arguments
/// * `instructions_string` a string expected to be formatted like `"move 3 from 9 to 6\nmove 4 from 1 to 8\n...`
/// # Example
/// ```
/// let instruction = parse_instructions("move 3 from 9 to 6");
/// //  Instruction {
/// //     crate_count: 3,
/// //     initial_stack: 9,
/// //     destination_stack: 6,
/// // },
/// ```
fn parse_instructions(instructions_string: &str) -> Vec<Instruction> {
    let re = Regex::new(r"move (?P<crate_count>[0-9]*) from (?P<initial_stack_index>[0-9]*) to (?P<destination_stack_index>[0-9]*)").unwrap();

    instructions_string
        .split('\n')
        .filter_map(|instruction| {
            if instruction.is_empty() {
                None
            } else {
                let res = re.captures(instruction).unwrap();
                let crate_count: usize = res.name("crate_count").unwrap().as_str().parse().unwrap();
                let initial_stack_index: usize = res
                    .name("initial_stack_index")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let destination_stack_index: usize = res
                    .name("destination_stack_index")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();

                Some(Instruction {
                    crate_count,
                    initial_stack_index,
                    destination_stack_index,
                })
            }
        })
        .collect()
}

fn main() {
    let instructions_string = include_str!("../../../day5.txt");
    let mut crate_stack = vec![
        vec!["S", "C", "V", "N"],
        vec!["Z", "M", "J", "H", "N", "S"],
        vec!["M", "C", "T", "G", "J", "N", "D"],
        vec!["T", "D", "F", "J", "W", "R", "M"],
        vec!["P", "F", "H"],
        vec!["C", "T", "Z", "H", "J"],
        vec!["D", "P", "R", "Q", "F", "S", "L", "Z"],
        vec!["C", "S", "L", "H", "D", "F", "P", "W"],
        vec!["D", "S", "M", "P", "F", "N", "G", "Z"],
    ];

    let instructions = parse_instructions(instructions_string);

    for instruction in instructions.into_iter() {
        let Instruction {
            crate_count,
            destination_stack_index,
            initial_stack_index,
        } = instruction;

        let donor_stack = &mut crate_stack[initial_stack_index - 1];
        let mut payload = donor_stack.split_off(donor_stack.len() - crate_count);
        // payload.reverse(); Answer 1 needs the stacks reversed
        crate_stack[destination_stack_index - 1].append(&mut payload);
    }

    let answer = crate_stack
        .iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect::<Vec<&str>>()
        .join("");

    println!("{}", answer);
}
