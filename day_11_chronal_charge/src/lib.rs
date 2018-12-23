pub fn highest_total_square(serial_number: u32) -> Option<(u32, u32)> {
    FuelGrid { serial_number }.best_square()
}

struct FuelCell {
    x: u32,
    y: u32,
    serial_number: u32,
}

impl FuelCell {
    fn rack_id(&self) -> u32 {
        self.x + 10
    }

    fn power_level(&self) -> i8 {
        let step_1 = self.rack_id();
        let step_2 = step_1 * self.y;
        let step_3 = step_2 + self.serial_number;
        let step_4 = step_3 * self.rack_id();
        let step_5 = (step_4 / 100) % 10;
        step_5 as i8 - 5
    }

    fn square_power_level(&self) -> i8 {
        self.square_iters().map(|n| n.power_level()).sum()
    }

    fn square_iters<'a>(&'a self) -> impl Iterator<Item = FuelCell> + 'a {
        let serial_number = self.serial_number;
        [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]
            .iter()
            .map(move |&(x, y)| FuelCell {
                x: x + self.x,
                y: y + self.y,
                serial_number,
            })
    }
}

struct FuelGrid {
    serial_number: u32,
}

impl FuelGrid {
    fn iter_squares(&self) -> impl Iterator<Item = FuelCell> {
        FuelGridSquareIterator {
            serial_number: self.serial_number,
            x: 1,
            y: 1,
        }
    }

    fn best_square(&self) -> Option<(u32, u32)> {
        self.iter_squares()
            .max_by_key(|fc| fc.square_power_level())
            .map(|fc| (fc.x, fc.y))
    }
}

struct FuelGridSquareIterator {
    serial_number: u32,
    x: u32,
    y: u32,
}

impl Iterator for FuelGridSquareIterator {
    type Item = FuelCell;
    fn next(&mut self) -> Option<FuelCell> {
        let serial_number = self.serial_number;
        match (self.x, self.y) {
            (298, 298) => None,
            (298, y) => {
                self.x = 1;
                self.y += 1;
                Some(FuelCell {
                    x: 298,
                    y,
                    serial_number,
                })
            }
            (x, y) => {
                self.x += 1;
                Some(FuelCell {
                    x,
                    y,
                    serial_number,
                })
            }
        }
    }
}

#[cfg(test)]
mod highest_total_square_tests {
    use highest_total_square;

    #[test]
    fn worked_example_1() {
        assert_eq!(highest_total_square(18), Some((33, 45)));
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(highest_total_square(42), Some((21, 61)));
    }

    #[test]
    fn puzzle() {
        assert_eq!(highest_total_square(5034), Some((235, 63)));
    }

}
