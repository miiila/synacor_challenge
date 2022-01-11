use std::fs::File;
use bytes::{Buf, Bytes};
use std::io::prelude::*;

const MAX: u16 = 2 << 14;

pub fn handle_custom_command(
    command: &str,
    registers: &mut [u16; 8],
    memory: &mut [u16; MAX as usize],
    stack: &Vec<u16>,
    debug_file: &mut File,
) {
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
        "/markInDebug\n" => {
            debug_file.write(format!("\n\n === DEBUG MARKER === \n \n").as_bytes()).unwrap();
        }
        "/loadMemory\n" => {
            let mut f = File::open("dump").unwrap();
            let mut buff: Vec<u8> = vec![];
            f.read_to_end(&mut buff).unwrap();
            let mut bytes_buff = Bytes::from(buff);

            let mut i = 0;
            while !bytes_buff.is_empty() {
                memory[i] = bytes_buff.get_u16();
                i += 1;
            }
        }
        "/getItems\n" => {
            memory[0x14dc / 2] = 0x0; // tablet
            memory[0x1524 / 2] = 0x0; // blue coine
            memory[0x152c / 2] = 0x0; //teleporter
        }
        "/hackTeleport\n" => {
            memory[5489] = 21; // noop
            memory[5490] = 21; // noop
            memory[5495] = 7; // reverse eq check
            registers[7] = 25734;
        }
        _ => (),
    }
}

pub fn write_debug_message(message: String, registers: &[u16; 8], debug_file: &mut File) {
    debug_file.write(format!("{} [32768:{}, 32769:{}, 32770:{}, 32771:{}, 32772:{}, 32773:{}, 32774:{}, 32775:{}]\n", message, registers[0], registers[1], registers[2], registers[3], registers[4], registers[5], registers[6],registers[7],).as_bytes()).unwrap();
}
