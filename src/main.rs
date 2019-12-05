mod intcode;
mod util;

use intcode::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut prog_file : File = File::open("/home/cameron/PersonalProjects/advent_of_code_19/resrcs/input_2.txt")?;
    let mut prog = String::new();
    prog_file.read_to_string(&mut prog)?;

    let mut mem = parse_to_mem(prog);

    mem[1] = 0;
    mem[2] = 0;

    let slope1 = { let mut mem_copy = mem.to_vec(); mem_copy[1] = 1; exec_intcode(mem_copy, 0) - exec_intcode(mem.to_vec(), 0) };

    let noun = 19690720 / slope1;
    mem[1] = noun;

    let slope2 = { let mut mem_copy = mem.to_vec(); mem_copy[2] = 1; exec_intcode(mem_copy, 0) - exec_intcode(mem.to_vec(), 0) };

    let zero2 = { let mem_copy = mem.to_vec(); exec_intcode(mem_copy, 0) };
    let verb = (19690720 - zero2) / slope2;

    println!("Output: {}, Noun: {}, Verb: {}, Solution: {}", exec_intcode(mem, 0), noun, verb, 100 * noun + verb);

    Ok(())
}
