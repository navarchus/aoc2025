use crate::day_one::normalize;

pub fn solve(input: Vec<String>) -> i32 {
    let mut res = 0;
    let mut dial = 50;

    for val in input {
        let (direction, magnitude_str) = val.split_at(1);
        let magnitude_int = magnitude_str.parse::<i32>().unwrap();

        dial += match direction {
            "L" => -1 * magnitude_int,
            "R" => magnitude_int,
            _ => panic!("unsupported direction {}!", direction),
        };

        dial = normalize(dial);
        if dial == 0 {
            res += 1;
        }
    }

    res
}
