advent_of_code::solution!(2);
use regex::Regex;
pub fn part_one(input: &str) -> Option<u32> {
    let mut correct = 0;

    for line in input.lines() {
        correct += parse_line(line);
    }

    return Some(correct.try_into().unwrap());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut correct = 0;

    for line in input.lines() {
        correct += parse_line2(line);
    }

    return Some(correct.try_into().unwrap());
}

fn parse_line(line: &str) -> i32 {
    const MAX_BLUE: i32 = 14;
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    let mut i = 0;
    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();

    if !check_color(line, red, MAX_RED) {
        return 0;
    }

    if !check_color(line, green, MAX_GREEN) {
        return 0;
    }

    if !check_color(line, blue, MAX_BLUE) {
        return 0;
    }

    let game = Regex::new(r"Game (\d+)").unwrap();
    return game.captures(line).unwrap()[1].parse::<i32>().unwrap();
    
}

fn check_color(line: &str, re: Regex, max_count: i32) -> bool {
    for c in re.captures_iter(line) {
        let count = c[1].parse::<i32>().unwrap();
        if count > max_count {
            return false;
        }
    }
    return true;

}

fn parse_line2(line: &str) -> i32 {
    let mut i = 1;
    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();

    i *= min_color(line,red);
    i *= min_color(line,green);
    i *= min_color(line,blue);

    return i;
    
}

fn min_color(line: &str, re: Regex) -> i32 {
    let mut min = 0;
    for c in re.captures_iter(line) {
        let count = c[1].parse::<i32>().unwrap();
        if count > min {
            min = count;
        }
    }
    return min;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
