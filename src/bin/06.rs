advent_of_code::solution!(6);
use regex::Regex;
pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let re = Regex::new(r"\d+").unwrap();
    let times:Vec<&str> = re.find_iter(lines.next().unwrap()).map(|x| x.as_str()).collect();
    let distances:Vec<&str> = re.find_iter(lines.next().unwrap()).map(|x| x.as_str()).collect();
    let mut result = 1;
    for i in 0..times.len() {
        let t = times[i].parse().unwrap();
        let d = distances[i].parse().unwrap();
        let (mut min,mut max) = calculate_interval(t, d);
        if (t - min) * min == d {
            min += 1;
        }

        if (t - max) * max == d {
            max -= 1;
        }
        let winnable = max - min + 1;
        result *= winnable;
    }
    
    Some(result.try_into().unwrap())
}

pub fn calculate_interval(max_time: u64, distance: u64) -> (u64,u64) {
    let t = max_time as f64;
    let d = distance as f64;
    let min = 0.5_f64 * (t - (t.powf(2.0) - 4.0 * d).sqrt());
    let max = 0.5_f64 * (t + (t.powf(2.0) - 4.0 * d).sqrt());
    (min.ceil() as u64, max.floor() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let new_input = input.replace(" ","");
    let mut lines = new_input.lines();
    let re = Regex::new(r"\d+").unwrap();
    let times:Vec<&str> = re.find_iter(lines.next().unwrap()).map(|x| x.as_str()).collect();
    let distances:Vec<&str> = re.find_iter(lines.next().unwrap()).map(|x| x.as_str()).collect();
    let mut result:u64 = 1;
    for i in 0..times.len() {
        let t:u64 = times[i].parse().unwrap();
        let d:u64 = distances[i].parse().unwrap();
        let (mut min,mut max) = calculate_interval(t, d);
        if (t - min) * min == d {
            min += 1;
        }

        if (t - max) * max == d {
            max -= 1;
        }
        let winnable = max - min + 1;
        result *= winnable;
    }
    
    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
