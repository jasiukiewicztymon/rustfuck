fn main() {
    let alpha: Vec<String> = 
    vec![
    String::from("0x00"), String::from("0x01"), String::from("0x02"), String::from("0x03"), String::from("0x04"), 
    String::from("0x05"), String::from("0x06"), String::from("0x07"), String::from("0x08"), String::from("0x09"), 
    String::from("0x10"), String::from("0x11"), String::from("0x12"), String::from("0x13"), String::from("0x14"), 
    String::from("0x15"), String::from("0x16"), String::from("0x17"), String::from("0x18"), String::from("0x19"),
    String::from("0x20"), String::from("0x21"), String::from("0x22"), String::from("0x23"), String::from("0x24"), 
    String::from("0x25"), String::from("0x26"), String::from("0x27"), String::from("0x28"), String::from("0x29"),
    String::from("0x30"), String::from("0x31"), String::from(" "), String::from("!"), String::from("\""), 
    String::from("#"), String::from("$"), String::from("%"), String::from("&"), String::from("'"), 
    String::from("("), String::from(")"), String::from("*"), String::from("+"), String::from(","), 
    String::from("-"), String::from("."), String::from("/"), String::from("0"), String::from("1"), 
    String::from("2"), String::from("3"), String::from("4"), String::from("5"), String::from("6"), 
    String::from("7"), String::from("8"), String::from("9"), String::from(":"), String::from(";"),
    String::from("<"), String::from("="), String::from(">"), String::from("?"), String::from("@"), 
    String::from("A"), String::from("B"), String::from("C"), String::from("D"), String::from("E"), 
    String::from("F"), String::from("G"), String::from("H"), String::from("I"), String::from("J"), 
    String::from("K"), String::from("L"), String::from("M"), String::from("N"), String::from("O"),
    String::from("P"), String::from("Q"), String::from("R"), String::from("S"), String::from("T"), 
    String::from("U"), String::from("V"), String::from("W"), String::from("X"), String::from("Y"), 
    String::from("Z"), String::from("["), String::from("\\"), String::from("]"), String::from("^"), 
    String::from("_"), String::from("`"), String::from("a"), String::from("b"), String::from("c"),
    String::from("d"), String::from("e"), String::from("f"), String::from("g"), String::from("h"), 
    String::from("i"), String::from("j"), String::from("k"), String::from("l"), String::from("m"),
    String::from("n"), String::from("o"), String::from("p"), String::from("q"), String::from("r"), 
    String::from("s"), String::from("t"), String::from("u"), String::from("v"), String::from("w"),
    String::from("x"), String::from("y"), String::from("z"), String::from("{"), String::from("|"), 
    String::from("}"), String::from("~")
    ];

    let binding = "a".to_string();
    let mut prompt = binding.chars();
    let source: String = String::from(">+.,.<.");
    let mut memory: Vec<i32> = Vec::from([0]); let mut loops: Vec<i32> = Vec::new();
    let mut index: i32 = 0; let mut i: i32 = 0;
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
                loops.push(i + 1);
            }
            b']' => {
                if memory[usize::try_from(index).unwrap()] == 0 {
                    loops.pop();
                }
                else {
                    i = loops[usize::try_from(loops.len() - 1).unwrap()];
                }
            }
            b'.' => {
                if memory[usize::try_from(index).unwrap()] < alpha.len().try_into().unwrap() {
                    if memory[usize::try_from(index).unwrap()] < 32 {
                        print!("\x1b[1m");
                    }
                    print!("{}", alpha[usize::try_from(memory[usize::try_from(index).unwrap()]).unwrap()]);
                    if memory[usize::try_from(index).unwrap()] < 32 {
                        print!("\x1b[0m");
                    }
                }
            }
            b',' => {
                let c = prompt.next().unwrap();
                memory[usize::try_from(index).unwrap()] = c as i32;
            }
            b'+' => {
                memory[usize::try_from(index).unwrap()] += 1;
            }
            b'-' => {
                if memory[usize::try_from(index).unwrap()] == 0 {
                    memory[usize::try_from(index).unwrap()] -= 1;
                }
            }
            _ => {
                panic!("[🧠 Fuckrust] : Invalid token {}", ch);
            }
        }
        i += 1;
    }
    if loops.len() > 0 {
        panic!("[🧠 Fuckrust] : Fatal error, a loop has been started but any end has been declared");
    }
}
