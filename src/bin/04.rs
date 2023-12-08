advent_of_code::solution!(4);

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    copies: u32,
}

impl Card {
    fn matches(&self) -> u32 {
        let mut matches = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        matches
    }

    fn points(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }
        // return 2^numbers_in_winning_numbers
        2u32.pow(matches - 1)
    }
}

impl From<&str> for Card {
    /// Parse a card from a string (line).
    /// Line has the following format:
    /// "Card 1: 1 2 3 4 5 | 6 7 8 9 10"
    /// where the first number is the card id, the numbers before the pipe are the winning numbers
    /// and the numbers after the pipe are the numbers on the card.
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();

        // drop the first 5 chars, which are "Card "
        let id = parts[0][5..].trim().parse::<u32>();
        let id = id.unwrap();

        let cards = parts[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = cards[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        //println!("{:?}", cards[1]);
        let numbers = cards[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Self {
            id,
            winning_numbers,
            numbers,
            copies: 1,
        }
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let cards = _input.lines().map(Card::from).collect::<Vec<Card>>();
    let total_points = cards.iter().map(|card| card.points()).sum::<u32>();
    Some(total_points)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut num_cards = 0;
    let mut cards = _input.lines().map(Card::from).collect::<Vec<Card>>();
    for i in 0..cards.len() {
        let matches = cards[i].matches() as usize;
        for j in 1..=matches {
            if i + j >= cards.len() {
                break;
            }
            cards[i + j].copies += cards[i].copies;
        }
        num_cards += cards[i].copies;
    }
    Some(num_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let points = card.points();
        assert_eq!(points, 8);
    }

    #[test]
    fn test_card_from_string() {
        let card = Card::from("Card 1: 1 2 3 4 5 | 6 7 8 9 10");
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(card.numbers, vec![6, 7, 8, 9, 10]);

        let card = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

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
