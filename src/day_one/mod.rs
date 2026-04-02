pub mod question_one;
pub mod question_two;

pub const INPUT_PATH: &str = "../../inputs/day1/input.txt";

const MIN: i32 = 0;
const MAX: i32 = 99;

pub fn normalize(n: i32) -> i32 {
    let mut res = n % 100;
    if res > MAX {
        res = MIN + (res - (MAX + 1));
    } else if res < MIN {
        res = (MAX + 1) - (MIN - res).abs()
    }
    assert!(res >= MIN && res <= MAX);
    res
}
