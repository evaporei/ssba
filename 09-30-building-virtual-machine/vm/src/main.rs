type Memory = [u8; 20];

type Registers = [u8; 3];

fn compute(memory: &mut Memory) {
    let mut registers: Registers = [0x00, 0x00, 0x00]; // PC, R1, and R2

    loop {
        let (pc_inc, instruction_address) = fetch(registers[0] as usize, memory);
        registers[0] += pc_inc as u8;

        let op = decode(instruction_address, &memory);

        if let Op::Halt = op {
            break;
        }

        op.execute(&mut registers, memory);
    }
}

fn fetch(pc: usize, memory: &Memory) -> (u8, Vec<u8>) {
    match memory[pc] {
        0x01..=0x04 => (3, memory[pc..pc + 3].into()),
        0xff => (1, vec![memory[pc]]),
        _ => panic!("can't know size to fetch unknown instruction"),
    }
}

fn decode(instruction: Vec<u8>, memory: &Memory) -> Op {
    match instruction[..] {
        [0x01, reg, addr] => {
            let value = u16::from_le_bytes(memory[addr as usize..addr as usize + 2].try_into().unwrap());
            Op::Load(reg, value)
        },
        [0x02, reg, addr] => Op::Store(reg, addr),
        [0x03, reg1, reg2] => Op::Add(reg1, reg2),
        [0x04, reg1, reg2] => Op::Sub(reg1, reg2),
        [0xff] => Op::Halt,
        _ => panic!("can't decode unknown instruction"),
    }
}

#[derive(Debug)]
enum Op {
    Load(u8, u16),// = 0x01
    Store(u8, u8),// = 0x02
    Add(u8, u8),// = 0x03
    Sub(u8, u8),// = 0x04
    Halt,// = 0xFF
}

impl Op {
    fn execute(self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Op::Load(reg, value) => {
                registers[reg as usize] = value as u8;
            }
            Op::Store(reg, addr) => {
                memory[addr as usize] = registers[reg as usize];
            }
            Op::Add(reg1, reg2) => {
                let r1 = registers[reg1 as usize];
                let r2 = registers[reg2 as usize];
                let sum = r1.wrapping_add(r2);
                registers[reg1 as usize] = sum;
            }
            Op::Sub(reg1, reg2) => {
                let r1 = registers[reg1 as usize];
                let r2 = registers[reg2 as usize];
                let sub = r1.wrapping_sub(r2);
                registers[reg1 as usize] = sub;
            }
            Op::Halt => {
                // nothing, the break; is implemented in 'compute'
            }
        }
    }
}

fn main() {
    // 255 + 3 = 258
    let mut memory = [
      0x01, 0x01, 0x10,  // 0x00: load A 0x10
      0x01, 0x02, 0x12,  // 0x03: load B 0x12
      0x03, 0x01, 0x02,  // 0x06: add A B
      0x02, 0x01, 0x0e,  // 0x09: store A 0x0e
      0xff,              // 0x0c: halt
      0x00,              // 0x0d: <<unused>>
      0x00, 0x00,        // 0x0e: output
      0xff, 0x00,        // 0x10: input X = 255
      0x03, 0x00,        // 0x12: input Y = 3
    ];

    compute(&mut memory);
    println!("Testing 255 + 3 = 258");
    assert_eq!(memory[0x0e], 2);
    assert_eq!(memory[0x0f], 1);

    // 256 - 3 = 253
    let mut memory = [
      0x01, 0x01, 0x10,  // 0x00: load A 0x10
      0x01, 0x02, 0x12,  // 0x03: load B 0x12
      0x04, 0x01, 0x02,  // 0x06: sub A B
      0x02, 0x01, 0x0e,  // 0x09: store A 0x0e
      0xff,              // 0x0c: halt
      0x00,              // 0x0d: <<unused>>
      0x00, 0x00,        // 0x0e: output
      0x00, 0x01,        // 0x10: input X = 256
      0x03, 0x00,        // 0x12: input Y = 3
    ];

    compute(&mut memory);
    println!("Testing 256 - 3 = 253");
    assert_eq!(memory[0x0e], 253);
    assert_eq!(memory[0x0f], 0);
}
