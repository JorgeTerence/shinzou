struct CPU {
    mbr: u64,
    mar: u16,
    ir: u8,
    ibr: u16,
    pc: usize,
    ac: i64,
    mq: u32,
    jumped: bool,
}

impl CPU {
    fn new() -> Self {
        Self {
            mbr: 0,
            mar: 0,
            ir: 0,
            ibr: 0,
            pc: 0,
            ac: 0,
            mq: 0,
            jumped: false,
        }
    }
}

// TODO: BIG allow negative numbers
pub fn run(exe: String) {
    let mut cpu = CPU::new();
    let mut mem = exe
        .lines()
        .map(|s| u64::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u64>>();

    'loopy: while cpu.pc < mem.len() {
        cpu.mbr = mem[cpu.pc];
        let pair = [cpu.mbr / 0x100000, cpu.mbr % 0x10000];

        cpu.jumped = false;

        for half in pair {
            // Jumping to correct side of the word
            if cpu.jumped {
                cpu.jumped = false;
                continue;
            }

            let arg: usize = (half / 0x1000).try_into().unwrap();
            cpu.ir = (half % 0x1000) as u8;

            match cpu.ir {
                0 | 0xFF => (),
                0b0000_0001 => {
                    cpu.ac = mem[arg] as i64;
                }
                0b0000_1101 => {
                    cpu.pc = arg;
                    continue 'loopy;
                }
                0b0000_1110 => {
                    cpu.pc = arg;
                    cpu.jumped = true;
                    continue 'loopy;
                }
                0b0000_1111 => {
                    if cpu.ac < 0 {
                        cpu.pc = arg;
                        continue 'loopy;
                    }
                }
                0b0001_0000 => {
                    if cpu.ac < 0 {
                        cpu.pc = arg;
                        cpu.jumped = true;
                        continue 'loopy;
                    }
                }
                0b0000_0101 => {
                    cpu.ac += mem[arg] as i64;
                }
                0b0000_0110 => {
                    cpu.ac -= mem[arg] as i64;
                }
                0b0010_0001 => {
                    mem[arg] = cpu.ac as u64;
                }
                0b0001_0010 => {
                    mem[arg] = mem[arg] % 0x100 + cpu.ac as u64 + mem[arg] / 0x100000;
                }
                0b0001_0011 => {
                    mem[arg] = mem[arg] % 0x100000 + cpu.ac as u64 + mem[arg] / 0x100;
                }
                0b1000_0001 => {
                    print!("{}", mem[arg]);
                }
                0b1000_0010 => {
                    print!("{}", mem[arg] as u8 as char);
                }
                _ => {
                    println!("Unknown operator: {:08b}", arg);
                    println!("Full word: {:b}", half);
                }
            };
        }

        cpu.pc += 1; // this is wrong
    }
}
