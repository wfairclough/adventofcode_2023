use std::fs::File;
use std::io::prelude::*;

type Count = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cube {
    Red(Count),
    Green(Count),
    Blue(Count),
}

#[derive(Debug ,Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Hand {
    Rgb(Cube, Cube, Cube),
}

impl Ord for Hand {
    // A hand is less than another if red < red && green < green && blue < blue 
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        match (self, other) {
            (Hand::Rgb(Cube::Red(r1), Cube::Green(g1), Cube::Blue(b1)), Hand::Rgb(Cube::Red(r2), Cube::Green(g2), Cube::Blue(b2))) => {
                println!("{}/{} {}/{} {}/{}", r1, r2, g1, g2, b1, b2);
                if r1 > r2 || g1 > g2 || b1 > b2 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            },
            _ => panic!("Invalid hand"),
        }
    }
}

type Bag = Hand;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    hands: Vec<Hand>,
    game_number: u32,
}

trait IsValidForBag {
    fn is_valid_for_bag(&self, bag: &Bag) -> bool;
}

impl IsValidForBag for Hand {
    fn is_valid_for_bag(&self, bag: &Bag) -> bool {
        !self.cmp(bag).is_gt()
    }
}

impl IsValidForBag for Game {
    fn is_valid_for_bag(&self, bag: &Bag) -> bool {
        for hand in &self.hands {
            if !hand.is_valid_for_bag(bag) {
                return false;
            }
        }
        true
    }
}


// Incorrect Guesses:
// 3472
fn main() {
    let games = parse_file("input");
    // let games = parse_input("sample_input");
    let bag = Bag::Rgb(Cube::Red(12), Cube::Green(13), Cube::Blue(14));
    let mut sum = 0;
    for game in games {
        let is_valid = game.is_valid_for_bag(&bag);
        println!("{} {:?}", is_valid, game);
        if is_valid {
            sum += game.game_number;
        }
    }
    println!("Sum: {}", sum);
}

// parse the input file into a vector of games
// each game has a vector of hands
// each hand has a vector of cubes
// hands are separated by a semicolon
// cubes are separated by a comma and have a color and a Count
// games are separated by a newline
// each game has a game number
//
// Example line: Game 7: 7 green, 7 blue, 2 red; 2 red, 7 green, 16 blue; 17 blue, 3 green, 3 red; 2 blue, 5 green, 3 red
fn parse_file(path: &str) -> Vec<Game> {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    parse_contents(&contents)
}

fn parse_contents(contents: &str) -> Vec<Game> {
    let lines = contents.lines();
    let games = lines.map(|line| parse_game(line)).collect::<Vec<Game>>();
    games
}

fn parse_game(line: &str) -> Game {
    let line = line.trim();
    let mut game = Game {
        hands: Vec::new(),
        game_number: 0,
    };

    println!("{}", line);
    let mut line = line.split(':');
    game.game_number = line.next().unwrap().split(' ').nth(1).unwrap().parse().unwrap();
    game.hands = line.next().unwrap().split(';').map(|hand| parse_hand(hand)).collect();

    game
}

// Example hand: 7 green, 7 blue, 2 red
fn parse_hand(hand_str: &str) -> Hand {
    let hand_str = hand_str.trim();
    if hand_str.is_empty() {
        return Hand::Rgb(Cube::Red(0), Cube::Green(0), Cube::Blue(0));
    }
    let cubes_str = hand_str.split(',');
    let cubes: Vec<Cube> = cubes_str.map(|cube| parse_cube(cube)).collect();

    let red_cubes = cubes.iter().find(|c| match c {
        Cube::Red(_) => true,
        _ => false,
    }).unwrap_or(&Cube::Red(0));

    let green_cubes = cubes.iter().find(|c| match c {
        Cube::Green(_) => true,
        _ => false,
    }).unwrap_or(&Cube::Green(0));

    let blue_cubes = cubes.iter().find(|c| match c {
        Cube::Blue(_) => true,
        _ => false,
    }).unwrap_or(&Cube::Blue(0));

    let hand = Hand::Rgb(red_cubes.clone(), green_cubes.clone(), blue_cubes.clone());
    hand
}

