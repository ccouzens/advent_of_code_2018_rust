pub fn highest_total_square(serial_number: u32, square_size: u32) -> Option<(u32, u32)> {
    FuelGrid { serial_number }.best_square(square_size)
}

pub fn highest_variable_square(serial_number: u32) -> Option<(u32, u32, u32)> {
    (1..=300)
        .flat_map(|s| highest_total_square(serial_number, s).map(|(x, y)| (x, y, s)))
        .max_by_key(|&(x, y, s)| {
            FuelSquare {
                x,
                y,
                serial_number,
                square_size: s,
            }
            .square_power_level()
        })
}

struct FuelCell {
    x: u32,
    y: u32,
    serial_number: u32,
}

struct FuelSquare {
    x: u32,
    y: u32,
    serial_number: u32,
    square_size: u32,
}

impl FuelCell {
    fn rack_id(&self) -> u32 {
        self.x + 10
    }

    fn power_level(&self) -> i32 {
        let step_1 = self.rack_id();
        let step_2 = step_1 * self.y;
        let step_3 = step_2 + self.serial_number;
        let step_4 = step_3 * self.rack_id();
        let step_5 = (step_4 / 100) % 10;
        step_5 as i32 - 5
    }
}

impl FuelSquare {
    fn square_power_level(&self) -> i32 {
        self.iter().map(|n| n.power_level()).sum()
    }

    fn iter(&self) -> impl Iterator<Item = FuelCell> {
        FuelSquareCellIterator {
            serial_number: self.serial_number,
            x: self.x,
            y: self.y,
            square_size: self.square_size,
            iteration: 0,
        }
    }
}

struct FuelSquareCellIterator {
    serial_number: u32,
    x: u32,
    y: u32,
    square_size: u32,
    iteration: u32,
}

impl Iterator for FuelSquareCellIterator {
    type Item = FuelCell;
    fn next(&mut self) -> Option<FuelCell> {
        if self.square_size * self.square_size == self.iteration {
            return None;
        }
        let iteration = self.iteration;
        self.iteration += 1;
        Some(FuelCell {
            x: self.x + iteration / self.square_size,
            y: self.y + iteration % self.square_size,
            serial_number: self.serial_number,
        })
    }
}

struct FuelGrid {
    serial_number: u32,
}

impl FuelGrid {
    fn iter_squares(&self, square_size: u32) -> impl Iterator<Item = FuelSquare> {
        FuelGridSquareIterator {
            serial_number: self.serial_number,
            iteration: 0,
            square_size,
        }
    }

    fn best_square(&self, square_size: u32) -> Option<(u32, u32)> {
        self.iter_squares(square_size)
            .max_by_key(|fc| fc.square_power_level())
            .map(|fc| (fc.x, fc.y))
    }
}

struct FuelGridSquareIterator {
    serial_number: u32,
    iteration: u32,
    square_size: u32,
}

impl Iterator for FuelGridSquareIterator {
    type Item = FuelSquare;
    fn next(&mut self) -> Option<FuelSquare> {
        let serial_number = self.serial_number;
        let square_size = self.square_size;
        let limit = 301 - square_size;
        let x = 1 + self.iteration % limit;
        let y = 1 + self.iteration / limit;
        self.iteration += 1;
        if y > limit {
            None
        } else {
            Some(FuelSquare {
                x,
                y,
                serial_number,
                square_size,
            })
        }
    }
}

#[cfg(test)]
mod highest_3_by_3_total_square_tests {
    use highest_total_square;

    #[test]
    fn worked_example_1() {
        assert_eq!(highest_total_square(18, 3), Some((33, 45)));
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(highest_total_square(42, 3), Some((21, 61)));
    }

    #[test]
    fn puzzle() {
        assert_eq!(highest_total_square(5034, 3), Some((235, 63)));
    }
}

#[cfg(test)]
mod highest_variable_square_size_tests {
    use highest_variable_square;

    #[test]
    fn worked_example_1() {
        assert_eq!(highest_variable_square(18), Some((90, 269, 16)));
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(highest_variable_square(42), Some((232, 251, 12)));
    }

    #[test]
    fn puzzle() {
        assert_eq!(highest_variable_square(5034), Some((229, 251, 16)));
    }

}
