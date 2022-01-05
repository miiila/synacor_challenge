use bytes::{Buf, Bytes};
use std::fs::File;
use std::io;
use std::io::prelude::*;

const MAX: u16 = 2 << 14;

fn main() {
    let mut memory: [u16; MAX as usize] = [0; MAX as usize];
    let mut registers: [u16; 8] = [0; 8];
    let mut stack: Vec<u16> = vec![];
    let mut input_buffer: Vec<u8> = vec![];
    let mut input_buffer_pos = 0;

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
        //dbg!(ins, ins_pointer, memory[ins_pointer], memory[ins_pointer+1], memory[ins_pointer+2], memory[ins_pointer+3]);
        //dbg!(registers);
        match ins {
            // halt
            0 => break,
            // set
            1 => {
                a = a - (MAX);
                let b = get_value(memory[ins_pointer + 2], &registers);
                registers[a as usize] = b;
                ins_pointer += 3;
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
            // gt
            5 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = (b > c) as u16;
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
            // mult
            10 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = (b as u32 * c as u32) % MAX as u32;
                set_value(res as u16, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // mod
            11 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = b % c;
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // and
            12 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = b & c;
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // or
            13 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let c = get_value(memory[ins_pointer + 3], &registers);
                let res = b | c;
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 4;
            }
            // not
            14 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let res = MAX - b - 1;
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 3;
            }
            // rmem
            15 => {
                let b = get_value(memory[ins_pointer + 2], &registers);
                let res = memory[b as usize];
                set_value(res, a, &mut registers, &mut memory).unwrap();
                ins_pointer += 3;
            }
            // wmem
            16 => {
                a = get_value(a, &registers);
                let b = get_value(memory[ins_pointer + 2], &registers);
                memory[a as usize] = b;
                ins_pointer += 3;
            }
            // call
            17 => {
                stack.push((ins_pointer+2) as u16);
                ins_pointer = get_value(a, &registers) as usize;
            }
            // ret
            18 => {
                if stack.is_empty() {
                    break;
                }
                let res = stack.pop().unwrap();
                ins_pointer = res as usize;
            }
            // out
            19 => {
                a = get_value(a, &registers);
                print!("{}", a as u8 as char);
                ins_pointer += 2
            }
            // in
            20 => {
                if input_buffer_pos == input_buffer.len() {
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).unwrap();
                    if s.starts_with('/') {
                        println!("Custom command detected");
                        handle_custom_command(&s, &registers, &memory, &stack);
                        continue;
                    }
                    input_buffer = s.into_bytes();
                    input_buffer_pos = 0;
                }
                let ch = input_buffer[input_buffer_pos];
                set_value(ch.into(), a, &mut registers, &mut memory).unwrap();
                input_buffer_pos+=1;
                ins_pointer += 2;
            }
            // noop
            21 => ins_pointer += 1,
            _ => panic!("Unknown code provided"),
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

fn handle_custom_command(command: &str, registers: &[u16; 8], memory: &[u16; MAX as usize], stack: &Vec<u16>) {
    match command {
        "/showRegisters\n" => {
            dbg!(registers);
        }
        "/showStack\n" => {
            dbg!(stack);
        }
        "/dumpMemory\n" => {
            let mut file = File::create("dump").unwrap();
            for m in memory {
                 file.write(&m.to_be_bytes()).unwrap();
            }
        }
        _ => ()
    }
}
