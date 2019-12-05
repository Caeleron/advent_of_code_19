use crate::util::*;

pub fn parse_to_mem(s: String) -> Vec::<usize> { s.split(',').filter_map(string_to_usize).collect::<Vec::<usize>>() }

pub fn exec_intcode(mut mem: Vec::<usize>, ip: usize) -> usize {
    let opcode = mem[ip];
    let src1 = mem[ip + 1];
    let src2 = mem[ip + 2];
    let dest = mem[ip + 3];

    match opcode {
        1 => { mem[dest] = mem[src1] + mem[src2]; exec_intcode(mem, ip + 4) },
        2 => { mem[dest] = mem[src1] * mem[src2]; exec_intcode(mem, ip + 4) },
        99 => mem[0],
        _ => panic!("Unknown opcode! IP pointed to {} containing {}!", ip, opcode)
    }
}
