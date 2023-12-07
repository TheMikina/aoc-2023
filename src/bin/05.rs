advent_of_code::solution!(5);

pub struct AlmanacMap {
    pub destination_start: u64,
    pub source_start: u64,
    pub length: u64
}

impl AlmanacMap {
    pub fn new(destination_start: u64, source_start: u64, length: u64) -> AlmanacMap {
        AlmanacMap {
            destination_start,
            source_start,
            length
        }
    }

    pub fn get_destination(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source >= self.source_start + self.length {
            //println!{"{} is not in range {}-{}", source, self.source_start, self.source_start + self.length};
            return None;
        }
        let offset = source - self.source_start;
        return Some(self.destination_start + offset);
    }
}

pub fn parse_almanac_line(line:&str) -> AlmanacMap {
    let mut parts = line.split(" ");
    let destination_start = parts.next().unwrap().parse::<u64>().unwrap();
    let source_start = parts.next().unwrap().parse::<u64>().unwrap();
    let length = parts.next().unwrap().parse::<u64>().unwrap();
    AlmanacMap {
        destination_start,
        source_start,
        length
    }
}

pub fn parse_almanac_category(input: &str) -> Vec<AlmanacMap> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(parse_almanac_line(line));
    }
    result
}

pub fn get_seed_destination(seed: u64, almanacs: &Vec<Vec<AlmanacMap>>) -> Option<u64> {
    let mut destination = seed;
    //println!("Seed {}", seed);
    for almanac in almanacs {
        for map in almanac {
            match map.get_destination(destination) {
                None => continue,
                Some(new_destination) => {
                //println!("{} -> {}", destination, new_destination);
                destination = new_destination;
                break;
                }
            }
        }
    }
    return Some(destination);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut almanacs: Vec<Vec<AlmanacMap>> = Vec::new();
    let mut index = 666;
    for line in input.lines() {
        if line.contains("seeds") || line.len() < 2 {
            continue;
        }

        if line.contains("-") {
            if index == 666 {
                index = 0;
            } else {
                index += 1;
            }
            almanacs.push(Vec::new());
            continue;
        }

        almanacs[index].push(parse_almanac_line(line));
    }

    let mut lines = input.lines();
    let mut smallest = u64::MAX;
    for number in lines.next().unwrap().split(" ") {
        match number.parse::<u64>() {
            Ok(seed) => {
                if let Some(destination) = get_seed_destination(seed, &almanacs) {
                    //println!("Seed: {} -> {}", seed, destination);
                    if destination < smallest {
                        smallest = destination;
                    }
                }
            },
            Err(_) => continue
        }
    }

    Some(smallest)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut almanacs: Vec<Vec<AlmanacMap>> = Vec::new();
    let mut index = 666;
    for line in input.lines() {
        if line.contains("seeds") || line.len() < 2 {
            continue;
        }

        if line.contains("-") {
            if index == 666 {
                index = 0;
            } else {
                index += 1;
            }
            almanacs.push(Vec::new());
            continue;
        }

        almanacs[index].push(parse_almanac_line(line));
    }

    let mut lines = input.lines();
    let mut smallest = u64::MAX;
    let mut start = 0;
    for number in lines.next().unwrap().split(" ") {
        match number.parse::<u64>() {
            Ok(next) => {
                if start == 0 {
                    start = next;
                    continue;
                }
                //cprintln!("{}-{}", start, next);
                for seed in start..start+next {
                    if let Some(destination) = get_seed_destination(seed, &almanacs) {
                        //println!("Seed: {} -> {}", seed, destination);
                        if destination < smallest {
                            smallest = destination;
                        }
                    } 
                }
                start = 0;
            },
            Err(_) => continue
        }
    }

    Some(smallest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
