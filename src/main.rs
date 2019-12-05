mod fuel;

use fuel::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut fuel_file : File = File::open("/home/cameron/PersonalProjects/advent_of_code_19/resrcs/input_1.txt")?;
    let mut fuel_file_contents = String::new();
    fuel_file.read_to_string(&mut fuel_file_contents)?;

    let fuel_needed = fuel_file_contents.lines().filter_map(|x| x.parse::<usize>().ok()).map(calc_from_mass).sum::<usize>();
    println!("The amount of fuel you need is: {} units", fuel_needed);

    Ok(())
}
