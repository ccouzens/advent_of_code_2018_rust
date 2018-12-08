#[macro_use]
extern crate nom;

use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn instruction_order(input: &str) -> String {
    let mut prereqs = Prerequisite::parse_multiple(input).collect::<HashSet<_>>();
    let mut unvisited = BTreeSet::new();
    for p in prereqs.iter() {
        unvisited.insert(p.requirement);
        unvisited.insert(p.unblocks);
    }
    let mut output = String::with_capacity(unvisited.len());
    while let Some(&next_step) = unvisited
        .iter()
        .find(|&&step| !prereqs.iter().any(|prereq| prereq.unblocks == step))
    {
        output += next_step;
        prereqs.retain(|p| p.requirement != next_step);
        unvisited.remove(next_step);
    }
    output
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Prerequisite<'a> {
    requirement: &'a str,
    unblocks: &'a str,
}

impl<'a> Prerequisite<'a> {
    fn parse_multiple(input: &'a str) -> impl Iterator<Item = Prerequisite> + 'a {
        input.lines().flat_map(Self::parse).map(|(_r, p)| p)
    }

    named!(
        parse<&'a str, Self>,
        do_parse!(
            tag_s!("Step ")
                >> requirement: take_while1!(nom::AsChar::is_alpha)
                >> tag_s!(" must be finished before step ")
                >> unblocks: take_while1!(nom::AsChar::is_alpha)
                >> tag_s!(" can begin.")
                >> (Self {
                    requirement,
                    unblocks
                })
        )
    );
}

#[cfg(test)]
mod instruction_order_tests {
    use instruction_order;

    #[test]
    fn worked_example() {
        assert_eq!(
            instruction_order(include_str!("../example.txt")),
            "CABDFE".to_string()
        );
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            instruction_order(include_str!("../input.txt")),
            "ADEFKLBVJQWUXCNGORTMYSIHPZ".to_string()
        );
    }
}
