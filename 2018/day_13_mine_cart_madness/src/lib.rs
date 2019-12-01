use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Simulation {
    track: Track,
    carts: Vec<Cart>,
}

impl Simulation {
    pub fn first_crash(&mut self) -> (u8, u8) {
        loop {
            if let Some(crash) = self.move_or_crash() {
                return crash;
            }
        }
    }

    pub fn last_cart(&mut self) -> (u8, u8) {
        loop {
            self.move_or_crash();
            match (self.carts.get(0), self.carts.get(1)) {
                (Some(cart), None) => return cart.location,
                (Some(_a), Some(_b)) => {}
                _ => panic!("too many crashes"),
            }
        }
    }

    fn carts(&self) -> impl Iterator<Item = &Cart> {
        self.carts.iter()
    }

    fn move_or_crash(&mut self) -> Option<(u8, u8)> {
        self.carts.sort_by_key(|cart| {
            let (x, y) = cart.location;
            (y, x)
        });
        let mut first_crash = None;
        let mut cart_locations: HashMap<(u8, u8), (u8, u8)> =
            HashMap::from_iter(self.carts().map(|c| (c.location, c.id)));
        let mut crashed_carts = HashSet::new();
        for cart in self.carts.iter_mut() {
            if crashed_carts.contains(&cart.id) {
                continue;
            }
            cart_locations.remove(&cart.location);
            cart.update(&self.track);
            if let Some(other_cart) = cart_locations.insert(cart.location, cart.id) {
                first_crash = Some(first_crash.unwrap_or(cart.location));
                crashed_carts.insert(cart.id);
                crashed_carts.insert(other_cart);
                cart_locations.remove(&cart.location);
            }
        }
        self.carts.retain(|cart| !crashed_carts.contains(&cart.id));
        first_crash
    }
}

impl FromStr for Simulation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, t) in line.chars().enumerate() {
                let (x, y) = (x as u8, y as u8);
                use crate::Direction::*;
                let cart_direction = match t {
                    '^' => Some(North),
                    'v' => Some(South),
                    '<' => Some(West),
                    '>' => Some(East),
                    _ => None,
                };
                if let Some(d) = cart_direction {
                    carts.push(Cart::new((x, y), d, IntersectionBehaviour::Left));
                }
            }
        }

        Ok(Simulation {
            track: s.parse()?,
            carts,
        })
    }
}

#[derive(PartialEq, Debug)]
struct Cart {
    id: (u8, u8),
    location: (u8, u8),
    direction: Direction,
    intersection_behaviour: IntersectionBehaviour,
}

impl Cart {
    fn new(
        location: (u8, u8),
        direction: Direction,
        intersection_behaviour: IntersectionBehaviour,
    ) -> Self {
        Cart {
            id: location,
            location,
            direction,
            intersection_behaviour,
        }
    }

    fn update(&mut self, track: &Track) {
        use crate::Direction::{East, North, South, West};
        use crate::IntersectionBehaviour::{Left, Right, Straight};
        use crate::TrackDirection::{Intersection, PrimaryDiagonal, SecondaryDiagonal};
        self.location = match &self.direction {
            North => (self.location.0, self.location.1 - 1),
            East => (self.location.0 + 1, self.location.1),
            South => (self.location.0, self.location.1 + 1),
            West => (self.location.0 - 1, self.location.1),
        };

        self.direction = match (
            self.direction,
            track.tracks.get(&self.location),
            &self.intersection_behaviour,
        ) {
            (North, Some(PrimaryDiagonal), _) => West,
            (North, Some(SecondaryDiagonal), _) => East,
            (East, Some(PrimaryDiagonal), _) => South,
            (East, Some(SecondaryDiagonal), _) => North,
            (South, Some(PrimaryDiagonal), _) => East,
            (South, Some(SecondaryDiagonal), _) => West,
            (West, Some(PrimaryDiagonal), _) => North,
            (West, Some(SecondaryDiagonal), _) => South,

            (_, Some(Intersection), Straight) => self.direction,

            (North, Some(Intersection), Left) => West,
            (North, Some(Intersection), Right) => East,
            (East, Some(Intersection), Left) => North,
            (East, Some(Intersection), Right) => South,
            (South, Some(Intersection), Left) => East,
            (South, Some(Intersection), Right) => West,
            (West, Some(Intersection), Left) => South,
            (West, Some(Intersection), Right) => North,

            (_, None, _) => self.direction,
        };

        if track.tracks.get(&self.location) == Some(&Intersection) {
            self.intersection_behaviour = match self.intersection_behaviour {
                Left => Straight,
                Straight => Right,
                Right => Left,
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum IntersectionBehaviour {
    Left,
    Straight,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default)]
struct Track {
    tracks: HashMap<(u8, u8), TrackDirection>,
}

impl FromStr for Track {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut track = Track::default();
        for (y, line) in s.lines().enumerate() {
            for (x, t) in line.chars().enumerate() {
                let (x, y) = (x as u8, y as u8);
                use crate::TrackDirection::*;
                let track_segment = match t {
                    '\\' => Some(PrimaryDiagonal),
                    '/' => Some(SecondaryDiagonal),
                    '+' => Some(Intersection),
                    _ => None,
                };
                if let Some(segment) = track_segment {
                    track.tracks.insert((x, y), segment);
                }
            }
        }
        Ok(track)
    }
}

#[derive(PartialEq)]
enum TrackDirection {
    PrimaryDiagonal,   // \
    SecondaryDiagonal, // /
    Intersection,      // +
}

#[cfg(test)]
mod worked_example_part_1 {
    use crate::Cart;
    use crate::Direction;
    use crate::IntersectionBehaviour;
    use crate::Simulation;

    fn simulation() -> Simulation {
        include_str!("../worked_example/input.txt").parse().unwrap()
    }

    #[test]
    fn carts() {
        assert_eq!(
            simulation().carts().collect::<Vec<_>>(),
            vec!(
                &Cart::new((2, 0), Direction::East, IntersectionBehaviour::Left),
                &Cart::new((9, 3), Direction::South, IntersectionBehaviour::Left)
            )
        );
    }

    #[test]
    fn first_crash() {
        assert_eq!(simulation().first_crash(), (7, 3));
    }
}

#[cfg(test)]
mod worked_example_part_2 {
    use crate::Simulation;
    fn simulation() -> Simulation {
        include_str!("../worked_example/crash.txt").parse().unwrap()
    }

    #[test]
    fn last_cart() {
        assert_eq!(simulation().last_cart(), (6, 4));
    }
}

#[cfg(test)]
mod puzzle {
    use crate::Simulation;
    fn simulation() -> Simulation {
        include_str!("../puzzle.txt").parse().unwrap()
    }

    #[test]
    fn first_crash() {
        assert_eq!(simulation().first_crash(), (50, 54));
    }

    #[test]
    fn last_cart() {
        assert_eq!(simulation().last_cart(), (50, 100));
    }
}
