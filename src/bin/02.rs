advent_of_code::solution!(2);

struct Move {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Move {
    pub fn from_str(input: &str) -> Self {
        let mut m = Move {
            red: 0,
            green: 0,
            blue: 0,
        };

        let cubes = input.split(",").for_each(|el| {
            let el = el.trim();
            let (amount, color) = el.split_once(" ").unwrap();
            let amount = amount.parse::<u32>().unwrap();

            match color {
                "red" => m.red = amount,
                "green" => m.green = amount,
                "blue" => m.blue = amount,
                _ => panic!(""),
            };
        });
        m
    }
}

struct Game {
    pub moves: Vec<Move>,
    pub id: u32,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let (game_str, move_line) = input.split_once(":").unwrap();
        let game_id = game_str.split(" ").last().unwrap();

        Self {
            id: game_id.parse::<u32>().unwrap(),
            moves: move_line
                .split(";")
                .map(|move_str| Move::from_str(move_str))
                .collect(),
        }
    }

    fn game_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.moves.iter().fold(true, |acc, el| {
            acc && (el.red <= red && el.green <= green && el.blue <= blue)
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(|el| Game::from_str(el)).collect();

    Some(
        games
            .iter()
            .filter(|el| el.game_possible(12, 13, 14))
            .map(|el| el.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(|el| Game::from_str(el)).collect();

    Some(
        games
            .iter()
            .map(|game| {
                let min_required = game.moves.iter().fold(
                    Move {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |acc, el| Move {
                        red: u32::max(acc.red, el.red),
                        green: u32::max(acc.green, el.green),
                        blue: u32::max(acc.blue, el.blue),
                    },
                );
                min_required.red * min_required.green * min_required.blue
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
