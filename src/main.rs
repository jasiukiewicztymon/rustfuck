use std::io;
use std::env;
use std::fs;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn get_char() -> u8 {
    use io::Read;
    // Ideally you would buffer with BufReader
    let mut stdino = io::stdin();
    let mut buf = [0u8];
    stdino.read_exact(&mut buf).unwrap();
    buf[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut HexPrint: bool = false;
    let mut BetterPrint: bool = false;
    let mut Light: bool = false;

    let mut filename;

    // Getting args
    if args.len() > 1 {
        filename = &args[args.len() - 1];
        let args = &args[1..args.len() - 1];

        for arg in args {
            let mut arg: &str = arg;
            match arg {
                "--hex" =>  HexPrint = true,
                "--bprint" => BetterPrint = true,
                _ => panic!("[🧠 Fuckrust] : Fatal error, unknown argument")
            }
        }
    }
    else {
        panic!("[🧠 Fuckrust] : Fatal error, any file name has been passed");
    }

    let source: String = String::from(fs::read_to_string(filename)
    .expect("Should have been able to read the file"));

    let mut memory: Vec<i32> = vec![0]; let mut loops: Vec<i32> = Vec::new();
    let mut index: i32 = 0; let mut i: i32 = 0;

    let mut stack: Vec<i32> = Vec::new();

    // set the loops map
    for j in 0..source.len() {
        loops.push(j.try_into().unwrap());

        if source.as_bytes()[j] == b'[' {
            stack.push(j.try_into().unwrap());
        } else if source.as_bytes()[j] == b']' {
            let t = stack[usize::try_from(stack.len() - 1).unwrap()]; 
            stack.pop();
            loops[usize::try_from(t).unwrap()] = i32::try_from(j).unwrap();
            loops[usize::try_from(j).unwrap()] = t;
        }
    }

    while i < source.len().try_into().unwrap() {        
        let ch = source.as_bytes()[usize::try_from(i).unwrap()];
        match ch {
            b'>' => {
                index += 1;
                if index >= (memory.len() - 1).try_into().unwrap() {
                    memory.push(0);
                }
            }   
            b'<' => {
                if index > 0 {
                    index -= 1;
                }
            }
            b'[' => {
                if memory[usize::try_from(index).unwrap()] == 0 {
                    i = loops[usize::try_from(i).unwrap()];
                }
            }
            b']' => {
                if memory[usize::try_from(index).unwrap()] != 0 {
                    i = loops[usize::try_from(i).unwrap()];
                }
            }
            b',' => {
                let c = get_char();
                memory[usize::try_from(index).unwrap()] = c as i32;
            }
            b'.' => {
                let temp: u32 = memory[usize::try_from(index).unwrap()].try_into().unwrap();

                if BetterPrint {
                    if Light {
                        print!("\x1b[1m");
                    }
                    Light = !Light;
                }

                if HexPrint {
                    print!("0x{:04}", temp);
                } else {                    
                    if let Some(ch) = std::char::from_u32(temp) {
                        print!("{}", ch);
                    }
                }    

                if BetterPrint {
                    print!("\x1b[0m");
                }
            }
            b'+' => {
                memory[usize::try_from(index).unwrap()] += 1;
            }
            b'-' => {
                if memory[usize::try_from(index).unwrap()] > 0 {
                    memory[usize::try_from(index).unwrap()] -= 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
}
