fn main() {
    println!("Advent of Code: Day 2");
    let submarine = Position { x: 0, y: 0 };
}

struct Position {
    x: u8,
    y: u8,
}

fn str_to_int(s: &str) -> Result<u8, std::num::ParseIntError> {
    s.trim().parse()
}

fn dive(mut submarine: Position, command: &str) -> Position {
    let split = command.split_whitespace().collect::<Vec<&str>>();
    match split[0] {
        "forward" => {
            let amount = str_to_int(split[1]).unwrap();
            submarine.x += amount;
        }
        _ => println!("didn't recognize command: {}", command),
    };
    submarine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_forward() {
        let pos = Position { x: 0, y: 0 };

        let pos = dive(pos, "forward 1");

        assert!(pos.x == 1);
        assert!(pos.y == 0);

        let pos = dive(pos, "forward 5");

        assert!(pos.x == 6);
        assert!(pos.y == 0);
    }
}
