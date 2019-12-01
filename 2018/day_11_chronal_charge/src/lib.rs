pub fn highest_total_square(
    serial_number: usize,
    square_size: usize,
) -> Option<(i32, usize, usize)> {
    FuelGrid::new(serial_number).best_square_for_size(square_size)
}

pub fn highest_variable_square(serial_number: usize) -> Option<(i32, usize, usize, usize)> {
    FuelGrid::new(serial_number).best_square_overall()
}

struct SummedSquareTable([i32; 301 * 301]);

impl SummedSquareTable {
    fn index(x: usize, y: usize) -> usize {
        x * 301 + y
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

struct FuelGrid {
    summed_square_tables: SummedSquareTable,
}

impl FuelGrid {
    fn new(serial_number: usize) -> FuelGrid {
        FuelGrid {
            summed_square_tables: SummedSquareTable::new(serial_number),
        }
    }

    fn best_square_for_size(&self, square_size: usize) -> Option<(i32, usize, usize)> {
        let mut max = None;
        let mut best = None;
        let s = square_size;
        for x in 1..=301 - s {
            for y in 1..=301 - s {
                let current = self.summed_square_tables.box_sum(x, y, s);
                if max.map_or(true, |m| m < current) {
                    max = Some(current);
                    best = Some((x, y));
                }
            }
        }
        max.and_then(|m| best.map(|(x, y)| (m, x, y)))
    }

    fn best_square_overall(&self) -> Option<(i32, usize, usize, usize)> {
        (1..=300)
            .flat_map(|s| {
                self.best_square_for_size(s)
                    .map(|(power, x, y)| (power, x, y, s))
            })
            .max_by_key(|&(power, _x, _y, _s)| power)
    }
}

#[cfg(test)]
mod highest_3_by_3_total_square_tests {
    use highest_total_square;

    #[test]
    fn worked_example_1() {
        assert_eq!(highest_total_square(18, 3), Some((29, 33, 45)));
    }

    #[test]
    fn worked_example_2() {
        assert_eq!(highest_total_square(42, 3), Some((30, 21, 61)));
    }

    #[test]
    fn puzzle() {
        assert_eq!(highest_total_square(5034, 3), Some((29, 235, 63)));
    }
}

#[cfg(test)]
mod highest_variable_square_size_tests {
    use highest_variable_square;

    #[test]
    fn worked_example_1() {
        assert_eq!(highest_variable_square(18), Some((113, 90, 269, 16)));
    }
    #[test]
    fn worked_example_2() {
        assert_eq!(highest_variable_square(42), Some((119, 232, 251, 12)));
    }
    #[test]
    fn puzzle() {
        assert_eq!(highest_variable_square(5034), Some((109, 229, 251, 16)));
    }

}
