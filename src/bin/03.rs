advent_of_code::solution!(3);
use regex::Regex;
use std::ops::Range;
use std::collections::HashMap;
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
            if find_symbol(&lines, check_range, i,r"[^0-9\.]")  {
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

pub fn find_symbol(lines: &Vec<&str>, range: Range<usize>, line: usize, regex: &str) -> bool {

    let re = Regex::new(regex).unwrap();
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

pub fn find_stars(lines: &Vec<&str>, range: Range<usize>, line: usize, dictionary: &mut HashMap<String,Engine>, for_number: u32 ) -> bool {

    let re = Regex::new(r"\*").unwrap();
    for i in extend_range(lines.len(), line..line+1) {
        if i > lines.len() {
            continue;
        }

        for star in re.find_iter(&lines[i]) {
            if star.start() < range.start || star.start() >= range.end {
                continue;
            }
            let index = format!("{}-{}", i, star.start());
            println!("Found star at: {}, for number {}", index, for_number);
            if dictionary.contains_key(&index) {
                dictionary.get_mut(&index).unwrap().found_count += 1;
                dictionary.get_mut(&index).unwrap().gear_ratio *= for_number;
                continue;
            } else {
                dictionary.insert(index, Engine { gear_ratio: for_number, found_count: 1});
            }
        }
    }

    return false;
}

pub struct Engine {
    gear_ratio: u32,
    found_count:u32
}
pub fn part_two(input: &str) -> Option<u32> {

    let lines = input.lines().collect::<Vec<&str>>();
    let mut dict: HashMap<String,Engine> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for i in 0..lines.len() {
        let line = lines[i];
        let numbers = re.find_iter(line);
        for number in numbers {
            let found_at = number.range();
            let check_range = extend_range(line.len(), found_at);
            find_stars(&lines, check_range, i,&mut dict, number.as_str().parse::<u32>().unwrap());
        }
    }
    for (_, engine) in dict {
        println!("Found: {} times, sum {}", engine.found_count, engine.gear_ratio);
        if engine.found_count == 2 {
            sum += engine.gear_ratio;
        }
    }
    return Some(sum.try_into().unwrap());
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
