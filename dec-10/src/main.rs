mod data;

fn main() {
    println!("Advent of Code: Day 10");

    // let result = solve(data::DATA);
    // println!("{}", result);
}

fn get_matching_paren(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn is_matching_paren(s: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            ')' | ']' | '}' | '>' => {
                if stack.pop() != get_matching_paren(c) {
                    return false;
                }
            }
            _ => stack.push(c),
        };
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matching_paren() {
        assert!(is_matching_paren("()"));
        assert!(is_matching_paren("([{<>}])"));
        assert!(!is_matching_paren("(]"));
    }
}
