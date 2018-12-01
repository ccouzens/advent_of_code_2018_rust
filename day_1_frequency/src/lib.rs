pub fn frequency(input: &'static str) -> i32 {
    let mut sum = 0;
    let mut sign = true;
    let mut value = 0;
    for c in input.chars() {
        match (c, c.to_digit(10)) {
            ('+', None) => {
                sign = true;
                value = 0;
            }
            ('-', None) => {
                sign = false;
                value = 0;
            }
            (_, Some(digit)) => {
                value = value * 10 + digit as i32;

                sum += if sign {
                    value - value / 10
                } else {
                    value / 10 - value
                };
            }
            _ => {}
        }
    }
    sum
}

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
