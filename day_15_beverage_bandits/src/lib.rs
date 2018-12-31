extern crate image;
use core::cmp::max;
use std::str::FromStr;

enum Fighter {
    Goblin,
    Elf,
}

#[derive(Default)]
struct Battle {
    dimensions: (u32, u32),
    caverns: std::collections::HashSet<(u32, u32)>,
    fighters: std::collections::HashMap<(u32, u32), (u8, Fighter)>,
}

impl FromStr for Battle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut battle = Battle::default();
        for (line, line_number) in s.lines().filter(|&l| !l.is_empty()).zip(0..) {
            for (c, col_number) in line.chars().zip(0..) {
                battle.dimensions = (
                    max(battle.dimensions.0, col_number),
                    max(battle.dimensions.1, line_number),
                );
                if c != '#' {
                    battle.caverns.insert((col_number, line_number));
                }
                if c == 'G' {
                    battle
                        .fighters
                        .insert((col_number, line_number), (200, Fighter::Goblin));
                } else if c == 'E' {
                    battle
                        .fighters
                        .insert((col_number, line_number), (200, Fighter::Elf));
                }
            }
        }
        Ok(battle)
    }
}

impl Battle {
    fn to_image(&self) -> image::RgbImage {
        image::ImageBuffer::from_fn(self.dimensions.0 + 1, self.dimensions.1 + 1, |x, y| match (
            self.fighters.get(&(x, y)),
            self.caverns.contains(&(x, y)),
        ) {
            (Some((health, Fighter::Elf)), _) => image::Rgb([0u8, *health, 0u8]),
            (Some((health, Fighter::Goblin)), _) => image::Rgb([0u8, 0u8, *health]),
            (None, true) => image::Rgb([255u8; 3]),
            (None, false) => image::Rgb([0u8; 3]),
        })
    }
}

#[cfg(test)]
mod worked_example_1 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_1.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_produces_the_correct_initial_image() {
        assert!(
            battle().to_image().into_vec()
                == image::load_from_memory(include_bytes!("../worked_examples/example_1.png"))
                    .unwrap()
                    .raw_pixels()
        );
    }
}
