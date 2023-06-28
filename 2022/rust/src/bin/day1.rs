use std::str::Split;

struct Elf {
    food: Vec<u32>,
}

impl Elf {
    fn new(food_str: &str) -> Elf {
        let food = food_str
            .split('\n')
            .map(|food_sub_str| {
                if !food_sub_str.is_empty() {
                    str::parse(food_sub_str).expect("Given string was not valid")
                } else {
                    0
                }
            })
            .collect();
        Elf { food }
    }
    fn sum_calories(&self) -> u32 {
        self.food.iter().sum()
    }
}

fn main() {
    let input: Split<&str> = include_str!("../../../day1.txt").split("\n\n");
    let mut total_calories_each_elf_is_carrying: Vec<u32> =
        input.map(|group| Elf::new(group).sum_calories()).collect();

    total_calories_each_elf_is_carrying.sort();

    let top_three: &u32 = &total_calories_each_elf_is_carrying
        [total_calories_each_elf_is_carrying.len() - 3..total_calories_each_elf_is_carrying.len()]
        .iter()
        .sum();

    let max_calories = total_calories_each_elf_is_carrying.iter().max().unwrap();

    println!("most calories: {:?}", max_calories);
    println!("top 3: {:?}", top_three)
}
