use std::ops::Range;

fn main() {
    // solve_part_1();

    solve_part_2();
}

fn solve_part_2() {
    let input = include_str!("assets/day5/input");

    let input = input
        .split("\n\n")
        .map(|content| {
            let content: Vec<_> = content.split(":").filter(|s| !s.is_empty()).collect();
            content
        })
        .collect::<Vec<_>>();

    let ranged_seeds = input.get(0).unwrap().get(1).unwrap().into_ranged_seeds();

    let seeds = ranged_seeds
        .ranges
        .iter()
        .fold(Vec::new(), |mut acc, range| {
            for i in range.clone() {
                acc.push(Seed(i));
            }
            acc
        });

    let seed_to_soil = input
        .get(1)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let soil_to_fertilizer = input
        .get(2)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let fertilizer_to_water = input
        .get(3)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let water_to_light = input
        .get(4)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let light_to_temperature = input
        .get(5)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let temperature_to_humidity = input
        .get(6)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let humidity_to_location = input
        .get(7)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let soils = seeds
        .iter()
        .map(|seed| seed.convert_to(&seed_to_soil))
        .collect::<Vec<_>>();

    let locations = soils
        .iter()
        .map(|soil| soil.convert_to(&soil_to_fertilizer))
        .map(|fertilizer| fertilizer.convert_to(&fertilizer_to_water))
        .map(|water| water.convert_to(&water_to_light))
        .map(|light| light.convert_to(&light_to_temperature))
        .map(|temperature| temperature.convert_to(&temperature_to_humidity))
        .map(|humidity| humidity.convert_to(&humidity_to_location))
        .collect::<Vec<_>>();

    let min_location = locations.iter().map(|location| location.0).min().unwrap();

    println!("min location: {}", min_location);
}

