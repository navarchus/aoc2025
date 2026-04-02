pub fn solve(input: Vec<String>) -> i32 {
    let mut res = 0;
    let mut start_dial = 50;

    for val in input {
        let (direction, magnitude_str) = val.split_at(1);
        let magnitude_int = magnitude_str.parse::<i32>().unwrap();

        let mut end_dial = start_dial;
        let num_of_zero = match direction {
            "L" => {
                end_dial -= magnitude_int;
                ((end_dial + 1)..=start_dial)
                    .fold(0, |acc, val| if val % 100 == 0 { acc + 1 } else { acc })
            }
            "R" => {
                end_dial += magnitude_int;
                (start_dial..end_dial)
                    .fold(0, |acc, val| if val % 100 == 0 { acc + 1 } else { acc })
            }
            _ => panic!("unsupported direction {}!", direction),
        };

        start_dial = end_dial;
        res += num_of_zero;
    }
    res
}
