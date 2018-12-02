use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn checksum(list: &str) -> u32 {
    let (doubles, triples) = list.split_whitespace().map(id_analysis).fold(
        (0, 0),
        |(doubles, triples), (double, triple)| {
            (
                doubles + if double { 1 } else { 0 },
                triples + if triple { 1 } else { 0 },
            )
        },
    );
    doubles * triples
}

pub fn common_letters(list: &str) -> Option<String> {
    let list_entries: Vec<&str> = list.split_whitespace().collect();
    for (index, entry_a) in list_entries.iter().enumerate() {
        for entry_b in list_entries.iter().skip(index + 1) {
            if hamming_distance_of_one(entry_a, entry_b) {
                return Some(
                    entry_a
                        .chars()
                        .zip(entry_b.chars())
                        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                        .collect(),
                );
            }
        }
    }
    None
}

fn id_analysis(id: &str) -> (bool, bool) {
    let mut letter_counts = HashMap::new();
    for letter in id.chars() {
        *letter_counts.entry(letter).or_default() += 1;
    }
    let counts: HashSet<u8> = HashSet::from_iter(letter_counts.drain().map(|kv| kv.1));
    (counts.contains(&2), counts.contains(&3))
}

fn hamming_distance_of_one(a: &str, b: &str) -> bool {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a != b)
        .take(2)
        .count()
        == 1
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

#[cfg(test)]
mod common_letters {
    use common_letters;

    #[test]
    fn worked_example() {
        let list = "abcde fghij klmno pqrst fguij axcye wvxyz";
        assert_eq!(common_letters(list), Some("fgij".to_string()));
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            common_letters(include_str!("../input.txt")),
            Some("cnjxoritzhvbosyewrmqhgkul".to_string())
        );
    }
}
