use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate {
    col: i32,
    row: i32,
}

impl Coordinate {
    fn new(col: i32, row: i32) -> Self {
        Self { col: col, row: row }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum EnginePiece {
    Symbol(char),
    Number(i32),
    PartNumber(String),
    Empty,
}

fn main() {
    let input = read_input("input");
    // let input = read_input("sample_input");

    let result = process(&input);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn process(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let pieces = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let mut skip_col_values = vec![];
            line.chars().enumerate().map(move |(col, c)| {
                if skip_col_values.contains(&col) {
                    return (
                        Coordinate::new(col as i32, row as i32),
                        EnginePiece::Number(c.to_digit(10).unwrap() as i32),
                    );
                }
                return (
                    Coordinate::new(col as i32, row as i32),
                    match c {
                        '.' => EnginePiece::Empty,
                        c if c.is_ascii_digit() => {
                            // Found a Part Number so lets find the rest of it
                            let line = &line[col..];
                            let part_number = line
                                .chars()
                                .take_while(|&c| c.is_ascii_digit())
                                .collect::<String>();
                            // Now we need to skip col + part_number.len() chars
                            (col..col + part_number.len()).for_each(|c| {
                                skip_col_values.push(c);
                            });
                            EnginePiece::PartNumber(part_number)
                        }
                        c => EnginePiece::Symbol(c),
                    },
                );
            })
        })
        .collect::<BTreeMap<Coordinate, EnginePiece>>();

    let positions = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    // Combine sequencial numbers inso a vector
    let mut valid_part_numbers: Vec<i32> = vec![];
    for (coordinate, piece) in &pieces {
        println!("{:?} {:?}", coordinate, piece);

        if let EnginePiece::PartNumber(part_num) = piece {
            // Check all the coordinates around the Part Number for a Symbol
            'part_num_loop: for (i, _) in part_num.chars().enumerate() {
                for (row_offset, col_offset) in positions.iter() {
                    let check_coordinate = Coordinate::new(
                        coordinate.col + col_offset + (i as i32),
                        coordinate.row + row_offset,
                    );
                    if let Some(EnginePiece::Symbol(_)) = pieces.get(&check_coordinate) {
                        valid_part_numbers.push(part_num.parse::<i32>().expect("Invalid part number string"));
                        break 'part_num_loop;
                    }
                }
            }
        }
    }

    println!("{:?}", valid_part_numbers);

    // Sum of all the valid part numbers
    let sum = valid_part_numbers.iter().sum::<i32>();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
    }
}
