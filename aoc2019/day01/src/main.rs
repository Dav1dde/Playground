use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    let f = BufReader::new(File::open("../input.txt").unwrap());

    let result: (i32, i32) = f.lines()
        .map(|x| x.unwrap().parse().unwrap())
        .map(|x| (calculate_fuel(x), calculate_total_fuel(x)))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("Fuel required for launch without fuel -> {}", result.0);
    println!("Total fuel required for launch -> {}", result.1);
}

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3)  - 2
}


#[test]
fn test_calculate_fuel() {
    assert_eq!(calculate_fuel(12), 2);
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 654);
    assert_eq!(calculate_fuel(100756), 33583);
}

fn calculate_total_fuel(fuel_mass: i32) -> i32 {
    let mut total_fuel = 0;
    let mut remaining_fuel = calculate_fuel(fuel_mass);

    while remaining_fuel > 0 {
        total_fuel = total_fuel + remaining_fuel;
        remaining_fuel = calculate_fuel(remaining_fuel);
    }

    return total_fuel;
}

#[test]
fn test_calculate_total_fuel() {
    assert_eq!(calculate_total_fuel(2), 0);
    assert_eq!(calculate_total_fuel(654), 312);
    assert_eq!(calculate_total_fuel(33583), 16763);
}
