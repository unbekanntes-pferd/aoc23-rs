struct Race {
    distance: u64,
    duration: u64,
}

impl From<(u64, u64)> for Race {
    fn from((distance, duration): (u64, u64)) -> Self {
        Self { distance, duration }
    }
}


fn solve_part1(races: Vec<Race>) -> u64 {

    let result = races.iter().map(|race| {

        (1..=race.duration).filter(|hold| {
            let velocity = *hold;

            race.distance < (velocity * (race.duration - hold)) 
        }).count()

    }).fold(1, |acc, count| acc * count);

    result as u64
}

fn solve_part2(race: Race) -> u64 {
    // faster for larger numbers 

    let low = (1..race.duration).find(|hold| {
        let velocity = *hold as u128;

        (race.distance as u128) < (velocity * (race.duration as u128 - *hold as u128)) 
    }).unwrap();

    let high = (1..race.duration).rev().find(|hold| {
        let velocity = *hold as u128;

        (race.distance as u128) < (velocity * (race.duration as u128 - *hold as u128)) 
    }).unwrap();

    high - low + 1
}

fn main() {
    let input = include_str!("assets/day6/input");

    let times = input.lines().collect::<Vec<_>>().get(0).unwrap().split(" ").filter(|s| {
        !s.is_empty() && s.parse::<u64>().is_ok()
    })
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<_>>();

    let distances = input.lines().collect::<Vec<_>>().get(1).unwrap().split(" ").filter(|s| {
        !s.is_empty() && s.parse::<u64>().is_ok()
    })
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<_>>();

    let races = times.iter().enumerate().map(|(index, time)| {
        let distance = *distances.get(index).unwrap();

        Race::from((distance, *time))
    }).collect();

    let result = solve_part1(races);

    println!("Result: {}", result);

    let input = include_str!("assets/day6/input");

    let nums: Vec<_> = input.lines().map(|line| line.split(" ").filter(|s| !s.is_empty() && s.parse::<u32>().is_ok()).collect::<Vec<_>>().join("").parse::<u64>().unwrap()).collect();

    let game = Race::from((*nums.get(1).unwrap(), *nums.get(0).unwrap()));

    let result = solve_part2(game);

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("assets/day6/input_test");

        let times = input.lines().collect::<Vec<_>>().get(0).unwrap().split(" ").filter(|s| {
            !s.is_empty() && s.parse::<u64>().is_ok()
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

        let distances = input.lines().collect::<Vec<_>>().get(1).unwrap().split(" ").filter(|s| {
            !s.is_empty() && s.parse::<u64>().is_ok()
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

        let races = times.iter().enumerate().map(|(index, time)| {
            let distance = *distances.get(index).unwrap();

            Race::from((distance, *time))
        }).collect();

        assert_eq!(solve_part1(races), 288);
    }

    #[test]
    fn test_part2() {

        let input = include_str!("assets/day6/input_test");

        let nums: Vec<_> = input.lines().map(|line| line.split(" ").filter(|s| !s.is_empty() && s.parse::<u32>().is_ok()).collect::<Vec<_>>().join("").parse::<u64>().unwrap()).collect();
    
        let race = Race::from((*nums.get(1).unwrap(), *nums.get(0).unwrap()));

        assert_eq!(race.duration, 71530);
        assert_eq!(race.distance, 940200);
    
        let result = solve_part2(race);

        assert_eq!(result, 71503);
    }
}
