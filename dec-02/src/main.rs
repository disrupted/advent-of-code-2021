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
    let amount = str_to_int(split[1]).unwrap();
    match split[0] {
        "forward" => submarine.x += amount,
        "down" => submarine.y += amount,
        "up" => submarine.y -= amount,
        _ => eprintln!("didn't recognize command: {}", command),
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

    #[test]
    fn test_move_down() {
        let pos = Position { x: 0, y: 0 };

        let pos = dive(pos, "down 1");

        assert!(pos.x == 0);
        assert!(pos.y == 1);

        let pos = dive(pos, "down 10");

        assert!(pos.x == 0);
        assert!(pos.y == 11);
    }

    #[test]
    fn test_move_up() {
        let pos = Position { x: 50, y: 100 };

        let pos = dive(pos, "up 1");

        assert!(pos.x == 50);
        assert!(pos.y == 99);

        let pos = dive(pos, "up 99");

        assert!(pos.x == 50);
        assert!(pos.y == 0);
    }
}
