advent_of_code::solution!(2);

struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

impl From<&str> for Set {
    fn from(s: &str) -> Self {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        let parts = s.split(", ");
        for part in parts {
            let mut part_parts = part.split(' ');
            let number = part_parts.next().unwrap().parse::<u32>().unwrap();
            let color = part_parts.next().unwrap();
            match color {
                "blue" => {
                    blue = number;
                }
                "green" => {
                    green = number;
                }
                "red" => {
                    red = number;
                }
                _ => {
                    println!("{:?}", color);
                    panic!("Unknown color")
                }
            }
        }
        Set { blue, green, red }
    }
}

struct Game {
    number: u32,
    sets: Vec<Set>,
}

impl Game {
    fn possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for set in &self.sets {
            if set.red > max_red || set.green > max_green || set.blue > max_blue {
                return false;
            }
        }
        true
    }

    fn fewest_cubes_possible(&self) -> Set {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for set in &self.sets {
            blue = set.blue.max(blue);
            green = set.green.max(green);
            red = set.red.max(red);
        }

        Set { blue, green, red }
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut parts = s.split(": ");

        let first_part = parts.next().unwrap();
        //drop the first 5 chars
        let number = first_part[5..].parse::<u32>().unwrap();

        let second_part = parts.next().unwrap();
        let mut sets = Vec::new();
        let set_parts = second_part.split("; ");
        for set_part in set_parts {
            sets.push(Set::from(set_part));
        }

        Game { number, sets }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(Game::from).collect();
    let erg = games
        .iter()
        .filter(|game| game.possible(12, 13, 14))
        .map(|game| game.number)
        .sum();
    Some(erg)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(Game::from).collect();
    let reds = games
        .iter()
        .map(|game| game.sets.iter().map(|set| set.red).max().unwrap())
        .collect::<Vec<u32>>();
    let greens = games
        .iter()
        .map(|game| game.sets.iter().map(|set| set.green).max().unwrap())
        .collect::<Vec<u32>>();
    let blues = games
        .iter()
        .map(|game| game.sets.iter().map(|set| set.blue).max().unwrap())
        .collect::<Vec<u32>>();

    let mut power = Vec::new();
    for i in 0..reds.len() {
        power.push(reds[i] * greens[i] * blues[i]);
    }
    Some(power.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_from() {
        let set = Set::from("3 blue, 4 red");
        assert_eq!(set.blue, 3);
        assert_eq!(set.green, 0);
        assert_eq!(set.red, 4);
        let set: Set = Set::from("3 green, 4 blue, 1 red");
        assert_eq!(set.blue, 4);
        assert_eq!(set.green, 3);
        assert_eq!(set.red, 1);
    }

    #[test]
    fn test_game_from() {
        let game = Game::from("Game 1: 3 blue, 4 red; 4 green, 1 red");
        assert_eq!(game.number, 1);
        assert_eq!(game.sets.len(), 2);
        assert_eq!(game.sets[0].blue, 3);
        assert_eq!(game.sets[0].green, 0);
        assert_eq!(game.sets[0].red, 4);
        assert_eq!(game.sets[1].blue, 0);
        assert_eq!(game.sets[1].green, 4);
        assert_eq!(game.sets[1].red, 1);
    }

    #[test]
    fn test_fewest_cubes_possible() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let set = game.fewest_cubes_possible();
        assert_eq!(set.blue, 6);
        assert_eq!(set.green, 2);
        assert_eq!(set.red, 4);
    }

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
