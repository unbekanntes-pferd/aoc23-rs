use std::ops::Range;

fn main() {
    let input = include_str!("assets/day5/input");
    // let (seeds, conversion_maps) = solve_part_1(input);
    let (seeds, conversion_maps) = solve_part_2(input);
    let result = calculate_location(&seeds, &conversion_maps);
    println!("Result: {}", result);
}

fn calculate_location(seeds: &[Seed], conversion_maps: &[Vec<ConversionMap>]) -> usize {
    let locations = seeds
        .iter()
        .map(|seed| seed.convert_to(&conversion_maps[0]))
        .map(|soil| soil.convert_to(&conversion_maps[1]))
        .map(|fertilizer| fertilizer.convert_to(&conversion_maps[2]))
        .map(|water| water.convert_to(&conversion_maps[3]))
        .map(|light| light.convert_to(&conversion_maps[4]))
        .map(|temperature| temperature.convert_to(&conversion_maps[5]))
        .map(|humidity| humidity.convert_to(&conversion_maps[6]))
        .collect::<Vec<_>>();

    locations.iter().map(|location| location.0).min().unwrap()
}

fn solve_part_2(input: &str) -> (Vec<Seed>, Vec<Vec<ConversionMap>>) {
    let input = input
        .split("\n\n")
        .map(|content| {
            let content: Vec<_> = content.split(':').filter(|s| !s.is_empty()).collect();
            content
        })
        .collect::<Vec<_>>();

    let ranged_seeds = input.first().unwrap().get(1).unwrap().to_ranged_seeds();

    let seeds = ranged_seeds
        .ranges
        .iter()
        .fold(Vec::new(), |mut acc, range| {
            for i in range.clone() {
                acc.push(Seed(i));
            }
            acc
        });

    let conversion_maps = (1..8)
        .map(|i| {
            let conversion_map = input
                .get(i)
                .unwrap()
                .get(1)
                .unwrap()
                .trim_start_matches('\n')
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_conversion_map())
                .collect::<Vec<_>>();
            conversion_map
        })
        .collect::<Vec<_>>();

    (seeds, conversion_maps)
}

#[allow(dead_code)]
fn solve_part_1(input: &str) -> (Vec<Seed>, Vec<Vec<ConversionMap>>) {
    let input = input
        .split("\n\n")
        .map(|content| {
            let content: Vec<_> = content.split(':').filter(|s| !s.is_empty()).collect();
            content
        })
        .collect::<Vec<_>>();

    let seeds = input
        .first()
        .unwrap()
        .get(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| Seed(s.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let conversion_maps = (1..8)
        .map(|i| {
            let conversion_map = input
                .get(i)
                .unwrap()
                .get(1)
                .unwrap()
                .trim_start_matches('\n')
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_conversion_map())
                .collect::<Vec<_>>();
            conversion_map
        })
        .collect::<Vec<_>>();

    (seeds, conversion_maps)
}

#[derive(Debug)]
struct Seed(usize);
#[derive(Debug)]
struct Soil(usize);
#[derive(Debug)]
struct Fertilizer(usize);
#[derive(Debug)]
struct Water(usize);
#[derive(Debug)]
struct Light(usize);
#[derive(Debug)]
struct Temperature(usize);
#[derive(Debug)]
struct Humidity(usize);
#[derive(Debug)]
struct Location(usize);

struct RangedSeeds {
    ranges: Vec<Range<usize>>,
}

trait IntoRangedSeeds {
    fn to_ranged_seeds(&self) -> RangedSeeds;
}

impl IntoRangedSeeds for &str {
    fn to_ranged_seeds(&self) -> RangedSeeds {
        let nums = self
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let ranges = nums
            .chunks(2)
            .map(|chunk| {
                let start = *chunk.first().unwrap();
                let len = *chunk.get(1).unwrap();
                start..start + len
            })
            .collect::<Vec<_>>();

        RangedSeeds { ranges }
    }
}

impl ConvertTo<Soil> for Seed {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Soil {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Soil(conversion_map.destination.start + idx);
            }
        }
        Soil(self.0)
    }
}

impl ConvertTo<Fertilizer> for Soil {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Fertilizer {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Fertilizer(conversion_map.destination.start + idx);
            }
        }
        Fertilizer(self.0)
    }
}

impl ConvertTo<Water> for Fertilizer {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Water {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Water(conversion_map.destination.start + idx);
            }
        }
        Water(self.0)
    }
}

impl ConvertTo<Light> for Water {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Light {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Light(conversion_map.destination.start + idx);
            }
        }
        Light(self.0)
    }
}

impl ConvertTo<Temperature> for Light {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Temperature {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Temperature(conversion_map.destination.start + idx);
            }
        }
        Temperature(self.0)
    }
}

impl ConvertTo<Humidity> for Temperature {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Humidity {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Humidity(conversion_map.destination.start + idx);
            }
        }
        Humidity(self.0)
    }
}

impl ConvertTo<Location> for Humidity {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> Location {
        for conversion_map in conversion_maps {
            if conversion_map.source.contains(&self.0) {
                let idx = self.0 - conversion_map.source.start;
                return Location(conversion_map.destination.start + idx);
            }
        }
        Location(self.0)
    }
}

trait ConvertTo<T> {
    fn convert_to(&self, conversion_maps: &[ConversionMap]) -> T;
}

#[derive(Debug)]
struct ConversionMap {
    destination: Range<usize>,
    source: Range<usize>,
}

trait IntoConversionMap {
    fn to_conversion_map(&self) -> ConversionMap;
}

impl IntoConversionMap for &str {
    fn to_conversion_map(&self) -> ConversionMap {
        let nums: Vec<_> = self
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let destination_start = *nums.first().unwrap();
        let source_start = *nums.get(1).unwrap();
        let len = nums.get(2).unwrap();

        let destination = destination_start..destination_start + len;
        let source = source_start..source_start + len;

        ConversionMap {
            destination,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, calculate_location, solve_part_2};

    #[test]
    fn it_works() {
        let input = include_str!("assets/day5/input_test");

        let (seeds, conversion_maps) = solve_part_1(input);
        let min_location = calculate_location(&seeds, &conversion_maps);

        assert_eq!(min_location, 35);
    }

    #[test]
    fn it_still_works() {
        let input = include_str!("assets/day5/input_test");

        let (seeds, conversion_maps) = solve_part_2(input);
        let min_location = crate::calculate_location(&seeds, &conversion_maps);
        assert_eq!(min_location, 46);
    }
}
