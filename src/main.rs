use bytes::{Buf, Bytes};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut memory: [u16; 2 << 14] = [0; 2 << 14];
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
        let mut ins = memory[ins_pointer];
        //dbg!(ins, ins_pointer, memory[ins_pointer], memory[ins_pointer+1], memory[ins_pointer+2]);
        if is_register(ins) {
            ins = registers[(ins - (2 << 14)) as usize]
        }
        let mut a = memory[ins_pointer + 1];
        //dbg!(ins, ins_pointer, memory[ins_pointer], memory[ins_pointer+1], memory[ins_pointer+2]);
                //dbg!(registers);
        match ins {
            0 => break,
            1 => {
                //dbg!(registers);
                //dbg!(ins_pointer, memory[ins_pointer+1], memory[ins_pointer+2]);
                let reg = a - (2 << 14);
                let mut b = memory[ins_pointer + 2];
                if is_register(b) {
                    b = registers[(b - (2 << 14)) as usize]
                }
                registers[reg as usize] = b;
                ins_pointer += 3;
                //dbg!(registers);
            }
            2 => {
                if is_register(a) {
                    a = registers[(a - (2 << 14)) as usize]
                }
                stack.push(a);
                ins_pointer += 2;
            }
            3 => {
                let res = stack.pop().unwrap();
                //dbg!(res);
                if is_register(a) {
                    registers[(a - (2 << 14)) as usize] = res;
                } else {
                    memory[a as usize] = res;
                }
                ins_pointer += 2;
            }
            4 => {
                let mut b = memory[ins_pointer+2];
                let mut c = memory[ins_pointer+3];
                if is_register(b) {
                    b = registers[(b - (2 << 14)) as usize]
                }
                if is_register(c) {
                    c = registers[(c - (2 << 14)) as usize]
                } 
                let res = (b==c) as u16;
                if is_register(a) {
                    registers[(a - (2 << 14)) as usize] = res;
                } else {
                   memory[a as usize] = res; 
                }
                ins_pointer += 4;
            }
            6 => ins_pointer = a as usize,
            7 => {
                if is_register(a) {
                    a = registers[(a - (2 << 14)) as usize]
                }
                if a > 0 {
                    ins_pointer = memory[ins_pointer + 2] as usize;
                } else {
                    ins_pointer += 3;
                }
            }
            8 => {
                if is_register(a) {
                    a = registers[(a - (2 << 14)) as usize]
                }
                if a == 0 {
                    ins_pointer = memory[ins_pointer + 2] as usize;
                } else {
                    ins_pointer += 3;
                }
            }
            9 => {
                let mut b = memory[ins_pointer+2];
                let mut c = memory[ins_pointer+3];
                if is_register(b) {
                    b = registers[(b - (2 << 14)) as usize]
                }
                if is_register(c) {
                    c = registers[(c - (2 << 14)) as usize]
                } 
                let res = (b + c) % (2<<14);
                if is_register(a) {
                    registers[(a - (2 << 14)) as usize] = res;
                } else {
                   memory[a as usize] = res; 
                }
                ins_pointer += 4;
            }
            19 => {
                print!("{}", memory[ins_pointer + 1] as u8 as char);
                ins_pointer += 2
            }
            21 => ins_pointer += 1,
            _ => ins_pointer += 1,
        }
    }
}

fn is_register(i: u16) -> bool {
    i >= 2 << 14
}
