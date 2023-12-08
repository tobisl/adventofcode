advent_of_code::solution!(3);

type Position = (i32, i32);
type Length = usize;

#[derive(Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
enum Item {
    Symbol(Position),
    Part(u32, Position, Length),
}

impl Item {
    fn neighbors(&self) -> Vec<Position> {
        match self {
            Item::Symbol(mypos) => {
                let (row, col) = mypos;
                vec![
                    (row - 1, col - 1),
                    (row - 1, *col),
                    (row - 1, col + 1),
                    (*row, col - 1),
                    (*row, col + 1),
                    (row + 1, col - 1),
                    (row + 1, *col),
                    (row + 1, col + 1),
                ]
            }
            Item::Part(_, mypos, length) => {
                let (row, col) = mypos;
                let mut neighbors: Vec<(i32, i32)> = Vec::new();
                neighbors.push((*row, col - 1));
                neighbors.push((*row + 1, col - 1));
                neighbors.push((*row - 1, col - 1));
                for i in 0..*length {
                    neighbors.push((row - 1, *col + i as i32));
                    neighbors.push((row + 1, *col + i as i32));
                }
                neighbors.push((*row, *col + *length as i32));
                neighbors.push((*row + 1, *col + *length as i32));
                neighbors.push((*row - 1, *col + *length as i32));
                neighbors.sort();
                neighbors.dedup();
                assert!(neighbors.len() == 2 * *length + 6, "length: {}", length);
                neighbors
            }
        }
    }
}

fn parse_symbols(input: &str) -> Vec<Item> {
    let mut symbols = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.char_indices() {
            // if '.' there is nothing, if digit, there is no symbol
            if c == '.' || c.is_ascii_digit() {
                continue;
            } else {
                symbols.push(Item::Symbol((row as i32, col as i32)));
            }
        }
    }
    symbols
}

fn parse_parts(input: &str) -> Vec<Item> {
    let mut parts = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut current_string = String::new();
        for (col, c) in line.char_indices() {
            // if digit, then add to length
            if c.is_ascii_digit() {
                current_string.push(c);
            } else if !current_string.is_empty() {
                let val = current_string.parse::<u32>().unwrap();
                let length = current_string.len();
                let position = (row as i32, col as i32 - length as i32);
                parts.push(Item::Part(val, position, length));
                current_string.clear();
            }
        }
        if !current_string.is_empty() {
            let val = current_string.parse::<u32>().unwrap();
            let length = current_string.len();
            let col = line.len();
            let position = (row as i32, col as i32 - length as i32);
            parts.push(Item::Part(val, position, length));
            current_string.clear();
        }
    }
    parts
}

fn part_has_symbol_neighbor(part: &Item, symbols: &[Item]) -> bool {
    let symbol_positions: Vec<Position> = symbols
        .iter()
        .map(|item| match item {
            Item::Symbol(pos) => *pos,
            _ => panic!("Should not happen"),
        })
        .collect();

    part.neighbors()
        .iter()
        .any(|neighbor| symbol_positions.binary_search(neighbor).is_ok())
}

fn symbol_has_two_part_neighbors(symbol: &Item, parts: &[Item]) -> Option<(Item, Item)> {
    let _symbol_position = match symbol {
        Item::Symbol(pos) => *pos,
        _ => panic!("Should not happen"),
    };

    let mut part_neighbors = Vec::new();
    for part in parts {
        if part_has_symbol_neighbor(part, &[*symbol]) {
            part_neighbors.push(*part);
        }
    }

    if part_neighbors.len() == 2 {
        Some((part_neighbors[0], part_neighbors[1]))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut symbols = parse_symbols(input);
    symbols.sort();
    symbols.dedup();
    let symbols = symbols;

    let parts = parse_parts(input);

    let relevant_parts: Vec<Item> = parts
        .iter()
        .filter(|part| part_has_symbol_neighbor(part, &symbols))
        .copied()
        .collect();

    let relevant_parts_sum = relevant_parts
        .iter()
        .map(|x| match x {
            Item::Part(val, _, _) => *val,
            _ => panic!("Should not happen"),
        })
        .sum();

    Some(relevant_parts_sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut symbols = parse_symbols(_input);
    symbols.sort();
    symbols.dedup();
    let symbols = symbols;

    let parts = parse_parts(_input);

    let gear_ratios: Vec<u32> = symbols
        .iter()
        .map(|sym| symbol_has_two_part_neighbors(sym, &parts))
        .filter(|x| x.is_some())
        .map(|x| match x {
            Some((a, b)) => (a, b),
            _ => panic!("Should not happen"),
        })
        .map(|(a, b)| {
            let (a_val, _a_pos, _a_len) = match a {
                Item::Part(val, pos, len) => (val, pos, len),
                _ => panic!("Should not happen"),
            };
            let (b_val, _b_pos, _b_len) = match b {
                Item::Part(val, pos, len) => (val, pos, len),
                _ => panic!("Should not happen"),
            };
            a_val * b_val
        })
        .collect();

    Some(gear_ratios.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_input() {
        let input = ".........925....\n373......*......\n.*....647.......\n923.........=866\n........759.....\n........-....832\n............*...\n.......83...49..\n"
            .to_string();
        println!("{}", input);
        let symbols = parse_symbols(&input);
        assert_eq!(symbols.len(), 5);
        let parts = parse_parts(&input);
        assert_eq!(parts.len(), 9);

        let relevant_parts: Vec<Item> = parts
            .iter()
            .filter(|part| part_has_symbol_neighbor(part, &symbols))
            .copied()
            .collect();

        println!("{:?}", symbols);
        println!("{:?}", parts);
        println!("{:?}", relevant_parts);

        let result = part_one(&input);
        assert_eq!(result, Some(5374));
    }

    #[test]
    fn test_parse_symbols() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let symbols = parse_symbols(input);
        assert_eq!(symbols.len(), 6);
        assert!(symbols.contains(&Item::Symbol((1, 3))));
        assert!(symbols.contains(&Item::Symbol((3, 6))));
        assert!(symbols.contains(&Item::Symbol((4, 3))));
        assert!(symbols.contains(&Item::Symbol((5, 5))));
        assert!(symbols.contains(&Item::Symbol((8, 3))));
        assert!(symbols.contains(&Item::Symbol((8, 5))));
    }

    #[test]
    fn test_parse_parts() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let parts = parse_parts(input);
        assert_eq!(parts.len(), 10);
        assert!(parts.contains(&Item::Part(467, (0, 0), 3)));
        assert!(parts.contains(&Item::Part(114, (0, 5), 3)));
        assert!(parts.contains(&Item::Part(35, (2, 2), 2)));
        assert!(parts.contains(&Item::Part(633, (2, 6), 3)));
        assert!(parts.contains(&Item::Part(617, (4, 0), 3)));
    }

    #[test]
    fn test_neighbors() {
        let item = Item::Symbol((0, 0));
        let neighbors = item.neighbors();
        let mut expected = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        expected.sort();
        expected.dedup();
        assert_eq!(neighbors, expected);
        let item = Item::Part(467, (0, 0), 3);
        let neighbors = item.neighbors();
        let mut expected = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (-1, 2),
            (-1, 3),
            (0, -1),
            (0, 3),
            (1, -1),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
        ];
        expected.sort();
        expected.dedup();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one(input);
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
