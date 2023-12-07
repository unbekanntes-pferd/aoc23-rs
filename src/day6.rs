struct Race {
    distance: u32,
    duration: u32,
}

impl From<(u32, u32)> for Race {
    fn from((distance, duration): (u32, u32)) -> Self {
        Self { distance, duration }
    }
}


fn solve_part1(races: Vec<Race>) -> u64 {

    let result = races.iter().map(|race| {

        (1..race.duration).into_iter().filter(|hold| {
            let velocity = *hold;

            race.distance < (velocity * (race.duration - hold)) 
        }).count()

    }).fold(1, |acc, count| acc * count);

    result as u64
}

fn solve_part2() {}

fn main() {
    let input = include_str!("assets/day6/input");

    let times = input.lines().collect::<Vec<_>>().get(0).unwrap().split(" ").filter(|s| {
        !s.is_empty() && s.parse::<u32>().is_ok()
    })
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<_>>();

    let distances = input.lines().collect::<Vec<_>>().get(1).unwrap().split(" ").filter(|s| {
        !s.is_empty() && s.parse::<u32>().is_ok()
    })
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<_>>();

    let races = times.iter().enumerate().map(|(index, time)| {
        let distance = *distances.get(index).unwrap();

        Race::from((distance, *time))
    }).collect();


    let result = solve_part1(races);

    println!("Result: {}", result);


    //solve_part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("assets/day6/input_test");

        let times = input.lines().collect::<Vec<_>>().get(0).unwrap().split(" ").filter(|s| {
            !s.is_empty() && s.parse::<u32>().is_ok()
        })
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

        let distances = input.lines().collect::<Vec<_>>().get(1).unwrap().split(" ").filter(|s| {
            !s.is_empty() && s.parse::<u32>().is_ok()
        })
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

        let races = times.iter().enumerate().map(|(index, time)| {
            let distance = *distances.get(index).unwrap();

            Race::from((distance, *time))
        }).collect();

        assert_eq!(solve_part1(races), 288);
    }

    #[test]
    fn test_part2() {}
}