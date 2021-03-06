pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn problem1(directions: &[Direction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for direction in directions {
        match direction {
            Direction::Forward(f) => horizontal += f,
            Direction::Down(d) => depth += d,
            Direction::Up(u) => depth -= u,
        }
    }
    horizontal * depth
}

pub fn problem2(directions: &[Direction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for direction in directions {
        match direction {
            Direction::Forward(f) => {
                horizontal += f;
                depth += aim * f
            }
            Direction::Down(d) => aim += d,
            Direction::Up(u) => aim -= u,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    use super::*;

    const DATA_PATH: &'static str = "data/day2";

    #[test]
    fn problem1_example() {
        let directions = [
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];
        assert_eq!(problem1(&directions), 150);
    }

    fn load_data() -> Vec<Direction> {
        let file = File::open(Path::new(DATA_PATH)).unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    .map(|split| split.to_string())
                    .collect::<Vec<String>>()
            })
            .filter_map(|strings| match [strings[0].as_str(), strings[1].as_str()] {
                ["forward", f] => Some(Direction::Forward(f.parse::<i32>().unwrap())),
                ["down", d] => Some(Direction::Down(d.parse::<i32>().unwrap())),
                ["up", u] => Some(Direction::Up(u.parse::<i32>().unwrap())),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn problem1_real() {
        let input = load_data();
        assert_eq!(problem1(input.as_slice()), 1636725);
    }

    #[test]
    fn problem2_real() {
        let input = load_data();
        assert_eq!(problem2(input.as_slice()), 1872757425);
    }
}
