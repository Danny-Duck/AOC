use std::str::Split;

fn main() {
    let input: Split<&str> = include_str!("../../../day1.txt").split("\n\n");
    let mut total_calories_each_elf_is_carrying: Vec<u32> = input
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
        .collect();

    total_calories_each_elf_is_carrying.sort();

    let top_three: &u32 = &total_calories_each_elf_is_carrying
        [total_calories_each_elf_is_carrying.len() - 3..total_calories_each_elf_is_carrying.len()]
        .iter()
        .sum();

    let max_calories = total_calories_each_elf_is_carrying.iter().max().unwrap();

    println!("most calories: {:?}", max_calories);
    println!("top 3: {:?}", top_three)
}
