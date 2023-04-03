use std::io;
use std::env;
use std::fs;

use crossterm::{queue, style::Print};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn get_char() -> u8 {
    use io::Read;
    // Ideally you would buffer with BufReader
    let mut stdino = io::stdin();
    let mut buf = [0u8];
    stdino.read_exact(&mut buf).unwrap();
    buf[0]
}
pub fn write(content: String) {
    use io::Write;

    let mut stdout = io::stdout();
    queue!(stdout, Print(content.to_string()));
    stdout.flush();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut HexPrint: bool = false;
    let mut BetterPrint: bool = false;
    let mut DebugPrint: bool = false;

    let mut Light: bool = false;

    let mut filename;

    if args.len() > 1 {
        filename = &args[args.len() - 1];
        let args = &args[1..args.len() - 1];

        for arg in args {
            let mut arg: &str = arg;
            match arg {
                "--hex" =>  HexPrint = true,
                "--bprint" => BetterPrint = true,
                "--debug" => DebugPrint = true,
                _ => panic!("[🧠 Fuckrust] : Fatal error, unknown argument")
            }
        }
    }
    else {
        panic!("[🧠 Fuckrust] : Fatal error, any file name has been passed");
    }

    let source: String = String::from(fs::read_to_string(filename)
    .expect("Should have been able to read the file"));
    let mut memory: Vec<i32> = Vec::from([0]); let mut loops: Vec<i32> = Vec::new();
    let mut index: i32 = 0; let mut i: i32 = 0;
    while i < source.len().try_into().unwrap() {
        if DebugPrint {
            write(String::from("[🧠 Fuckrust DEBUG] : iteration number ".to_owned() + &i.to_string() + "\n"));
        }
        
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
                loops.push(i);
            }
            b']' => {
                if memory[usize::try_from(index).unwrap()] == 0 {
                    loops.pop();
                }
                else {
                    i = loops[usize::try_from(loops.len() - 1).unwrap()];
                    if DebugPrint {
                        write(String::from("[🧠 Fuckrust DEBUG] : go to iteration number ".to_owned() + &(i + 1).to_string() + "\n"));
                    }
                }
            }
            b',' => {
                enable_raw_mode();

                let c = get_char();
                memory[usize::try_from(index).unwrap()] = c as i32;

                disable_raw_mode();
            }
            b'.' => {
                if BetterPrint && Light {
                    write(String::from("\x1b[1m"));
                }

                if HexPrint {
                    let temp: u32 = memory[usize::try_from(index).unwrap()].try_into().unwrap();

                    if temp < 32 || temp > 126 {
                        if temp < 10 {
                            write(String::from("0x000".to_owned() + &temp.to_string()));
                        }
                        else if temp < 100 {
                            write(String::from("0x00".to_owned() + &temp.to_string()));
                        }
                        else if temp < 1000 {
                            write(String::from("0x0".to_owned() + &temp.to_string()));
                        }
                        else {
                            write(String::from("\x1b[31m0x0000"));
                        }
                    }
                    else {
                        write(String::from(std::char::from_u32(memory[usize::try_from(index).unwrap()].try_into().unwrap()).unwrap()));
                    }
                }
                else { write(String::from(std::char::from_u32(memory[usize::try_from(index).unwrap()].try_into().unwrap()).unwrap())); }
                Light = !Light;
                write(String::from("\x1b[0m"));
            }
            b'+' => {
                memory[usize::try_from(index).unwrap()] += 1;
            }
            b'-' => {
                if memory[usize::try_from(index).unwrap()] > 0 {
                    memory[usize::try_from(index).unwrap()] -= 1;
                }
            }
            _ => {
            }
        }
        i += 1;
    }
    if loops.len() > 0 {
        panic!("[🧠 Fuckrust] : Fatal error, a loop has been started but any end has been declared");
    }
}