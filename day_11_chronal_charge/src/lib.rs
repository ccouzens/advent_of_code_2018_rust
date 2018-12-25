pub fn highest_total_square(serial_number: usize, square_size: usize) -> Option<(usize, usize)> {
    FuelGrid::new(serial_number).best_square(square_size)
}

pub fn highest_variable_square(serial_number: usize) -> Option<(usize, usize, usize)> {
    let grid = FuelGrid::new(serial_number);
    (1..=300)
        .flat_map(|s| grid.best_square(s).map(|(x, y)| (x, y, s)))
        .max_by_key(|&(x, y, s)| grid.summed_square_tables.box_sum(x, y, s))
}

struct SummedSquareTable([i32; 301 * 301]);

impl SummedSquareTable {
    fn index<T>(x: T, y: T) -> usize
    where
        usize: std::convert::From<T>,
    {
        usize::from(x) * 301 + usize::from(y)
    }

    fn new(serial_number: usize) -> SummedSquareTable {
        let i = SummedSquareTable::index;
        let mut t = [0; 301 * 301];
        for x in 1..=300 {
            for y in 1..=300 {
                let cell = FuelCell::new(x, y, serial_number);
                t[i(x, y)] =
                    cell.power_level() + t[i(x, y - 1)] + t[i(x - 1, y)] - t[i(x - 1, y - 1)]
            }
        }
        SummedSquareTable(t)
    }

    fn sum_at(&self, x: usize, y: usize) -> i32 {
        self.0[SummedSquareTable::index(x, y)]
    }

    fn box_sum(&self, x: usize, y: usize, size: usize) -> i32 {
        let (x1, y1, x2, y2) = (x - 1, y - 1, x + size - 1, y + size - 1);
        self.sum_at(x2, y2) - self.sum_at(x1, y2) - self.sum_at(x2, y1) + self.sum_at(x1, y1)
    }
}

struct FuelCell {
    x: usize,
    y: usize,
    serial_number: usize,
}

struct FuelSquare<'a> {
    x: usize,
    y: usize,
    square_size: usize,
    summed_square_tables: &'a SummedSquareTable,
}

impl FuelCell {
    fn new(x: usize, y: usize, serial_number: usize) -> Self {
        FuelCell {
            x,
            y,
            serial_number,
        }
    }

    fn rack_id(&self) -> usize {
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

impl<'a> FuelSquare<'a> {
    fn square_power_level(&self) -> i32 {
        let x = self.x;
        let y = self.y;
        let s = self.square_size;
        self.summed_square_tables.box_sum(x, y, s)
    }
}

struct FuelGrid {
    summed_square_tables: SummedSquareTable,
}

impl FuelGrid {
    fn new(serial_number: usize) -> FuelGrid {
        FuelGrid {
            summed_square_tables: SummedSquareTable::new(serial_number),
        }
    }

    fn iter_squares(&self, square_size: usize) -> impl Iterator<Item = FuelSquare> {
        FuelGridSquareIterator {
            iteration: 0,
            square_size,
            summed_square_tables: &self.summed_square_tables,
        }
    }

    fn best_square(&self, square_size: usize) -> Option<(usize, usize)> {
        self.iter_squares(square_size)
            .max_by_key(|fc| fc.square_power_level())
            .map(|fc| (fc.x, fc.y))
    }
}

struct FuelGridSquareIterator<'a> {
    iteration: usize,
    square_size: usize,
    summed_square_tables: &'a SummedSquareTable,
}

impl<'a> Iterator for FuelGridSquareIterator<'a> {
    type Item = FuelSquare<'a>;
    fn next(&mut self) -> Option<FuelSquare<'a>> {
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
                square_size,
                summed_square_tables: self.summed_square_tables,
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
