fn fuel_required(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

#[test]
fn test_fuel_required() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(fuel_required(100756), 33583);
}

fn accumulative_fuel_required(mut mass: u64) -> u64 {
    let mut sum = 0;
    while mass != 0 {
        mass = fuel_required(mass);
        sum += mass;
    }
    sum
}

#[test]
fn test_accumulative_fuel_required() {
    assert_eq!(accumulative_fuel_required(14), 2);
    assert_eq!(accumulative_fuel_required(1969), 966);
    assert_eq!(accumulative_fuel_required(100756), 50346);
}

fn main() {
    let input = include_str!("../input");
    let sum = input
        .lines()
        .map(|line| line.parse::<u64>())
        .map(|rmass| rmass.map(accumulative_fuel_required))
        .sum::<Result<u64, _>>();
    println!("{:?}", sum);
}
