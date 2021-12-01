use std::collections::VecDeque;

const INPUT: &str = include_str!("../../data/day_01.txt");

pub fn both() -> (i32, i32) {
    let mut total_increases = 0;
    let mut prev = INPUT.lines().next().unwrap().parse::<i32>().unwrap();
    let mut total_window_sum_increases = 0;
    let mut window_size = 0;
    let mut window_sum = 0;
    // Queue of integers of size 3
    let mut window_contents = VecDeque::new();
    for line in INPUT.lines() {
        let num = line.parse::<i32>().unwrap();
        if num > prev {
            total_increases += 1;
        }

        if window_size < 3 {
            window_contents.push_back(num);
            window_sum += num;
            window_size += 1;
        } else {
            let prev_sum = window_sum;
            window_contents.push_back(num);
            window_sum += num;
            window_sum -= match window_contents.pop_front() {
                Some(x) => x,
                None => panic!("No more items in window"),
            };
            if window_sum > prev_sum {
                total_window_sum_increases += 1;
            }
        }

        prev = num;
    }

    (total_increases, total_window_sum_increases)
}
