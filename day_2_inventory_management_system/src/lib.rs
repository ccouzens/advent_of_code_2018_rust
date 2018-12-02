use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn checksum(list: &str) -> u32 {
    let (doubles, triples) = list.split_whitespace().map(id_analysis).fold(
        (0, 0),
        |(mut doubles, mut triples), (double, triple)| {
            if double {
                doubles += 1
            };
            if triple {
                triples += 1
            };
            (doubles, triples)
        },
    );
    doubles * triples
}

fn id_analysis(id: &str) -> (bool, bool) {
    let mut letter_counts: HashMap<char, u8> = HashMap::new();
    for letter in id.chars() {
        *letter_counts.entry(letter).or_default() += 1;
    }
    let counts: HashSet<u8> = HashSet::from_iter(letter_counts.values().cloned());
    (counts.contains(&2), counts.contains(&3))
}

#[cfg(test)]
mod checksum {
    use checksum;
    #[test]
    fn worked_example() {
        let list = "abcdef bababc abbcde abcccd aabcdd abcdee ababab";
        assert_eq!(checksum(list), 12);
    }

    #[test]
    fn puzzle() {
        assert_eq!(checksum(include_str!("../input.txt")), 7410);
    }
}
