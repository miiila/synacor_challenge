use bytes::{Buf, Bytes};
use std::fs::File;
use std::io::prelude::*;

const MAX: u16 = 2 << 14;

fn main() {
    let mut memory: [u16; MAX as usize] = [0; MAX as usize];
    let mut registers: [u16; 8] = [0; 8];
    let mut stack: Vec<u16> = vec![];

    // Load program
    let mut f = File::open("challenge.bin").unwrap();
    let mut buff: Vec<u8> = vec![];
    f.read_to_end(&mut buff).unwrap();
    let mut bytes_buff = Bytes::from(buff);

    let mut i = 0;
    while !bytes_buff.is_empty() {
        memory[i] = bytes_buff.get_u16_le();
        i += 1;
    }

    // Run program
    let mut ins_pointer = 0;
    loop {
        let ins = get_value(memory[ins_pointer], &registers);
        let mut a = memory[ins_pointer + 1];
        //dbg!(ins, ins_pointer, memory[ins_pointer], memory[ins_pointer+1], memory[ins_pointer+2]);
        //dbg!(registers);
        match ins {
            // halt
            0 => break,
            // set
            1 => {
                //dbg!(registers);
                //dbg!(ins_pointer, memory[ins_pointer+1], memory[ins_pointer+2]);
                a = a - (MAX);
                let b = get_value(memory[ins_pointer + 2], &registers);
                registers[a as usize] = b;
                ins_pointer += 3;
                //dbg!(registers);
            }
            // push
            2 => {
                a = get_value(a, &registers);
                stack.push(a);
                ins_pointer += 2;
            }
            // pop
            3 => {
                let res = stack.pop().unwrap();
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 2;
            }
            // eq
            4 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = (b == c) as u16;
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // jmp
            6 => ins_pointer = a as usize,
            // jt
            7 => {
                a = get_value(a, &registers);
                if a > 0 {
                    ins_pointer = memory[ins_pointer + 2] as usize;
                } else {
                    ins_pointer += 3;
                }
            }
            // jf
            8 => {
                a = get_value(a, &registers);
                if a == 0 {
                    ins_pointer = memory[ins_pointer + 2] as usize;
                } else {
                    ins_pointer += 3;
                }
            }
            // add
            9 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = (b + c) % (MAX);
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // out
            19 => {
                print!("{}", memory[ins_pointer + 1] as u8 as char);
                ins_pointer += 2
            }
            // noop
            21 => ins_pointer += 1,
            _ => ins_pointer += 1,
        }
    }
}

fn is_register(i: u16) -> bool {
    i >= MAX
}

fn get_value(val: u16, registers: &[u16; 8]) -> u16 {
    if is_register(val) {
        return registers[(val - MAX) as usize];
    }
    return val;
}

fn set_value(
    val: u16,
    addr: u16,
    registers: &mut [u16; 8],
    memory: &mut [u16; MAX as usize],
) -> Result<(), ()> {
    if is_register(addr) {
        registers[(addr - MAX) as usize] = val;
    } else {
        memory[addr as usize] = val;
    }

    return Ok(());
}
