mod intcode;
mod util;

use intcode::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut prog_file : File = File::open("/home/cameron/PersonalProjects/advent_of_code_19/resrcs/input_2.txt")?;
    let mut prog = String::new();
    prog_file.read_to_string(&mut prog)?;

    println!("The program returned: {}", exec_intcode(parse_to_mem(prog), 0));

    Ok(())
}
