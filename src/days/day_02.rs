const INPUT: &str = include_str!("../../data/day_02.txt");

pub fn part_1() -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in INPUT.lines() {
        let mut split = line.split_whitespace();
        // Split string into tuple of direction and number
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => horizontal_position += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    horizontal_position * depth
}

pub fn part_2() -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in INPUT.lines() {
        let mut split = line.split_whitespace();
        // Split string into tuple of direction and number
        let commands = split.next().unwrap();
        let units = split.next().unwrap().parse::<i32>().unwrap();
        match commands {
            "forward" => {
                horizontal_position += units;
                depth += aim * units;
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => panic!("Unknown direction: {}", commands),
        }
    }

    horizontal_position * depth
}
