pub fn frequency(input: &'static str) -> i32 {
    numbers_iterator(input).sum()
}

pub fn repeated_frequency(input: &'static str) -> i32 {
    let mut seen = std::collections::HashSet::new();
    let mut cumulative_sum = 0;
    for num in numbers_iterator(input).cycle() {
        if !seen.insert(cumulative_sum) {
            return cumulative_sum;
        }
        cumulative_sum += num;
    }
    panic!("should not reach");
}

fn numbers_iterator(input: &'static str) -> NumberIterator {
    NumberIterator {
        chars: input.chars().peekable(),
    }
}

#[derive(Clone)]
struct NumberIterator<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = None;
        let mut sign = None;
        while let Some(c) = self.chars.by_ref().next() {
            match (c, c.to_digit(10), sign) {
                ('+', None, None) => {
                    value = Some(0);
                    sign = Some(true);
                }
                ('-', None, None) => {
                    value = Some(0);
                    sign = Some(false);
                }
                (_, Some(digit), Some(true)) => {
                    value = value.map(|v| v * 10 + digit as i32);
                }
                (_, Some(digit), Some(false)) => {
                    value = value.map(|v| v * 10 - digit as i32);
                }
                _ => {}
            };
            if self.chars.peek() == Some(&'+') || self.chars.peek() == Some(&'-') {
                return value;
            }
        }
        value
    }
}

#[cfg(test)]
mod frequency {
    use frequency;
    #[test]
    fn worked_example() {
        assert_eq!(frequency("+1, -2, +3, +1"), 3);
    }

    #[test]
    fn second_example() {
        assert_eq!(frequency("+1, +1, +1"), 3);
    }
    #[test]
    fn thrid_example() {
        assert_eq!(frequency("+1, +1, -2"), 0);
    }

    #[test]
    fn fourth_example() {
        assert_eq!(frequency("-1, -2, -3"), -6);
    }

    #[test]
    fn puzzle() {
        assert_eq!(frequency(include_str!("../input.txt")), 493);
    }
}

#[cfg(test)]
mod repeated_frequency {
    use repeated_frequency;
    #[test]
    fn worked_example() {
        assert_eq!(repeated_frequency("+1, -2, +3, +1"), 2);
    }

    #[test]
    fn second_example() {
        assert_eq!(repeated_frequency("+1, -1"), 0);
    }

    #[test]
    fn third_example() {
        assert_eq!(repeated_frequency("+3, +3, +4, -2, -4"), 10);
    }

    #[test]
    fn fourth_example() {
        assert_eq!(repeated_frequency("-6, +3, +8, +5, -6"), 5);
    }

    #[test]
    fn fifth_example() {
        assert_eq!(repeated_frequency("+7, +7, -2, -7, -4"), 14);
    }

    #[test]
    fn puzzle() {
        assert_eq!(repeated_frequency(include_str!("../input.txt")), 413);
    }
}
