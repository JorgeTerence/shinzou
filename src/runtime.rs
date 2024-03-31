struct CPU {
    mbr: [u32; 2],
    mar: u16,
    ir: u8,
    ibr: u16,
    pc: usize,
    ac: u32,
    mq: u32,
}

impl CPU {
    fn new() -> Self {
        Self {
            mbr: [0, 0],
            mar: 0,
            ir: 0,
            ibr: 0,
            pc: 0,
            ac: 0,
            mq: 0,
        }
    }
}

pub fn run(exe: String) {
    let mut cpu = CPU::new();
    let binding = exe
        .lines()
        .map(|l| l.split_at(20))
        .flat_map(|tup| [tup.0, tup.1])
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let mut mem = binding.chunks(2).collect::<Vec<&[u32]>>();

    while cpu.pc < mem.len() {
        for half in mem[cpu.pc] {
            let arg: usize = (half / 0x1000).try_into().unwrap();
            cpu.ir = (half % 0x1000) as u8;
            // stor, jump left if
            match cpu.ir {
                0 | 0xFF => (),
                0b0000_0001 => {
                    cpu.ac = mem[arg];
                }
                0b0000_0101 => {
                    cpu.ac += mem[arg];
                }
                0b0000_0110 => {
                    cpu.ac -= mem[arg];
                }
                0b0010_0001 => {
                    mem[arg] = cpu.ac;
                }
                0b0001_0010 => {
                    mem[arg][0] = cpu.ac;
                }
                0b0001_0011 => {
                    mem[arg][1] = cpu.ac;
                }
                0b1000_0001 => {
                    print!("{}", mem[arg][0] % 0xFF);
                }
                0b1000_0010 => {
                    print!("{}", (mem[arg][0] % 0xFF) as u8 as char);
                }
                _ => {
                    println!("Unknown operator: {:08b}", arg);
                }
            };
        }

        cpu.pc += 1; // this is wrong
    }
}
