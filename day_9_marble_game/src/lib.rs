use std::collections::LinkedList;
use std::iter::repeat;

pub fn winning_score(players: usize, rounds: u32) -> u32 {
    let mut state = GameState::new(players);
    for _ in 0..=rounds {
        state.play_move();
    }
    *state.scores.iter().max().unwrap_or(&0)
}

#[derive(Debug)]
struct GameState {
    circle: LinkedList<u32>,
    turn: u32,
    scores: Vec<u32>,
}

impl GameState {
    fn new(players: usize) -> Self {
        Self {
            circle: LinkedList::new(),
            turn: 0,
            scores: repeat(0).take(players).collect(),
        }
    }

    fn play_move(&mut self) {
        match (self.turn == 0, self.turn % 23 == 0) {
            (true, _) => self.circle.push_front(self.turn),
            (false, true) => {
                self.move_head(7, false);
                let current_player = self.turn as usize % self.scores.len();
                self.scores[current_player] += self.circle.pop_front().unwrap_or(0) + self.turn;
            }
            (false, false) => {
                self.move_head(2, true);
                self.circle.push_front(self.turn);
            }
        }
        self.turn += 1;
    }

    fn move_head(&mut self, distance: usize, clockwise: bool) {
        let circle_size = self.circle.len();
        let split_point = if clockwise {
            distance % circle_size
        } else {
            circle_size - distance
        };
        let mut split = self.circle.split_off(split_point);
        split.append(&mut self.circle);
        self.circle = split;
    }
}

#[cfg(test)]
mod winning_score_tests {
    use winning_score;

    #[test]
    fn worked_example() {
        assert_eq!(winning_score(9, 25), 32);
    }

    #[test]
    fn example_1() {
        assert_eq!(winning_score(10, 1618), 8317);
    }

    #[test]
    fn example_2() {
        assert_eq!(winning_score(13, 7999), 146373);
    }

    #[test]
    fn example_3() {
        assert_eq!(winning_score(17, 1104), 2764);
    }

    #[test]
    fn example_4() {
        assert_eq!(winning_score(21, 6111), 54718);
    }

    #[test]
    fn example_5() {
        assert_eq!(winning_score(30, 5807), 37305);
    }

    #[test]
    fn puzzle() {
        assert_eq!(winning_score(476, 71431), 384205);
    }

    #[test]
    fn puzzle_part_2() {
        assert_eq!(winning_score(476, 71431 * 100), 3066307353);
    }

}
