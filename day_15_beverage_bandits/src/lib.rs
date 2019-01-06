extern crate image;
use core::cmp::max;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum FighterType {
    Goblin,
    Elf,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn ordered_neighbours(self) -> [Self; 4] {
        let (x, y) = (self.x, self.y);
        [
            Coordinate { x, y: y - 1 },
            Coordinate { x: x - 1, y },
            Coordinate { x: x + 1, y },
            Coordinate { x, y: y + 1 },
        ]
    }

    fn is_neighbour(self, other: Self) -> bool {
        match (self.x == other.x, self.y == other.y) {
            (true, true) => false,
            (false, false) => false,
            (true, false) => self.y + 1 == other.y || self.y == other.y + 1,
            (false, true) => self.x + 1 == other.x || self.x == other.x + 1,
        }
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FighterType {
    fn from_char(s: char) -> Option<FighterType> {
        match s {
            'G' => Some(FighterType::Goblin),
            'E' => Some(FighterType::Elf),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct FighterId(u32, u32);

#[derive(Clone)]
struct Fighter {
    location: Coordinate,
    health: u8,
    attack_power: u8,
    id: FighterId,
    fighter_type: FighterType,
}

impl Fighter {
    fn to_pixel(&self) -> image::Rgb<u8> {
        match self.fighter_type {
            FighterType::Goblin => image::Rgb([0u8, 0u8, self.health]),
            FighterType::Elf => image::Rgb([0u8, self.health, 0u8]),
        }
    }

    fn new(location: Coordinate, id: FighterId, fighter_type: FighterType) -> Self {
        Fighter {
            location,
            health: 200,
            attack_power: 3,
            id,
            fighter_type,
        }
    }

    fn has_targets(battle: &Battle, id: FighterId) -> Option<bool> {
        if let Some(fighter) = battle.fighters.get(&id) {
            let ft = fighter.fighter_type;
            Some(battle.fighters.values().any({ |f| f.fighter_type != ft }))
        } else {
            None
        }
    }

    fn move_fighter(battle: &mut Battle, id: FighterId) {
        if let Some(fighter) = battle.fighters.get(&id) {
            let view = BattleMovement::new(&battle, fighter);
            if let Some(fighter) = battle.fighters.get_mut(&id) {
                fighter.location = view.new_location();
            }
        }
    }

    fn attack(battle: &mut Battle, id: FighterId) {
        if let Some(fighter) = battle.fighters.get(&id) {
            let fighter_type = fighter.fighter_type;
            let location = fighter.location;
            let attack_power = fighter.attack_power;
            let mut enemies: HashMap<Coordinate, &mut Fighter> = HashMap::from_iter(
                battle
                    .fighters
                    .values_mut()
                    .filter(|f| f.fighter_type != fighter_type)
                    .filter(|f| f.location.is_neighbour(location))
                    .map(|f| (f.location, f)),
            );
            if let Some(weakest_health) = enemies.values().map(|f| f.health).min() {
                for c in location.ordered_neighbours().iter() {
                    if let Some(enemy) = enemies.get_mut(c) {
                        if enemy.health != weakest_health {
                            continue;
                        }
                        if enemy.health > attack_power {
                            enemy.health -= attack_power;
                        } else {
                            let id = enemy.id;
                            battle.fighters.remove(&id);
                        }
                        return;
                    }
                }
            }
        }
    }
}

type Caverns = std::collections::HashSet<Coordinate>;

#[derive(Default, Clone)]
pub struct Battle {
    round_number: u32,
    dimensions: Coordinate,
    caverns: Caverns,
    fighters: HashMap<FighterId, Fighter>,
}

pub struct BattleMovement {
    open_tiles: HashSet<Coordinate>,
    enemies: HashSet<Coordinate>,
    position: Coordinate,
}

impl BattleMovement {
    fn new(battle: &Battle, current_fighter: &Fighter) -> Self {
        let fighter_locations: HashSet<Coordinate> =
            battle.fighters.values().map(|f| f.location).collect();
        BattleMovement {
            open_tiles: HashSet::from_iter(
                battle
                    .caverns
                    .iter()
                    .filter(|c| !fighter_locations.contains(c))
                    .cloned(),
            ),
            enemies: battle
                .fighters
                .values()
                .filter_map(|f| {
                    if f.fighter_type == current_fighter.fighter_type {
                        None
                    } else {
                        Some(f.location)
                    }
                })
                .collect(),
            position: current_fighter.location,
        }
    }

    fn new_location(&self) -> Coordinate {
        let immediate_moves = self.position.ordered_neighbours();
        if immediate_moves
            .iter()
            .any({ |c| self.enemies.contains(&c) })
        {
            return self.position;
        }

        let targets: BTreeSet<Coordinate> = self
            .enemies
            .iter()
            .flat_map(|&c| {
                let neighbours = c.ordered_neighbours();
                neighbours
                    .into_iter()
                    .filter(|&n| self.open_tiles.contains(n))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut unvisited_set = self.open_tiles.clone();
        let mut current_nodes_ordered = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];
        for (working_nodes, &c) in current_nodes_ordered.iter_mut().zip(immediate_moves.iter()) {
            if unvisited_set.remove(&c) {
                working_nodes.insert(c);
            }
        }
        let mut expanded = true;
        while expanded {
            expanded = false;
            for target in targets.iter() {
                for (working_nodes, &initial_move) in
                    current_nodes_ordered.iter().zip(immediate_moves.iter())
                {
                    if working_nodes.contains(target) {
                        return initial_move;
                    }
                }
            }

            for working_nodes in current_nodes_ordered.iter_mut() {
                let mut new_working_nodes = HashSet::new();
                for working_node in working_nodes.iter() {
                    for n in working_node.ordered_neighbours().iter().cloned() {
                        if unvisited_set.remove(&n) {
                            new_working_nodes.insert(n);
                            expanded = true
                        }
                    }
                }
                *working_nodes = new_working_nodes;
            }
        }
        self.position
    }
}

impl FromStr for Battle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut battle = Battle::default();
        for (line, line_number) in s.lines().filter(|&l| !l.is_empty()).zip(0..) {
            battle.dimensions.y = max(battle.dimensions.y, line_number);
            for (c, col_number) in line.chars().zip(0..) {
                let coord = Coordinate {
                    x: col_number,
                    y: line_number,
                };
                battle.dimensions.x = max(battle.dimensions.x, coord.x);
                if c != '#' {
                    battle.caverns.insert(coord);
                }
                if let Some(fighter_type) = FighterType::from_char(c) {
                    let id = FighterId(col_number, line_number);
                    battle
                        .fighters
                        .insert(id, Fighter::new(coord, id, fighter_type));
                }
            }
        }
        Ok(battle)
    }
}

impl Battle {
    pub fn to_image(&self) -> image::RgbImage {
        let fighters: HashMap<Coordinate, &Fighter> =
            HashMap::from_iter(self.fighters.values().map(|f| (f.location, f)));
        image::ImageBuffer::from_fn(self.dimensions.x + 1, self.dimensions.y + 1, |x, y| {
            let coord = Coordinate { x, y };
            match (fighters.get(&coord), self.caverns.contains(&coord)) {
                (Some(fighter), _) => fighter.to_pixel(),
                (None, true) => image::Rgb([255u8; 3]),
                (None, false) => image::Rgb([0u8; 3]),
            }
        })
    }

    fn turn_order(&self) -> Vec<FighterId> {
        let mut fighter_ids = self.fighters.keys().cloned().collect::<Vec<_>>();
        fighter_ids.sort_unstable_by_key(|id| {
            if let Some(fighter) = self.fighters.get(id) {
                Some((fighter.location.y, fighter.location.x))
            } else {
                None
            }
        });
        fighter_ids
    }

    pub fn round(mut self) -> Self {
        let turn_order = self.turn_order();
        for &fighter_id in turn_order.iter() {
            if Fighter::has_targets(&self, fighter_id) == Some(false) {
                return self;
            }
            Fighter::move_fighter(&mut self, fighter_id);
            Fighter::attack(&mut self, fighter_id);
        }
        self.round_number += 1;
        self
    }

    fn complete(&self) -> bool {
        self.fighters
            .values()
            .all({ |f| f.fighter_type == FighterType::Goblin })
            || self
                .fighters
                .values()
                .all({ |f| f.fighter_type == FighterType::Elf })
    }

    pub fn final_round(mut self) -> Self {
        while !self.complete() {
            self = self.round();
        }
        self
    }

    fn elf_count(&self) -> usize {
        self.fighters
            .values()
            .filter(|f| f.fighter_type == FighterType::Elf)
            .count()
    }

    pub fn final_round_no_elf_losses(mut self) -> Option<Self> {
        let elf_count = self.elf_count();
        while !self.complete() {
            self = self.round();
            if elf_count != self.elf_count() {
                return None;
            }
        }
        Some(self)
    }

    pub fn hit_points_sum(&self) -> u32 {
        self.fighters.values().map(|f| u32::from(f.health)).sum()
    }

    pub fn super_powered_elves(self) -> Option<Self> {
        for ap in 4..=200 {
            let mut candidate_battle = self.clone();
            for fighter in candidate_battle.fighters.values_mut() {
                if fighter.fighter_type == FighterType::Elf {
                    fighter.attack_power = ap;
                }
            }
            if let Some(successful_battle) = candidate_battle.final_round_no_elf_losses() {
                return Some(successful_battle);
            }
        }
        None
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

    fn battle_at_round(round: u32) -> Battle {
        let mut battle = battle();
        while battle.round_number != round {
            battle = battle.round();
        }
        battle
    }

    #[test]
    fn it_produces_the_correct_initial_image() {
        assert_eq!(
            battle_at_round(0).to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_1_0.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_is_correct_on_1st_round() {
        assert_eq!(
            battle_at_round(1).to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_1_1.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_is_correct_on_2nd_round() {
        assert_eq!(
            battle_at_round(2).to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_1_2.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 47);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 590);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_1_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_gives_the_correct_outcome_for_superpowered_elves() {
        let super_powered = battle().super_powered_elves().unwrap();
        assert_eq!(
            (super_powered.round_number, super_powered.hit_points_sum()),
            (29, 172)
        );
    }
}

#[cfg(test)]
mod worked_example_2 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_2.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_produces_the_correct_initial_image() {
        assert_eq!(
            battle().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_2_1.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_produces_the_correct_image_after_one_round() {
        assert_eq!(
            battle().round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_2_2.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 37);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 982);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_2_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }
}

#[cfg(test)]
mod worked_example_3 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_3.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 46);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 859);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_3_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }
}

#[cfg(test)]
mod worked_example_4 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_4.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 35);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 793);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_4_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }
}

#[cfg(test)]
mod worked_example_5 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_5.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 54);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 536);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_5_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }
}