// Example cube: 7 green
fn parse_cube(cube: &str) -> Cube {
    let cube = cube.trim();
    let mut cube_str = cube.split(' ');
    let count: u8 = cube_str.next().unwrap().parse().unwrap();
    let color = cube_str.next().unwrap();
    match color {
        "red" => Cube::Red(count),
        "green" => Cube::Green(count),
        "blue" => Cube::Blue(count),
        _ => panic!("Invalid color: {}", color),
    }
}

// Unit Tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube() {
        assert_eq!(parse_cube("7 green"), Cube::Green(7));
        assert_eq!(parse_cube("8 red"), Cube::Red(8));
        assert_eq!(parse_cube("9 blue"), Cube::Blue(9));
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("3 green, 7 blue, 2 red"), Hand::Rgb(Cube::Red(2), Cube::Green(3), Cube::Blue(7)));
        assert_eq!(parse_hand("11 blue, 2 red"), Hand::Rgb(Cube::Red(2), Cube::Green(0), Cube::Blue(11)));
        assert_eq!(parse_hand("2 red"), Hand::Rgb(Cube::Red(2), Cube::Green(0), Cube::Blue(0)));
        assert_eq!(parse_hand(""), Hand::Rgb(Cube::Red(0), Cube::Green(0), Cube::Blue(0)));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(parse_game("Game 17: 7 green, 7 blue, 2 red; 2 red, 7 green, 16 blue; 17 blue, 3 green, 3 red; 2 blue, 5 green, 3 red"), Game {
            game_number: 17,
            hands: vec![
                Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(7)),
                Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(16)),
                Hand::Rgb(Cube::Red(3), Cube::Green(3), Cube::Blue(17)),
                Hand::Rgb(Cube::Red(3), Cube::Green(5), Cube::Blue(2)),
            ],
        });
    }

    #[test]
    fn test_is_hand_valid_for_bag() {
        let bag = Bag::Rgb(Cube::Red(12), Cube::Green(13), Cube::Blue(14));
        let hand = Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(7));
        assert_eq!(hand.is_valid_for_bag(&bag), true);
        let hand = Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(16));
        assert_eq!(hand.is_valid_for_bag(&bag), false);
        let hand = Hand::Rgb(Cube::Red(12), Cube::Green(13), Cube::Blue(14));
        assert_eq!(hand.is_valid_for_bag(&bag), true);
    }

    #[test]
    fn test_is_game_valid_for_bag() {
        let bag = Bag::Rgb(Cube::Red(12), Cube::Green(13), Cube::Blue(14));
        let game = Game {
            game_number: 17,
            hands: vec![
                Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(7)),
                Hand::Rgb(Cube::Red(2), Cube::Green(7), Cube::Blue(16)),
                Hand::Rgb(Cube::Red(3), Cube::Green(3), Cube::Blue(11)),
                Hand::Rgb(Cube::Red(3), Cube::Green(5), Cube::Blue(2)),
            ],
        };
        assert_eq!(game.is_valid_for_bag(&bag), false);
    }

    #[test]
    fn test_sample_input_contents() {
        let sameple = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let samepl = sameple.trim();
        let games = parse_contents(samepl);
        assert_eq!(games.len(), 5);
        assert_eq!(games[0].game_number, 1);
        assert_eq!(games[0].hands.len(), 3);
        let bag = Bag::Rgb(Cube::Red(12), Cube::Green(13), Cube::Blue(14));
        assert_eq!(games[0].is_valid_for_bag(&bag), true);
        assert_eq!(games[1].is_valid_for_bag(&bag), true);
        assert_eq!(games[2].is_valid_for_bag(&bag), false);
        assert_eq!(games[3].is_valid_for_bag(&bag), false);
        assert_eq!(games[4].is_valid_for_bag(&bag), true);
        let sum = games.iter().filter(|g| g.is_valid_for_bag(&bag)).fold(0, |acc, g| acc + g.game_number);
        assert_eq!(sum, 8);
    }
}

