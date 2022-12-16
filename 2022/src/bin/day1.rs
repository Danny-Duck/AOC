use std::str::Split;

fn main() {
    let input: Split<&str> = include_str!("./day1.txt").split("\n\n");
    let groups: u32 = input
        .map(|group| {
            group
                .split('\n')
                .map(|string_number| {
                    if !string_number.is_empty() {
                        str::parse(string_number).expect("ahhh shit")
                    } else {
                        0
                    }
                })
                .sum()
        })
        .max()
        .unwrap();

    println!("{:?}", groups)
}
