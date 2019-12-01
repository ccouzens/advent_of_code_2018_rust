#[macro_use]
extern crate nom;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::iter::repeat;
use std::iter::FromIterator;

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
        output.push(next_step as char);
        prereqs.retain(|p| p.requirement != next_step);
        unvisited.remove(&next_step);
    }
    output
}

pub fn parallelized_time(input: &str, workers: usize, a_time: u8) -> Option<usize> {
    let mut prereqs = Prerequisite::parse_multiple(input).collect::<HashSet<_>>();
    let mut workers = Vec::from_iter(repeat(None).take(workers));
    let mut unvisited = BTreeSet::new();
    let mut not_built = HashSet::new();
    let mut time = 0usize;
    for p in prereqs.iter() {
        unvisited.insert(p.requirement);
        unvisited.insert(p.unblocks);
        not_built.insert(p.requirement);
        not_built.insert(p.unblocks);
    }
    while !not_built.is_empty() {
        for (&step, worker) in unvisited
            .iter()
            .filter(|&&step| !prereqs.iter().any(|prereq| prereq.unblocks == step))
            .zip(workers.iter_mut().filter(|worker| worker.is_none()))
        {
            *worker = Some(Work {
                what: step,
                time_left: step - b'A' + a_time,
            });
        }
        for step in workers.iter().flatten().map(|w| w.what) {
            unvisited.remove(&step);
        }
        if let Some(time_jump) = workers.iter().flatten().map(|w| w.time_left).min() {
            time += time_jump as usize;
            for maybe_worker in workers.iter_mut() {
                let mut done = false;
                if let Some(worker) = maybe_worker {
                    worker.time_left -= time_jump;
                    if worker.time_left == 0 {
                        done = true;
                        prereqs.retain(|p| p.requirement != worker.what);
                        not_built.remove(&worker.what);
                    }
                }
                if done {
                    *maybe_worker = None;
                }
            }
        } else {
            return None;
        }
    }

    Some(time)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Prerequisite {
    requirement: u8,
    unblocks: u8,
}

impl Prerequisite {
    fn parse_multiple<'a>(input: &'a str) -> impl Iterator<Item = Prerequisite> + 'a {
        input
            .lines()
            .map(str::as_bytes)
            .flat_map(Self::parse)
            .map(|(_r, p)| p)
    }

    named!(
        parse<&[u8], Self>,
        do_parse!(
            tag_s!("Step ")
                >> requirement: map_opt!(take!(1), (|bs: &[u8]| bs.get(0).cloned()))
                >> tag_s!(" must be finished before step ")
                >> unblocks: map_opt!(take!(1), (|bs: &[u8]| bs.get(0).cloned()))
                >> tag_s!(" can begin.")
                >> (Self {
                    requirement,
                    unblocks
                })
        )
    );
}

#[derive(Clone, PartialEq, Debug)]
struct Work {
    what: u8,
    time_left: u8,
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

#[cfg(test)]
mod parallelized_time_tests {
    use parallelized_time;

    #[test]
    fn worked_example() {
        assert_eq!(
            parallelized_time(include_str!("../example.txt"), 2, 1),
            Some(15)
        );
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            parallelized_time(include_str!("../input.txt"), 5, 61),
            Some(1120)
        );
    }

}
