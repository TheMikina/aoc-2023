advent_of_code::solution!(1);
use fancy_regex::Regex;

fn parse_line(line: &str, regex: &str) -> u32 {
    let mut dec = 0;
    let mut ones = 0;

    let re = Regex::new(regex).unwrap();

    for i in 0..line.len() {

        let matched_option = re.find_from_pos(line, i).unwrap();
        match matched_option {
            Some(res) => {
                let matched = res.as_str();
                    match convert_to_number(matched) {
                        Some(result) => {
                            dec = result;
                            break;
                        },
                        None => println!("Parsing {matched} failed.")
                    }
            },
            None => continue,
        }
    }

        for i in (0..line.len()).rev() {

            let matched_option = re.find_from_pos(line, i).unwrap();
            match matched_option {
                Some(res) => {
                    let matched = res.as_str();           
                    match convert_to_number(matched) {
                        Some(result) => {
                            ones = result;
                            break;
                        },
                        None => println!("Parsing {matched} failed.")
                    }
                },
                None => continue,
            }
        
    }
    let result = dec * 10 + ones;
    println!("{}", result);
    return result;

}

fn convert_to_number(input: &str) -> Option<u32> {
    if input.len() == 1 {
        return input.chars().last().expect("Parsing failed.").to_digit(10);
    }
    match input {
        "one" => return Some(1),
        "two" => return Some(2),
        "three" => return Some(3),
        "four" => return Some(4),
        "five" => return Some(5),
        "six" => return Some(6),
        "seven" => return Some(7),
        "eight" => return Some(8),
        "nine" => return Some(9),
        _ => return None
    }
}

pub fn part_one(input: &str) -> Option<u32> {

        let mut total = 0;
        for line in input.lines() {
            total += parse_line(line, r"([0-9])");
        }
        return Some(total);
    
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        total += parse_line(line, r"([0-9]|one|two|three|four|five|six|seven|eight|nine)");
    }
    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(281));
    }
}
