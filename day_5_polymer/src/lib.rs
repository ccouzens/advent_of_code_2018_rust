pub fn fully_react(input: &str) -> String {
    let mut partial: String = ReactIterator::new(input).collect();
    let mut length = None;
    while length != Some(partial.len()) {
        length = Some(partial.len());
        partial = ReactIterator::new(&partial).collect();
    }
    partial
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
        let current = self.input.next();
        match (self.last_character, current) {
            (Some(a), Some(b)) => {
                if letters_react(a, b) {
                    self.last_character = None;
                    self.next()
                } else {
                    self.last_character = Some(b);
                    Some(a)
                }
            }
            (Some(a), None) => {
                self.last_character = None;
                Some(a)
            }
            (None, Some(b)) => {
                self.last_character = Some(b);
                self.next()
            }
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod fully_react {
    use fully_react;

    #[test]
    fn worked_example() {
        assert_eq!(fully_react("dabAcCaCBAcCcaDA").len(), 10);
    }

    #[test]
    fn puzzle() {
        assert_eq!(fully_react(include_str!("../input.txt").trim()).len(), 9386);
    }
}
