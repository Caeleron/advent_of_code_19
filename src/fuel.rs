pub fn calc_from_mass(mass : usize) -> usize { if mass < 6 { 0 } else { mass / 3 - 2 }}

pub fn correct_for_rocket_tyranny(fuel : usize) -> usize {
    match fuel {
        0 => 0,
        _ => fuel + correct_for_rocket_tyranny(calc_from_mass(fuel)),
    }
}
