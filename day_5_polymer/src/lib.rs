pub fn fully_react<T: Iterator<Item = char>>(input: T) -> String {
    let mut reacted = String::new();
    for b in input {
        let last = reacted.pop();
        match last {
            Some(a) => {
                if !letters_react(a, b) {
                    reacted.push(a);
                    reacted.push(b);
                }
            }
            None => {
                reacted.push(b);
            }
        }
    }
    reacted
}

pub fn improved_react(input: &str) -> String {
    (b'a'..=b'z')
        .map(|l| {
            fully_react(
                input
                    .chars()
                    .filter(|t| t.to_ascii_lowercase() != l as char),
            )
        }).min_by_key(|reacted| reacted.len())
        .unwrap()
}

fn letters_react(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

#[cfg(test)]
mod fully_react {
    use fully_react;

    #[test]
    fn worked_example() {
        assert_eq!(
            fully_react("dabAcCaCBAcCcaDA".chars()),
            "dabCBAcaDA".to_string()
        );
        assert_eq!(fully_react("dabAcCaCBAcCcaDA".chars()).len(), 10);
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            fully_react(include_str!("../input.txt").trim().chars()).len(),
            9386
        );
    }
}

#[cfg(test)]
mod improved_react {
    use improved_react;

    #[test]
    fn worked_example() {
        assert_eq!(improved_react("dabAcCaCBAcCcaDA"), "daDA".to_string());
        assert_eq!(improved_react("dabAcCaCBAcCcaDA").len(), 4);
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            improved_react(include_str!("../input.txt").trim()).len(),
            4876
        );
    }
}
