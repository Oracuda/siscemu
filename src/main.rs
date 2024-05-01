use std::error::Error;

enum CPUState {
    Running,
    Halted,
}

struct CPU {
    instruction: u32,
    pc: u64,
    memory: Vec<u8>,
    registers: Vec<Vec<u8>>,
    state: CPUState,
}

trait Process<T, E> {
    fn step(&mut self) -> Result<T, E>;

    fn run_until_hault(&mut self) -> Result<T, E>;
}

const INSTRUCTION_NOOP: u8 = 0x000;
const INSTRUCTION_HALT: u8 = 0x001;
const INSTRUCTION_MOVE: u8 = 0x002;
const INSTRUCTION_LOAD: u8 = 0x003;
const INSTRUCTION_JUMP: u8 = 0x004;

impl Process<(), Box<dyn Error>> for CPU {
    fn step(&mut self) -> Result<(), Box<dyn Error>> {
        if self.pc as usize * 4 == self.memory.len() {
            return Err("reached end of cpu memory")?;
        }
        let instruction_bytes =
            <[u8; 4]>::try_from(&self.memory[self.pc as usize * 4..self.pc as usize * 4 + 4])
                .unwrap();
        self.instruction = u32::from_le_bytes(instruction_bytes);

        let opcode = instruction_bytes[3];
        let instr_mask = 0xFF;
        let mut instruction = 0x00;
        if (opcode & instr_mask) == (INSTRUCTION_NOOP & instr_mask) {
            instruction = INSTRUCTION_NOOP;
        } else if (opcode & instr_mask) == (INSTRUCTION_HALT & instr_mask) {
            instruction = INSTRUCTION_HALT;
        } else if (opcode & instr_mask) == (INSTRUCTION_MOVE & instr_mask) {
            instruction = INSTRUCTION_MOVE;
        } else if (opcode & instr_mask) == (INSTRUCTION_LOAD & instr_mask) {
            instruction = INSTRUCTION_LOAD;
        } else if (opcode & instr_mask) == (INSTRUCTION_JUMP & instr_mask) {
            instruction = INSTRUCTION_JUMP;
        };

        match instruction {
            INSTRUCTION_NOOP => {
                println!("NOOP @ ADDR {}", self.pc);
            }
            INSTRUCTION_HALT => {
                println!("HALT");
                self.state = CPUState::Halted;
                return Ok(());
            }
            INSTRUCTION_MOVE => {
                println!("MOVE")
            }
            INSTRUCTION_LOAD => {}
            INSTRUCTION_JUMP => {
                let to =
                    u64::from_le_bytes(<[u8; 8]>::try_from(&self.registers[0][0..8]).unwrap()) * 4;
                println!("JUMP @ ADDR {} TO ADDR {}", self.pc, to);
                self.pc = to;
                return Ok(());
            }
            _ => {}
        }

        self.pc += 1;
        Ok(())
    }

    fn run_until_hault(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.state {
                CPUState::Running => {
                    let err = self.step();
                    if err.is_err() {
                        return err;
                    }
                }
                CPUState::Halted => {
                    return Ok(());
                }
            };
        }
    }
}

impl CPU {
    fn run_step(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.state {
                CPUState::Running => {
                    let err = self.step();
                    if err.is_err() {
                        return err;
                    }
                }
                CPUState::Halted => {
                    return Ok(());
                }
            };
            let mut empty = String::new();
            let _ = std::io::stdin().read_line(&mut empty);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU {
        instruction: 0,
        pc: 0,
        memory: vec![0; 1024],
        registers: vec![vec![0; 8]; 4],
        state: CPUState::Running,
    };
    let len = cpu.memory.len();
    cpu.memory[len - 1] = 0x01; // INSERT HALT AT END OF MEMORY
    cpu.memory[7] = 0x004;
    //cpu.run_until_hault()?;
    cpu.run_step()?;
    Ok(())
}
