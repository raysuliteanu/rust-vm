use std::{fs::File, io::Read, path::PathBuf};

fn main() -> std::io::Result<()> {
    println!("bootstrapping");
    let machine = &mut Machine::start();

    let file_path = PathBuf::from("prog.txt");
    let mut file = File::open(file_path)?;
    machine.load_program(&mut file)?;
    machine.debug_dump_state();
    Ok(())
}

// type Word = u16;

#[derive(Debug)]
enum Register {
    /// instruction pointer
    IP,
    /// stack pointer
    SP,
    /// base pointer
    BP,
    /// accumulator
    AC,
    /// general purpose registers
    GP1,
    GP2,
    GP3,
    GP4,
    /// Flags
    Flags,
}

struct Cpu {
    registers: [Register; 9],
}

impl Cpu {
    fn new() -> Self {
        Self {
            registers: [
                Register::IP,
                Register::SP,
                Register::BP,
                Register::AC,
                Register::GP1,
                Register::GP2,
                Register::GP3,
                Register::GP4,
                Register::Flags,
            ],
        }
    }

    fn dump(&self) {
        self.dump_regs();
    }

    fn dump_regs(&self) {
        println!("CPU Registers");
        for r in &self.registers {
            println!("{:?}", r);
        }
    }
}

struct Memory {
    mem: [u8; 4096],
}

impl Memory {
    fn new() -> Self {
        Memory { mem: [0u8; 4096] }
    }

    fn load(&mut self, file: &mut File) -> std::io::Result<usize> {
        let read = file.read(&mut self.mem)?;
        assert!(read <= 4096);
        Ok(read)
    }

    fn dump(&self) {
        self.dump_stats();
        self.dump_memory();
    }

    fn dump_stats(&self) {
        println!("Memory Statistics");
        println!("size: {}\navailable: {}", self.mem.len(), -1); // need to keep track of avail mem
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
            println!("");
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
            cpus: [Cpu::new()],
            memory: Memory::new(),
        }
    }

    fn load_program(&mut self, file: &mut File) -> std::io::Result<()> {
        println!("loading progra {:?}", file);
        let _ = self.memory.load(file)?;
        Ok(())
    }

    fn debug_dump_state(&self) {
        self.cpus.iter().for_each(|cpu| cpu.dump());
        self.memory.dump();
    }
}
