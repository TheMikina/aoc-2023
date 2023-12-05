advent_of_code::solution!(3);
use regex::Regex;
use std::ops::Range;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        let numbers = re.find_iter(line);
        for number in numbers {
            let found_at = number.range();
            let check_range = extend_range(line.len(), found_at);
            println!("Found number: {}", number.as_str());
            if find_symbol(&lines, check_range, i)  {
                sum += number.as_str().parse::<i32>().unwrap();
            }
        }
    }

    return Some(sum.try_into().unwrap());
}

pub fn extend_range(max:usize, range: Range<usize>) -> Range<usize> {

    let mut start = range.start;
    let mut end = range.end + 1;

    if start > 0 {
        start -= 1;
    }

    if end > max {
        end = max;
    }

    return start..end;
}

pub fn find_symbol(lines: &Vec<&str>, range: Range<usize>, line: usize) -> bool {

    let re = Regex::new(r"[^0-9\.]").unwrap();
    for i in extend_range(lines.len(), line..line+1) {
        if i > lines.len() {
            continue;
        }
        println!("Checking line: {}", &lines[i][range.start..range.end]);
        if re.is_match(&lines[i][range.start..range.end]) {
            return true;
        }
    }

    return false;
}


pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