fn solve_part_1() {
    let input = include_str!("assets/day5/input");

    let input = input
        .split("\n\n")
        .map(|content| {
            let content: Vec<_> = content.split(":").filter(|s| !s.is_empty()).collect();
            content
        })
        .collect::<Vec<_>>();

    let seeds = input
        .get(0)
        .unwrap()
        .get(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| Seed(s.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let seed_to_soil = input
        .get(1)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let soil_to_fertilizer = input
        .get(2)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let fertilizer_to_water = input
        .get(3)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let water_to_light = input
        .get(4)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let light_to_temperature = input
        .get(5)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let temperature_to_humidity = input
        .get(6)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let humidity_to_location = input
        .get(7)
        .unwrap()
        .get(1)
        .unwrap()
        .trim_start_matches("\n")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.into_conversion_map())
        .collect::<Vec<_>>();

    let soils = seeds
        .iter()
        .map(|seed| seed.convert_to(&seed_to_soil))
        .collect::<Vec<_>>();

    let locations = soils
        .iter()
        .map(|soil| soil.convert_to(&soil_to_fertilizer))
        .map(|fertilizer| fertilizer.convert_to(&fertilizer_to_water))
        .map(|water| water.convert_to(&water_to_light))
        .map(|light| light.convert_to(&light_to_temperature))
        .map(|temperature| temperature.convert_to(&temperature_to_humidity))
        .map(|humidity| humidity.convert_to(&humidity_to_location))
        .collect::<Vec<_>>();

    let min_location = locations.iter().map(|location| location.0).min().unwrap();

    println!("min location: {}", min_location);
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
    fn into_ranged_seeds(&self) -> RangedSeeds;
}

impl IntoRangedSeeds for &str {
    fn into_ranged_seeds(&self) -> RangedSeeds {
        let nums = self
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let ranges = nums
            .chunks(2)
            .map(|chunk| {
                let start = *chunk.get(0).unwrap();
                let len = *chunk.get(1).unwrap();
                start..start + len
            })
            .collect::<Vec<_>>();

        RangedSeeds { ranges }
    }
}

impl ConvertTo<Soil> for Seed {
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Soil {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Fertilizer {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Water {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Light {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Temperature {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Humidity {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> Location {
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
    fn convert_to(&self, conversion_maps: &Vec<ConversionMap>) -> T;
}

#[derive(Debug)]
struct ConversionMap {
    destination: Range<usize>,
    source: Range<usize>,
}

trait IntoConversionMap {
    fn into_conversion_map(&self) -> ConversionMap;
}

impl IntoConversionMap for &str {
    fn into_conversion_map(&self) -> ConversionMap {
        let nums: Vec<_> = self
            .trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let destination_start = *nums.get(0).unwrap();
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
    use crate::{ConvertTo, IntoConversionMap, IntoRangedSeeds, Seed};

    #[test]
    fn it_works() {
        let input = include_str!("assets/day5/input_test");

        let input = input
            .split("\n\n")
            .map(|content| {
                let content: Vec<_> = content.split(":").filter(|s| !s.is_empty()).collect();
                content
            })
            .collect::<Vec<_>>();

        let seeds = input
            .get(0)
            .unwrap()
            .get(1)
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| Seed(s.parse::<usize>().unwrap()))
            .collect::<Vec<_>>();

        let seed_to_soil = input
            .get(1)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let soil_to_fertilizer = input
            .get(2)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let fertilizer_to_water = input
            .get(3)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let water_to_light = input
            .get(4)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let light_to_temperature = input
            .get(5)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let temperature_to_humidity = input
            .get(6)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let humidity_to_location = input
            .get(7)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let soils = seeds
            .iter()
            .map(|seed| seed.convert_to(&seed_to_soil))
            .collect::<Vec<_>>();

        let locations = soils
            .iter()
            .map(|soil| soil.convert_to(&soil_to_fertilizer))
            .map(|fertilizer| fertilizer.convert_to(&fertilizer_to_water))
            .map(|water| water.convert_to(&water_to_light))
            .map(|light| light.convert_to(&light_to_temperature))
            .map(|temperature| temperature.convert_to(&temperature_to_humidity))
            .map(|humidity| humidity.convert_to(&humidity_to_location))
            .collect::<Vec<_>>();

        let min_location = locations.iter().map(|location| location.0).min().unwrap();

        assert_eq!(min_location, 35);
    }

    #[test]
    fn it_still_works() {
        let input = include_str!("assets/day5/input_test");

        let input = input
            .split("\n\n")
            .map(|content| {
                let content: Vec<_> = content.split(":").filter(|s| !s.is_empty()).collect();
                content
            })
            .collect::<Vec<_>>();

        let ranged_seeds = input.get(0).unwrap().get(1).unwrap().into_ranged_seeds();

        let seeds = ranged_seeds
            .ranges
            .iter()
            .fold(Vec::new(), |mut acc, range| {
                for i in range.clone() {
                    acc.push(Seed(i));
                }
                acc
            });

        assert_eq!(seeds.len(), 27);

        let seed_to_soil = input
            .get(1)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let soil_to_fertilizer = input
            .get(2)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let fertilizer_to_water = input
            .get(3)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let water_to_light = input
            .get(4)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let light_to_temperature = input
            .get(5)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let temperature_to_humidity = input
            .get(6)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let humidity_to_location = input
            .get(7)
            .unwrap()
            .get(1)
            .unwrap()
            .trim_start_matches("\n")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.into_conversion_map())
            .collect::<Vec<_>>();

        let soils = seeds
            .iter()
            .map(|seed| seed.convert_to(&seed_to_soil))
            .collect::<Vec<_>>();

        let locations = soils
            .iter()
            .map(|soil| soil.convert_to(&soil_to_fertilizer))
            .map(|fertilizer| fertilizer.convert_to(&fertilizer_to_water))
            .map(|water| water.convert_to(&water_to_light))
            .map(|light| light.convert_to(&light_to_temperature))
            .map(|temperature| temperature.convert_to(&temperature_to_humidity))
            .map(|humidity| humidity.convert_to(&humidity_to_location))
            .collect::<Vec<_>>();

        let min_location = locations.iter().map(|location| location.0).min().unwrap();

        assert_eq!(min_location, 46);
    }
}
