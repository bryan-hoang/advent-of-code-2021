use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;

pub struct Solution {
    pub part_1: String,
    pub part_2: String,
    pub nanos: u128,
}

impl Solution {
    fn new(part_1: String, part_2: String, nanos: u128) -> Solution {
        Self {
            part_1,
            part_2,
            nanos,
        }
    }
}

pub fn run_day(day: u8) -> Result<Solution, String> {
    return match day {
        1 => run_both(|| day_01::both()),
        2 => run_parts(|| day_02::part_1(), || day_02::part_2()),
        _ => Err(String::from("No solution")),
    };
}

fn run_parts<A: ToString, B: ToString>(
    part_1: impl Fn() -> A,
    part_2: impl Fn() -> B,
) -> Result<Solution, String> {
    run_both(|| (part_1(), part_2()))
}

fn run_both<A: ToString, B: ToString>(both_parts: impl Fn() -> (A, B)) -> Result<Solution, String> {
    let result: ((A, B), u128) = timeit(both_parts);
    let answers = result.0;
    Ok(Solution::new(
        answers.0.to_string(),
        answers.1.to_string(),
        result.1,
    ))
}

fn timeit<T>(part: impl Fn() -> T) -> (T, u128) {
    let now: Instant = Instant::now();
    let value: T = part();
    let time: u128 = now.elapsed().as_nanos();
    (value, time)
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_01() {
        test_day(1, "232", "1783");
    }

    fn test_day(day: u8, first: &str, second: &str) {
        let sln = crate::days::run_day(day).unwrap();
        assert_eq!(sln.part_1, first);
        assert_eq!(sln.part_2, second);
    }
}
