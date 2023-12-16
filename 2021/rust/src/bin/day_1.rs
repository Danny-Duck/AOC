#[test]
fn test_depth_measure_functions() {
    let input: Vec<usize> = include_str!("./day_1_input.txt")
        .split('\n')
        .filter_map(|depth| {
            if depth.is_empty() {
                None
            } else {
                Some(depth.parse().unwrap())
            }
        })
        .collect();

    let sample_input_str = "199
200
208
210
200
207
240
269
260
263";

    let sample_input: Vec<usize> = sample_input_str
        .split('\n')
        .map(|depth| depth.parse().unwrap())
        .collect();

    assert_eq!(depth_measure_part_one(&sample_input), 7);
    assert_eq!(depth_measure_part_one(&input), 1167);

    assert_eq!(depth_measure_part_two(&sample_input), 5);
    assert_eq!(depth_measure_part_two(&input), 1130);
}

fn depth_measure_part_one(input: &[usize]) -> usize {
    input.windows(2).fold(0, |current, window| {
        if window[1] > window[0] {
            current + 1
        } else {
            current
        }
    })
}

fn depth_measure_part_two(input: &[usize]) -> usize {
    input.windows(4).fold(0, |current, window| {
        // the sliding window is only ever 4 indices long
        // 199  A
        // 200  A B
        // 208  A B
        // 210    B
        // if [200, 208, 210] (B) > [199, 200, 208] (A) it the depth has increased
        if window[1..=3].iter().sum::<usize>() > window[0..=2].iter().sum::<usize>() {
            current + 1
        } else {
            current
        }
    })
}

fn main() {
    let input: Vec<usize> = include_str!("./day_1_input.txt")
        .split('\n')
        .filter_map(|depth| {
            if depth.is_empty() {
                None
            } else {
                Some(depth.parse().unwrap())
            }
        })
        .collect();

    println!("part one {}", depth_measure_part_one(&input));
    println!("part two {}", depth_measure_part_two(&input));
}
