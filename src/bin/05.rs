advent_of_code::solution!(5);

enum ParserState {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn feed_hashmap(line: &str, hm: &mut MyHashMap) {
    let numbers: Vec<u32> = line
        .split(' ')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let destination_range_start = numbers[0];
    let source_range_start = numbers[1];
    let range_length = numbers[2];
    hm.insert(source_range_start, destination_range_start, range_length);
}

struct MyHashMap {
    ranges: Vec<(u32, u32, u32)>,
}

impl MyHashMap {
    fn get(&self, key: u32) -> u32 {
        for (source_start, destination_start, range) in self.ranges.iter() {
            if (key >= *source_start) && (key < *source_start + *range) {
                return key - source_start + destination_start;
            }
        }
        key
    }

    fn insert(&mut self, source_start: u32, destination_start: u32, range: u32) {
        self.ranges.push((source_start, destination_start, range));
    }

    fn new() -> Self {
        Self { ranges: vec![] }
    }
}

struct Problem {
    seeds: Vec<u32>,
    seed_to_soil: MyHashMap,
    soil_to_fertilizer: MyHashMap,
    fertilizer_to_water: MyHashMap,
    water_to_light: MyHashMap,
    light_to_temperature: MyHashMap,
    temperature_to_humidity: MyHashMap,
    humidity_to_location: MyHashMap,
}

fn parse_input(input: &str) -> Problem {
    let mut seed_to_soil_hashmap = MyHashMap::new();
    let mut soil_to_fertilizer_hashmap = MyHashMap::new();
    let mut fertilizer_to_water_hashmap = MyHashMap::new();
    let mut water_to_light_hashmap = MyHashMap::new();
    let mut light_to_temperature_hashmap = MyHashMap::new();
    let mut temperature_to_humidity_hashmap = MyHashMap::new();
    let mut humidity_to_location_hashmap = MyHashMap::new();

    // Split input by emtpy lines
    let blocks = input.lines().collect::<Vec<&str>>();

    let seed_numbers = &blocks[0].split(' ').collect::<Vec<&str>>()[1..];
    let seed_numbers = seed_numbers
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    //println!("{:?}", seed_numbers);

    let mut state = ParserState::SeedToSoil;
    let blocks = input.lines().collect::<Vec<&str>>();
    for line in blocks.iter().skip(1) {
        if line.contains("seed-to-soil map:") {
            state = ParserState::SeedToSoil;
        } else if line.contains("soil-to-fertilizer map:") {
            state = ParserState::SoilToFertilizer;
        } else if line.contains("fertilizer-to-water map:") {
            state = ParserState::FertilizerToWater;
        } else if line.contains("water-to-light map:") {
            state = ParserState::WaterToLight;
        } else if line.contains("light-to-temperature map:") {
            state = ParserState::LightToTemperature;
        } else if line.contains("temperature-to-humidity map:") {
            state = ParserState::TemperatureToHumidity;
        } else if line.contains("humidity-to-location map:") {
            state = ParserState::HumidityToLocation;
        } else if line.is_empty() {
            continue;
        } else {
            match state {
                ParserState::SeedToSoil => feed_hashmap(line, &mut seed_to_soil_hashmap),
                ParserState::SoilToFertilizer => {
                    feed_hashmap(line, &mut soil_to_fertilizer_hashmap)
                }
                ParserState::FertilizerToWater => {
                    feed_hashmap(line, &mut fertilizer_to_water_hashmap)
                }
                ParserState::WaterToLight => feed_hashmap(line, &mut water_to_light_hashmap),
                ParserState::LightToTemperature => {
                    feed_hashmap(line, &mut light_to_temperature_hashmap)
                }
                ParserState::TemperatureToHumidity => {
                    feed_hashmap(line, &mut temperature_to_humidity_hashmap)
                }
                ParserState::HumidityToLocation => {
                    feed_hashmap(line, &mut humidity_to_location_hashmap)
                }
            }
        }
    }
    Problem {
        seeds: seed_numbers,
        seed_to_soil: seed_to_soil_hashmap,
        soil_to_fertilizer: soil_to_fertilizer_hashmap,
        fertilizer_to_water: fertilizer_to_water_hashmap,
        water_to_light: water_to_light_hashmap,
        light_to_temperature: light_to_temperature_hashmap,
        temperature_to_humidity: temperature_to_humidity_hashmap,
        humidity_to_location: humidity_to_location_hashmap,
    }
}

fn map_all_seeds(seed: u32, problem: &Problem) -> u32 {
    let soil = problem.seed_to_soil.get(seed);
    let fertilizer = problem.soil_to_fertilizer.get(soil);
    let water = problem.fertilizer_to_water.get(fertilizer);
    let light = problem.water_to_light.get(water);
    let temperature = problem.light_to_temperature.get(light);
    let humidity = problem.temperature_to_humidity.get(temperature);
    problem.humidity_to_location.get(humidity)
}

pub fn part_one(input: &str) -> Option<u32> {
    let problem = parse_input(input);

    let min_location = problem
        .seeds
        .iter()
        .map(|x| map_all_seeds(*x, &problem))
        .min()
        .unwrap();

    Some(min_location)
}

pub fn part_two(input: &str) -> Option<u32> {
    let problem = parse_input(input);

    let min_location = problem
        .seeds
        .iter()
        .step_by(2)
        .cloned()
        .zip(problem.seeds.iter().skip(1).step_by(2).cloned())
        .flat_map(|(start, range)| (start..(start + range)))
        .map(|x| map_all_seeds(x, &problem))
        .min()
        .unwrap();

    Some(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_hashmap() {
        let mut my_hm = MyHashMap::new();
        my_hm.insert(2, 20, 3);
        assert_eq!(my_hm.get(2), 20);
        assert_eq!(my_hm.get(3), 21);
        assert_eq!(my_hm.get(4), 22);
        assert_eq!(my_hm.get(5), 5);
        assert_eq!(my_hm.get(1), 1);
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let problem = parse_input(&input);
        assert_eq!(problem.seeds, vec![79, 14, 55, 13]);
        assert_eq!(problem.seed_to_soil.get(79), 81);
        assert_eq!(problem.seed_to_soil.get(14), 14);
        assert_eq!(problem.seed_to_soil.get(55), 57);
        assert_eq!(problem.seed_to_soil.get(13), 13);

        assert_eq!(problem.seed_to_soil.get(79), 81);
        assert_eq!(problem.soil_to_fertilizer.get(81), 81);
        assert_eq!(problem.fertilizer_to_water.get(81), 81);
        assert_eq!(problem.water_to_light.get(81), 74);
        assert_eq!(problem.light_to_temperature.get(74), 78);
        assert_eq!(problem.temperature_to_humidity.get(78), 78);
        assert_eq!(problem.humidity_to_location.get(78), 82);
    }

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
