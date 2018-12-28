pub fn list_of_10_scores_after_n(n: usize) -> Vec<u8> {
    let mut kitchen = Kitchen::new();
    kitchen.step_until_n_recipies(n + 10);
    kitchen.recipies.iter().skip(n).take(10).cloned().collect()
}

pub fn score_search(s: &[u8]) -> usize {
    let mut kitchen = Kitchen::new();

    let check_at = |kitchen: &Kitchen, search_point| {
        if kitchen.recipies.get(search_point..search_point + s.len()) == Some(s) {
            return Some(search_point);
        } else {
            None
        }
    };
    loop {
        if let Some(f) = check_at(&kitchen, kitchen.recipies.len().saturating_sub(s.len()))
            .or_else(|| check_at(&kitchen, kitchen.recipies.len().saturating_sub(s.len() + 1)))
        {
            return f;
        }

        kitchen.step();
    }
}

struct Kitchen {
    elf_pointers: [usize; 2],
    recipies: Vec<u8>,
}

impl Kitchen {
    fn new() -> Self {
        Kitchen {
            recipies: vec![3, 7],
            elf_pointers: [0, 1],
        }
    }

    fn step(&mut self) {
        let sum: u8 = self.elf_pointers.iter().map(|&p| self.recipies[p]).sum();

        if sum >= 10 {
            self.recipies.push(1);
        }
        self.recipies.push(sum % 10);

        for p in self.elf_pointers.iter_mut() {
            *p = (*p + 1 + self.recipies[*p] as usize) % self.recipies.len();
        }
    }

    fn step_until_n_recipies(&mut self, n: usize) {
        while self.recipies.len() < n {
            self.step();
        }
    }
}

#[cfg(test)]
mod part_1s {
    use crate::list_of_10_scores_after_n;
    #[test]
    fn worked_example_1() {
        assert_eq!(
            list_of_10_scores_after_n(9),
            vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]
        );
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(
            list_of_10_scores_after_n(5),
            vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]
        );
    }

    #[test]
    fn worked_example_3() {
        assert_eq!(
            list_of_10_scores_after_n(18),
            vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]
        );
    }

    #[test]
    fn worked_example_4() {
        assert_eq!(
            list_of_10_scores_after_n(2018),
            vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]
        );
    }

    #[test]
    fn puzzle() {
        assert_eq!(
            list_of_10_scores_after_n(360781),
            vec![6, 5, 2, 1, 5, 7, 1, 0, 1, 0]
        );
    }
}

#[cfg(test)]
mod part_2s {
    use crate::score_search;

    #[test]
    fn worked_example_1() {
        assert_eq!(score_search(&[5, 1, 5, 8, 9]), 9);
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(score_search(&[0, 1, 2, 4, 5]), 5);
    }

    #[test]
    fn worked_example_3() {
        assert_eq!(score_search(&[9, 2, 5, 1, 0]), 18);
    }

    #[test]
    fn worked_example_4() {
        assert_eq!(score_search(&[5, 9, 4, 1, 4]), 2018);
    }

    #[test]
    fn puzzle() {
        assert_eq!(score_search(&[3, 6, 0, 7, 8, 1]), 20262967);
    }
}
