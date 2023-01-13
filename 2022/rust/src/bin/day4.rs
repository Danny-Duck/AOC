fn get_task_pairs(text: &str) -> Vec<Vec<Vec<u32>>> {
    let task_pairs: Vec<Vec<Vec<u32>>> = text
        .split('\n')
        .filter_map(|task_pair_string| {
            if !task_pair_string.is_empty() {
                Some(
                    task_pair_string
                        .split(',')
                        .filter_map(|task_pair| {
                            if !task_pair.is_empty() {
                                Some(
                                    task_pair
                                        .split('-')
                                        .map(|task| task.parse::<u32>().unwrap())
                                        .collect(),
                                )
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();

    task_pairs
}

fn part1(text: &str) -> usize {
    // let task_pairs: Vec<Vec<Vec<u32>>> = include_str!("./day4.txt")
    let task_pairs = get_task_pairs(text);

    let redundant_pairs_count = task_pairs.iter().fold(0usize, |acc, pair| {
        let first = &pair[0];
        let second = &pair[1];

        if (first[0] >= second[0] && first[1] <= second[1])
            || (first[0] <= second[0] && first[1] >= second[1])
        {
            acc + 1
        } else {
            acc
        }
    });

    redundant_pairs_count
}

fn part2(text: &str) -> usize {
    let task_pairs = get_task_pairs(text);

    let redundant_pairs_count: usize = task_pairs.iter().fold(0usize, |acc, pair| {
        let first = &pair[0];
        let second = &pair[1];

        if (first[0] <= second[0] && first[1] >= second[0])
            || (first[0] >= second[0] && first[0] <= second[1])
        {
            acc + 1
        } else {
            acc
        }
    });

    redundant_pairs_count
}

fn main() {
    let sample = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
1-5,1-1
6-6,4-6
2-87,2-86
11-97,11-96
12-99,12-96
23-42,5-75
33-57,33-41
8-86,8-55
44-82,44-81
2-6,4-8";

    let sample2 = "5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    println!("sample part 1: {:?}", part1(sample));
    println!("sample part 2: {:?}", part2(sample));

    let real = include_str!("./day4.txt");
    println!("part 1: {:?}", part1(real));
    println!("part 2: {:?}", part2(real));
}
