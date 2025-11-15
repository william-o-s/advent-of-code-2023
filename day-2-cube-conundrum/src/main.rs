use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32
}

impl Game {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn merge(&self, other: &Game) -> Game {
        Game {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Game {
    fn from(str: &str) -> Self {
        let mut game = Self { red: 0, green: 0, blue: 0 };

        for cubes in str.splitn(3, ", ") {
            if let Some((num, color)) = cubes.split_once(" ") {
                match color {
                    "red" => game.red = num.parse().unwrap(),
                    "green" => game.green = num.parse().unwrap(),
                    "blue" => game.blue = num.parse().unwrap(),
                    _ => ()
                }
            }
        }

        game
    }
}

fn main() {
    let filepath = "src/input.txt";

    let contents = fs::read_to_string(filepath)
        .expect("Please provide a valid input.txt file.");

    let mut total = 0;
    let mut sum = 0;
    for line in contents.lines() {
        if let Some((game_id, games)) = line.split_once(": ") {
            // Part 1
            let pass = games.split("; ").map(Game::from).all(|game| game.is_valid());
            if pass {
                let id = game_id
                    .split_once(" ")
                    .map(|(_, id)| id.parse::<i32>().unwrap())
                    .unwrap();
                dbg!(id);
                total += id;

            }

            // Part 2
            let empty_game = Game { red: 0, green: 0, blue: 0 };
            let powers = games
                .split("; ")
                .map(Game::from)
                .fold(empty_game, |acc, g| acc.merge(&g))
                .power();
            sum += powers;
        }
    }

    println!("Total: {total}");
    println!("Sum of Powers: {sum}");
}
