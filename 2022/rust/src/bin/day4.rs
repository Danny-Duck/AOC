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

    let task_pairs: Vec<Vec<Vec<u32>>> = include_str!("./day4.txt")
        // let task_pairs: Vec<Vec<Vec<u32>>> = sample
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

    let mut redundant_pairs_count = 0;

    for pair in task_pairs.iter() {
        let first = &pair[0];
        let second = &pair[1];

        if (first[0] == second[0] || first[1] == second[1])
            || (first[0] > second[0] && first[1] < second[1])
            || (first[0] < second[0] && first[1] > second[1])
        {
            redundant_pairs_count += 1
        }
    }

    println!("{:?}", redundant_pairs_count);
}
