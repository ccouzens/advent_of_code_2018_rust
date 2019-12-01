#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;

pub fn overlap(input: &str) -> usize {
    used_cloth(input).values().filter(|&&s| s).count()
}

pub fn no_overlap(input: &str) -> Option<u16> {
    let used_cloth = used_cloth(input);
    claims(input)
        .find(|claim| {
            claim
                .cords()
                .all(|cord| used_cloth.get(&cord) == Some(&false))
        }).map(|claim| claim.id)
}

fn used_cloth(input: &str) -> HashMap<(u16, u16), bool> {
    let mut used_cloth = HashMap::with_capacity(1000 * 1000);
    for claim in claims(input) {
        for (x, y) in claim.cords() {
            used_cloth
                .entry((x, y))
                .and_modify(|s| *s = true)
                .or_insert(false);
        }
    }
    used_cloth
}

#[derive(PartialEq, Debug)]
struct Claim {
    id: u16,
    from_left: u16,
    from_top: u16,
    width: u16,
    height: u16,
}

impl<'a> Claim {
    fn cords(&'a self) -> impl Iterator<Item = (u16, u16)> + 'a {
        (self.from_left..self.from_left + self.width)
            .flat_map(move |x| (self.from_top..self.from_top + self.height).map(move |y| (x, y)))
    }
}

named!(claim<&str,Claim>,
    do_parse!(
        tag_s!("#") >>
        id: map_res!(nom::digit, FromStr::from_str) >>
        ws!(tag_s!("@")) >>
        from_left: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!(",") >>
        from_top: map_res!(nom::digit, FromStr::from_str) >>
        ws!(tag_s!(":")) >>
        width: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!("x") >>
        height: map_res!(nom::digit, FromStr::from_str) >>

        (Claim{ id, from_left, from_top, width, height})
    )
);

struct ClaimsIterator<'a> {
    input: &'a str,
}

impl<'a> Iterator for ClaimsIterator<'a> {
    type Item = Claim;
    fn next(&mut self) -> Option<Self::Item> {
        match claim(self.input.trim_start()) {
            Ok((remaining, claim)) => {
                self.input = remaining;
                Some(claim)
            }
            Err(_) => None,
        }
    }
}

fn claims<'a>(input: &'a str) -> impl Iterator<Item = Claim> + 'a {
    ClaimsIterator { input }
}

#[cfg(test)]
mod overlap_tests {
    use overlap;
    #[test]
    fn worked_example() {
        let input = r#"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 4x4
            #3 @ 5,5: 2x2
        "#;
        assert_eq!(overlap(input), 4);
    }

    #[test]
    fn puzzle() {
        assert_eq!(overlap(include_str!("../input.txt")), 100595);
    }

}

#[cfg(test)]
mod no_overlap_tests {
    use no_overlap;
    #[test]
    fn worked_example() {
        let input = r#"
            #1 @ 1,3: 4x4
            #2 @ 3,1: 4x4
            #3 @ 5,5: 2x2
        "#;
        assert_eq!(no_overlap(input), Some(3));
    }

    #[test]
    fn puzzle() {
        assert_eq!(no_overlap(include_str!("../input.txt")), Some(415));
    }
}
