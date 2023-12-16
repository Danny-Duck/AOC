use std::ops::BitXor;

#[test]
fn test_get_sub_power_comsumption_rate() {
    let input: Vec<&str> = include_str!("./day_3_input.txt").split('\n').collect();
    let sample = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .split('\n')
        .collect::<Vec<&str>>();

    assert_eq!(get_sub_power_comsumption_rate_part_one(&input), 0);
    assert_eq!(get_sub_power_comsumption_rate_part_one(&sample), 198)
}

// count the most common bits for the gamma, then binary inverse them and that will be the epsilon.
// Multiply them both together and thats the power consumption rate.
fn get_sub_power_comsumption_rate_part_one(input: &[&str]) -> usize {
    let mut gamma: Vec<usize> = vec![];
    let mut position_count = 0;

    // assuming all the data points are the same length
    while position_count < input.first().unwrap().len() {
        let gamma_count: &usize = &input.iter().fold(0, |current, stat_readout| {
            if stat_readout.chars().nth(position_count).unwrap() == '1' {
                current + 1
            } else {
                current
            }
        });

        if gamma_count > &(input.len() / 2) {
            gamma.push(1);
        } else {
            gamma.push(0)
        }

        position_count += 1;
    }

    println!("norm {:?}", gamma);
    println!(
        "xor {:?}",
        gamma.iter().fold(0 as i32, |current, next| {
            println!("{current} << {next} {}", 0 << 1);
            println!("{current} << {next} {}", 1 << 1);
            current << (next + 1)
        })
    );

    0
}

fn main() {
    let input: Vec<&str> = include_str!("./day_3_input.txt").split('\n').collect();
    let sample = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .split('\n')
        .collect::<Vec<&str>>();

    println!("{}", get_sub_power_comsumption_rate_part_one(&sample));
}
