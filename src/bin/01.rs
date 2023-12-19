advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line_digits = line
                    .chars()
                    .filter(|el| el.is_digit(10))
                    .collect::<Vec<_>>();

                let num_collected = format!(
                    "{}{}",
                    line_digits.first().unwrap(),
                    line_digits.last().unwrap()
                );

                num_collected.parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
