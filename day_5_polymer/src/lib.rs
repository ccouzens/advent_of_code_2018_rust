pub fn fully_react(input: &str) -> String {
    let mut length = input.len();
    let mut partial: String = ReactIterator::new(input).collect();
    while length != partial.len() {
        length = partial.len();
        partial = ReactIterator::new(&partial).collect();
    }
    partial
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

struct ReactIterator<'a> {
    input: std::str::Chars<'a>,
    last_character: Option<char>,
}

impl<'a> ReactIterator<'a> {
    fn new(input: &'a str) -> ReactIterator {
        ReactIterator {
            input: input.chars(),
            last_character: None,
        }
    }
}

fn letters_react(a: char, b: char) -> bool {
    a.is_ascii_uppercase() && b.is_ascii_lowercase() && a.to_ascii_lowercase() == b
        || a.is_ascii_lowercase() && b.is_ascii_uppercase() && a.to_ascii_uppercase() == b
}

impl<'a> Iterator for ReactIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            let current = self.input.next();
            match (self.last_character, current) {
                (Some(a), Some(b)) => {
                    if letters_react(a, b) {
                        self.last_character = None;
                    } else {
                        self.last_character = Some(b);
                        return Some(a);
                    }
                }
                (Some(a), None) => {
                    self.last_character = None;
                    return Some(a);
                }
                (None, Some(b)) => {
                    self.last_character = Some(b);
                }
                (None, None) => return None,
            }
        }
    }
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