#[cfg(test)]
mod worked_example_6 {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../worked_examples/example_6.txt")
            .parse()
            .unwrap()
    }

    #[test]
    fn it_has_the_correct_number_of_rounds() {
        assert_eq!(battle().final_round().round_number, 20);
    }

    #[test]
    fn it_has_the_correct_final_health() {
        assert_eq!(battle().final_round().hit_points_sum(), 937);
    }

    #[test]
    fn it_has_the_correct_final_image() {
        assert_eq!(
            battle().final_round().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../worked_examples/example_6_final.png"))
                .unwrap()
                .raw_pixels()
        );
    }
}

#[cfg(test)]
mod puzzle {
    use crate::Battle;

    fn battle() -> Battle {
        include_str!("../puzzle.txt").parse().unwrap()
    }

    #[test]
    fn it_produces_the_correct_initial_image() {
        assert_eq!(
            battle().to_image().into_vec(),
            image::load_from_memory(include_bytes!("../puzzle/0.png"))
                .unwrap()
                .raw_pixels()
        );
    }

    #[test]
    fn it_has_the_correct_outcome() {
        let final_round = battle().final_round();
        assert_eq!(
            (final_round.round_number, final_round.hit_points_sum()),
            (149, 2326)
        );
    }

    #[test]
    fn it_gives_the_correct_outcome_for_superpowered_elves() {
        let super_powered = battle().super_powered_elves().unwrap();
        assert_eq!(
            (super_powered.round_number, super_powered.hit_points_sum()),
            (48, 1268)
        );
    }
}
