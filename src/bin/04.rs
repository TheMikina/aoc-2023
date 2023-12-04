advent_of_code::solution!(4);
use regex::Regex;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {

    let mut score = 0;
    for line in input.lines() {
        let mut groups = line.split(":").last().unwrap().split("|");
        let winning_str = groups.next().unwrap();
        let guessed_str = groups.next().unwrap();
        let re = Regex::new(r"\d+").unwrap();
        let winning = re.find_iter(winning_str).map(|n| n.as_str()).collect_vec();
        let correct = re.find_iter(guessed_str).map(|n| n.as_str()).filter(|guess| winning.contains(guess));
        let correct_count:u32 = correct.count().try_into().unwrap();
        if correct_count == 0 {
            continue
        }
        score += 2_u32.pow(correct_count - 1);
    }

    Some(score)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
