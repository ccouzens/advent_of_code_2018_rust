pub fn fully_react(input: &str) -> String {
    let mut reacted = String::with_capacity(input.len());
    for b in input.chars() {
        let last = reacted.pop();
        match (last, b) {
            (Some(a), b) => {
                if !letters_react(a, b) {
                    reacted.push(a);
                    reacted.push(b);
                }
            }
            (None, b) => {
                reacted.push(b);
            }
        }
    }
    reacted
}

pub fn improved_react(input: &str) -> Option<String> {
    (b'a'..=b'z')
        .map(|l| {
            input
                .chars()
                .filter(|t| t.to_ascii_lowercase() != l as char)
                .collect::<String>()
        }).map(|filtered| fully_react(&filtered))
        .min_by_key(|reacted| reacted.len())
}

fn letters_react(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

#[cfg(test)]
mod fully_react {
    use fully_react;

    #[test]
    fn worked_example() {
        assert_eq!(fully_react("dabAcCaCBAcCcaDA"), "dabCBAcaDA".to_string());
        assert_eq!(fully_react("dabAcCaCBAcCcaDA").len(), 10);
    }

    #[test]
    fn puzzle() {
        assert_eq!(fully_react(include_str!("../input.txt").trim()).len(), 9386);
    }
}

#[cfg(test)]
mod improved_react {
    use improved_react;

    #[test]
    fn worked_example() {
        assert_eq!(improved_react("dabAcCaCBAcCcaDA"), Some("daDA".to_string()));
        assert_eq!(improved_react("dabAcCaCBAcCcaDA").map(|r| r.len()), Some(4));
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            improved_react(include_str!("../input.txt").trim()).map(|r| r.len()),
            Some(4876)
        );
    }
}
