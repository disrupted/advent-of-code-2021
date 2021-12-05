fn main() {
    println!("Advent of Code: Day 5");
}

fn create_diagram(input: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ";
        let diagram = ".......1..
            ..1....1..
            ..1....1..
            .......1..
            .112111211
            ..........
            ..........
            ..........
            ..........
            222111....";
        let output = create_diagram(input);

        assert!(output == diagram);
    }
}
