#[macro_use]
extern crate nom;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::iter::FromIterator;

type Rule = (bool, bool, bool, bool, bool);

pub struct Cave {
    pots: BTreeSet<i64>,
    rules: HashSet<Rule>,
}

impl Cave {
    pub fn try_new(input: &str) -> Result<Self, (nom::Err<&str>)> {
        let (input, pots) = parse_pots(input)?;
        let mut rules = HashSet::new();
        for line in input.lines().filter(|l| !l.is_empty()) {
            if let (_, Some(rule)) = parse_rule(line)? {
                rules.insert(rule);
            }
        }
        Ok(Cave { pots, rules })
    }

    pub fn pots(&self) -> &BTreeSet<i64> {
        &self.pots
    }

    pub fn rules(&self) -> &HashSet<Rule> {
        &self.rules
    }

    pub fn generation_sum(mut self, generations: i64) -> i64 {
        let mut pot_counts = (None, None, None);
        for i in 0..generations {
            pot_counts = (
                pot_counts.1,
                pot_counts.2,
                Some(self.pots().iter().cloned().sum::<i64>()),
            );
            if let (Some(a), Some(b), Some(c)) = pot_counts {
                if c - b == b - a {
                    return (generations - i) * (b - a) + c;
                }
            }
            self = self.step();
        }
        self.pots().iter().cloned().sum::<i64>()
    }

    pub fn step(mut self) -> Self {
        if let (Some(first), Some(last)) =
            (self.pots().iter().next(), self.pots().iter().rev().next())
        {
            let mut new_pots = BTreeSet::new();
            for pot_num in first - 2..=last + 2 {
                let t = |i| self.pots().contains(&(pot_num + i));
                let active_rule = (t(-2), t(-1), t(0), t(1), t(2));
                if self.rules().contains(&active_rule) {
                    new_pots.insert(pot_num);
                }
            }
            self.pots = new_pots;
        };
        self
    }

    pub fn fast_forward(self, generations: u64) -> Self {
        let mut cave = self;
        for _ in 0..generations {
            cave = cave.step()
        }
        cave
    }
}

named!(parse_pots < &str,BTreeSet<i64>>,
    do_parse!(
        tag_s!("initial state: ")
            >> pots: take_while!(|c| c == '#' || c== '.')
            >> (
                BTreeSet::from_iter(pots.chars().enumerate().filter_map(|(i, l)| if l == '#' {Some(i as i64)} else {None}))
            )
    )
);

named!(parse_rule< &str, Option<Rule>>,
    do_parse!(
            rule: take_while_m_n!(5, 5, |c| c == '#' || c== '.')
            >> tag_s!(" => ")
            >> result: take_while_m_n!(1, 1, |c| c == '#' || c == '.')
            >> (
                if result == "#" {
                    let mut rs = rule.chars();
                    let mut t = || rs.next() == Some('#');
                    Some((t(), t(), t(), t(), t()))
                } else {
                    None
                }
            )
    )
);

#[cfg(test)]
mod worked_example_part_1 {
    use crate::Cave;
    use std::collections::BTreeSet;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn cave() -> Cave {
        Cave::try_new(include_str!("../worked_example.txt")).unwrap()
    }

    #[test]
    fn first_generation() {
        assert_eq!(
            cave().pots(),
            &BTreeSet::from_iter([0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24].into_iter().cloned())
        );
    }

    #[test]
    fn second_generation() {
        assert_eq!(
            cave().step().pots(),
            &BTreeSet::from_iter([0, 4, 9, 15, 18, 21, 24].into_iter().cloned())
        );
    }

    #[test]
    fn rules() {
        assert_eq!(
            cave().rules(),
            &HashSet::from_iter(
                [
                    (false, false, false, true, true),
                    (false, false, true, false, false),
                    (false, true, false, false, false),
                    (false, true, false, true, false),
                    (false, true, false, true, true),
                    (false, true, true, false, false),
                    (false, true, true, true, true),
                    (true, false, true, false, true),
                    (true, false, true, true, true),
                    (true, true, false, true, false),
                    (true, true, false, true, true),
                    (true, true, true, false, false),
                    (true, true, true, false, true),
                    (true, true, true, true, false)
                ]
                .iter()
                .cloned()
            )
        );
    }

    #[test]
    fn final_sum() {
        assert_eq!(cave().generation_sum(20), 325);
    }
}

#[cfg(test)]
mod puzzle {
    use crate::Cave;

    fn cave() -> Cave {
        Cave::try_new(include_str!("../puzzle.txt")).unwrap()
    }

    #[test]
    fn twenty_generations() {
        assert_eq!(cave().generation_sum(20), 3258);
    }

    #[test]
    fn fifty_billion_generations() {
        assert_eq!(cave().generation_sum(50_000_000_000), 3600000002022);
    }
}
