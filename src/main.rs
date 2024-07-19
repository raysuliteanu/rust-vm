extern crate core;

use std::{fs::File, mem, path::PathBuf};

fn main() -> std::io::Result<()> {
    println!("bootstrapping");
    let machine = &mut Machine::start();

    let file_path = PathBuf::from("prog.txt");
    let mut file = File::open(file_path)?;
    machine.load_program(&mut file)?;
    machine.debug_dump_state();
    machine.exec();
    println!("shutting down");
    Ok(())
}

type Word = u16;
struct Cpu {
    num: u8,
    registers: Registers,
}

impl Cpu {
    fn new(num: u8) -> Self {
        Self {
            num,
            registers: Registers::default(),
        }
    }

    fn set_flag(&mut self, flag: Word) {
        self.registers.flags.0 |= flag;
    }

    fn is_halt(&self) -> bool {
        self.registers.flags.0 & FLAG_H != 0
    }
    
    fn halt(&self) {
        println!("CPU {} halting", self.num);
    }

    fn dump(&self) {
        println!("CPU {}", self.num);
        self.registers.dump();
    }
}

/// equal flag
static FLAG_E: Word = 0b0010;
/// halt flag
static FLAG_H: Word = 0b1000;

/// a CPU register is the size of a Word
#[derive(Debug, Default)]
struct Register(Word);

#[derive(Debug, Default)]
struct Registers {
    /// instruction pointer
    ip: Register,
    /// stack pointer
    sp: Register,
    /// base pointer
    bp: Register,
    /// accumulator
    ac: Register,
    /// general purpose registers
    gp1: Register,
    gp2: Register,
    gp3: Register,
    gp4: Register,
    /// Flags
    flags: Register,
}

impl Registers {
    fn dump(&self) {
        println!("CPU Registers:");
        println!("IP: {}\tSP: {}\tBP: {}\tAC: {}", self.ip.0, self.sp.0, self.bp.0, self.ac.0);
        println!("GP1: {}\tGP2: {}\tGP3: {}\tGP4: {}", self.gp1.0, self.gp2.0, self.gp3.0, self.gp4.0);
        println!("Flags: {:0b}", self.flags.0);
    }
}

const WORD_SIZE: usize = mem::size_of::<Word>();
const MEM_SIZE: usize = 4096 * WORD_SIZE;

struct Memory {
    mem: [u8; MEM_SIZE],
}

impl Memory {
    fn new() -> Self {
        Memory { mem: [0u8; MEM_SIZE] }
    }

    fn get(&self, addr: Word) -> Word {
        println!("mem[{:0X}]", addr);
        let phys_addr = (addr / WORD_SIZE as Word) as usize;
        println!("phys_addr[{:0X}]", phys_addr);
        let hi = self.mem[phys_addr] as Word;
        let low = self.mem[phys_addr + 1] as Word;
        (hi << WORD_SIZE) | low
    }

    fn load(&mut self, _file: &mut File) -> std::io::Result<usize> {
        self.mem[0] = 0x00;
        self.mem[1] = 0x0e;
        Ok(2)
        // let read = file.read(&mut self.mem)?;
        // assert!(read <= MEM_SIZE);
        // Ok(read)
    }

    fn dump(&self) {
        self.dump_stats();
        self.dump_memory();
    }

    fn dump_stats(&self) {
        println!("Memory Statistics");
        println!("size: {}\navailable: {}b", self.mem.len(), -1); // need to keep track of avail mem
    }

    fn dump_memory(&self) {
        println!("Memory Dump");
        for line in self.mem.chunks(64) {
            for group in line.chunks(8) {
                for b in group {
                    print!("{:02X}", b);
                }
                print!(" ");
            }
            println!();
        }
    }
}

struct Machine {
    cpus: [Cpu; 1], // just 1 for now
    memory: Memory,
}

impl Machine {
    fn start() -> Self {
        Self {
            cpus: [Cpu::new(1)],
            memory: Memory::new(),
        }
    }

    fn load_program(&mut self, file: &mut File) -> std::io::Result<()> {
        println!("loading program {:?}", file);
        let _ = self.memory.load(file)?;
        Ok(())
    }

    fn exec(&mut self) {
        loop {
            let cur_cpu = &mut self.cpus[0];
            if cur_cpu.is_halt() {
                cur_cpu.halt();
               break; 
            }
            
            let mem_off = cur_cpu.registers.ip.0;
            let ins_code = self.memory.get(mem_off);
            let ins = Machine::decode(ins_code);
            match ins {
                Instructions::Halt(_) => {
                    cur_cpu.set_flag(FLAG_H);
                }
                _ => {}
            }
        }
    }
    
    fn debug_dump_state(&self) {
        self.cpus.iter().for_each(|cpu| cpu.dump());
        self.memory.dump();
    }

    fn decode(ins_code: Word) -> Instructions {
        println!("ins: {:04X}", ins_code);
        // for now just halt
        HLT
    }
}

struct Instruction {
    /// human-readable name
    name: &'static str,
    /// human-readable (short) description
    desc: &'static str,
    /// byte code
    code: u8,
}

const LRM: Instruction = Instruction {
    name: "lrm",
    desc: "load a register from a memory location",
    code: 0x01,
};

const LRA: Instruction = Instruction {
    name: "lra",
    desc: "load a register with an absolute (integer) value",
    code: 0x02,
};

const SRM: Instruction = Instruction {
    name: "srm",
    desc: "store a register value to a memory location",
    code: 0x03,
};

const MVR: Instruction = Instruction {
    name: "mvr",
    desc: "move a value from one register to another",
    code: 0x04,
};

const ADD: Instruction = Instruction {
    name: "add",
    desc: "add values in two registers together",
    code: 0x05,
};

const SUB: Instruction = Instruction {
    name: "sub",
    desc: "subtract values in two registers together",
    code: 0x06,
};

const MUL: Instruction = Instruction {
    name: "sub",
    desc: "multiply values in two registers together",
    code: 0x07,
};

const DIV: Instruction = Instruction {
    name: "div",
    desc: "divide values in two registers together",
    code: 0x08,
};

const CMP: Instruction = Instruction {
    name: "cmp",
    desc: "compare values in two registers together, setting 'E' flag to 1 in FLAGS if the values are equal",
    code: 0x09,
};

const JMP: Instruction = Instruction {
    name: "jmp",
    desc: "unconditional jump to memory address",
    code: 0x0A,
};

const JE: Instruction = Instruction {
    name: "je",
    desc: "jump to memory address if 'E' flag in FLAGS is 1",
    code: 0x0B,
};

const JNE: Instruction = Instruction {
    name: "jne",
    desc: "jump to memory address if 'E' flag in FLAGS is 0",
    code: 0x0C,
};

const SYS: Instruction = Instruction {
    name: "sys",
    desc: "make system call",
    code: 0x0D,
};

const HLT: Instructions = Instructions::Halt(Instruction {
    name: "hlt",
    desc: "halt execution",
    code: 0x0E,
});

enum Instructions {
    LoadReg(Instruction),
    LoadRegAbsolute(Instruction),
    StoreReg(Instruction),
    MovReg(Instruction),
    Add(Instruction),
    Sub(Instruction),
    Mul(Instruction),
    Div(Instruction),
    Cmp(Instruction),
    Jmp(Instruction),
    Je(Instruction),
    Jne(Instruction),
    Sys(Instruction),
    Halt(Instruction),
}
