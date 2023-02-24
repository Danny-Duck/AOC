use std::collections::HashSet;

struct Case {
    case: &'static str,
    expected: i32,
}

fn main() {
    let data_stream = include_str!("../../../day6.txt");

    let samples = vec![
        Case {
            case: "bvwbjplbgvbhsrlpgdmjqwftvncz",
            expected: 5,
        },
        Case {
            case: "nppdvjthqldpwncqszvftbrmjlhg",
            expected: 6,
        },
        Case {
            case: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            expected: 10,
        },
        Case {
            case: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            expected: 11,
        },
    ];

    for sample in samples.iter() {
        println!(
            "case: {:?} expected: {}",
            signal_processor(sample.case, 4),
            sample.expected
        );
    }
    println!("first result: {}", signal_processor(data_stream, 4));
    println!("second result: {}", signal_processor(data_stream, 14));
}

fn signal_processor(stream: &str, message_size: usize) -> usize {
    let chars: Vec<char> = stream.chars().collect();
    let indexable_message_size = message_size - 1;
    for (ind, _) in chars.iter().enumerate() {
        // index 3 will be the 4th element, this is important because we are looking backwards in
        // in the stream
        if ind > indexable_message_size {
            let window = &chars[(ind - indexable_message_size)..=ind];
            let mut seen = HashSet::new();
            for &c in window.iter() {
                if !seen.contains(&c) {
                    seen.insert(c);
                }
            }
            if seen.len() == message_size {
                return ind + 1;
            }
        }
    }
    0
}
