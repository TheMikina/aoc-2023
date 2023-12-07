advent_of_code::solution!(4);
use regex::Regex;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {

    let mut score = 0;
    for line in input.lines() {
        let correct_count = count_correct(line);
        if correct_count == 0 {
            continue
        }
        score += 2_u32.pow(correct_count - 1);
    }

    Some(score)
}

pub fn count_correct(line: &str) -> u32 {
    let mut groups = line.split(":").last().unwrap().split("|");
    let winning_str = groups.next().unwrap();
    let guessed_str = groups.next().unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let winning = re.find_iter(winning_str).map(|n| n.as_str()).collect_vec();
    let correct = re.find_iter(guessed_str).map(|n| n.as_str()).filter(|guess| winning.contains(guess));
    let correct_count:u32 = correct.count().try_into().unwrap();
    return correct_count;
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_lines = input.lines().count();
    let mut card_copies: Vec<u32> = vec![0;num_lines];
    let mut index: usize = 0;
    for line in input.lines() {
        let correct_count = count_correct(line);
        card_copies[index]+=1;
        let my_copies = card_copies[index];
        index += 1;
        if correct_count == 0 {
            continue
        }
        for i in 0..correct_count {
            let new_index: usize = index + (i as usize);
            card_copies[new_index] += my_copies;
        }

    }
    card_copies.iter().sum::<u32>().try_into().ok()
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
        assert_eq!(result, Some(30));
    }
}
