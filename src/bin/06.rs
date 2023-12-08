use std::str::FromStr;

advent_of_code::solution!(6);

type Time = u64;
type Distance = u64;
type Race = (Time, Distance);
struct Races(Vec<Race>);

#[derive(Debug, PartialEq, Eq)]
struct ParseRacesError;

impl FromStr for Races {
    type Err = ParseRacesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let times_line = s.lines().nth(0).ok_or(ParseRacesError)?;
        let times: Vec<Time> = times_line
            .split_whitespace()
            .map(|s| s.parse::<Time>().ok())
            .filter(|x| x.is_some())
            .map(|x| match x {
                Some(x) => x,
                None => panic!("Error parsing time"),
            })
            .collect();

        let distances_line = s.lines().nth(1).ok_or(ParseRacesError)?;
        let distances: Vec<Distance> = distances_line
            .split_whitespace()
            .map(|s| s.parse::<Distance>().ok())
            .filter(|x| x.is_some())
            .map(|x| match x {
                Some(x) => x,
                None => panic!("Error parsing time"),
            })
            .collect();

        let mut races: Races = Races(Vec::new());
        for (t, d) in times.iter().zip(distances.iter()) {
            races.0.push((*t, *d));
        }
        Ok(races)
    }
}

fn get_winning_options(max_time: Time, max_distance: Distance) -> u64 {
    println!("max_time: {}, max_distance: {}", max_time, max_distance);
    let mut winning_options = 0;
    for t in 1..=max_time {
        let speed_at_release = t;
        let distance_after_max_time = speed_at_release * (max_time - t);
        if u64::from(distance_after_max_time) > max_distance {
            winning_options += 1;
        }
    }
    winning_options
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = Races::from_str(input).unwrap();
    let mut res = 1;
    for (t, d) in races.0.iter() {
        let winning_options = get_winning_options(*t, *d);
        res *= winning_options;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = Races::from_str(input).unwrap();
    let time: String = races.0.iter().map(|(t, _)| t.to_string()).collect();
    let time: u64 = time.parse().unwrap();

    let distance: String = races.0.iter().map(|(_, d)| d.to_string()).collect();
    println!("distance: {}", distance);
    let distance: u64 = distance.parse().unwrap();

    let winning_options = get_winning_options(time, distance);

    Some(winning_options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromstr() {
        let input = "Times:    20     30\nDistances:    100       200   ";
        let races = Races::from_str(input).unwrap();

        assert_eq!(races.0.len(), 2);
        assert_eq!(races.0[0], (20, 100));
        assert_eq!(races.0[1], (30, 200));

        let races = Races::from_str(&advent_of_code::template::read_file("examples", DAY)).unwrap();

        let winning_options_race_1 = get_winning_options(races.0[0].0, races.0[0].1);
        assert_eq!(winning_options_race_1, 4);

        let winning_options_race_2 = get_winning_options(races.0[1].0, races.0[1].1);
        assert_eq!(winning_options_race_2, 8);

        let winning_options_race_3 = get_winning_options(races.0[2].0, races.0[2].1);
        assert_eq!(winning_options_race_3, 9);
    }

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
