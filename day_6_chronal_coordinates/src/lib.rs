use std::collections::HashSet;

pub fn largest_finite(input: &str) -> usize {
    let coordinates = parse_coordinates(input);

    coordinates
        .iter()
        .filter(|c| !c.is_infinite(coordinates.iter()))
        .map(|c| c.area(&coordinates))
        .max()
        .unwrap_or(0)
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    x: i16,
    y: i16,
}

impl Coordinate {
    fn in_cone_up(&self, other: &Coordinate) -> bool {
        self.y > other.y && (self.x - other.x).abs() <= self.y - other.y
    }

    fn in_cone_right(&self, other: &Coordinate) -> bool {
        self.x < other.x && (self.y - other.y).abs() <= other.x - self.x
    }

    fn in_cone_down(&self, other: &Coordinate) -> bool {
        self.y < other.y && (self.x - other.x).abs() <= other.y - self.y
    }

    fn in_cone_left(&self, other: &Coordinate) -> bool {
        self.x > other.x && (self.y - other.y).abs() <= self.x - other.x
    }

    fn is_infinite<'a, T: Iterator<Item = &'a Coordinate>>(&self, all: T) -> bool {
        let mut infinite_up = true;
        let mut infinite_right = true;
        let mut infinite_down = true;
        let mut infinite_left = true;
        for other in all.filter(|&o| o != self) {
            infinite_up = infinite_up && !self.in_cone_up(&other);
            infinite_right = infinite_right && !self.in_cone_right(&other);
            infinite_down = infinite_down && !self.in_cone_down(&other);
            infinite_left = infinite_left && !self.in_cone_left(&other);
            if !infinite_up && !infinite_right && !infinite_down && !infinite_left {
                return false;
            }
        }
        true
    }

    fn coordinates_at_distance(&self, distance: i16) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();
        for x in self.x - distance..=self.x + distance {
            let x_distance = (x - self.x).abs();
            let y_distance = distance - x_distance;
            coordinates.push(Coordinate {
                x,
                y: self.y - y_distance,
            });

            if y_distance != 0 {
                coordinates.push(Coordinate {
                    x,
                    y: self.y + y_distance,
                });
            }
        }
        coordinates
    }

    fn distance_between(&self, other: &Coordinate) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn area(&self, all_coordinates: &HashSet<Coordinate>) -> usize {
        let mut area = 0;
        let mut search_distance = 0;
        let mut found = true;
        while found {
            found = false;
            for s in self.coordinates_at_distance(search_distance).iter() {
                if all_coordinates
                    .iter()
                    .filter(|&c| c != self)
                    .all(|c| c.distance_between(s) > search_distance)
                {
                    area += 1;
                    found = true
                }
            }
            search_distance += 1;
        }
        area
    }
}

fn parse_coordinates(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .flat_map(|line| {
            let mut parts = line.split(", ");
            let first = parts.next();
            let second = parts.next();
            match (
                first.and_then(|v| v.parse().ok()),
                second.and_then(|v| v.parse().ok()),
            ) {
                (Some(x), Some(y)) => Some(Coordinate { x, y }),
                _ => None,
            }
        }).collect()
}

#[cfg(test)]
mod largest_finite {
    use largest_finite;
    #[test]
    fn worked_example() {
        assert_eq!(largest_finite(include_str!("../example.txt")), 17);
    }

    #[test]
    fn puzzle() {
        assert_eq!(largest_finite(include_str!("../input.txt")), 5365);
    }

}
