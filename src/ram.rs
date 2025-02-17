use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub enum RamInstruction {
    ReadReg(usize),  // Read from given register to r0
    ReadPtr(usize),  // Read from given memory address to r0

    StoreReg(usize), // Store r0 to given register
    StorePtr(usize), // Store r0 to given memory address
    
    LoadReg(usize),  // Load from given register to r0
    LoadPtr(usize),  // Load from given memory address to r0
    LoadConst(i64),  // Load given constant to r0
    
    AddReg(usize),   // Add given register to r0
    AddPtr(usize),   // Add given memory address to r0
    AddConst(i64),   // Add given constant to r0
    
    SubReg(usize),   // Subtract given register from r0
    SubPtr(usize),   // Subtract given memory address from r0
    SubConst(i64),   // Subtract given constant from r0
    
    Half,            // Divide r0 by 2
    Jump(i64),       // Set program counter to given address
    Jpos(i64),       // Set program counter to given address if r0 is positive
    Jzero(i64),      // Set program counter to given address if r0 is zero
    Jneg(i64),       // Set program counter to given address if r0 is negative
    Halt,            // Stop execution
}

#[derive(Debug)]
pub struct Ram {
    program_counter: usize,
    registers: HashMap<usize, i64>,
    program: Vec<RamInstruction>,
}

impl Ram {
    pub fn new(code: &str) -> Self {
        let mut ram = Ram {
            program_counter: 0,
            registers: HashMap::new(),
            program: Vec::new(),
        };
        ram.compile(code);
        ram
    }

    fn compile(&mut self, code: &str) {
        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            println!("{}. {}", self.program.len() + 1, line);
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let arg = match parts.next() {
                Some(arg) => arg,
                None => "",
            };
            match instruction {
                "READ" => {
                    if arg.starts_with("^") {
                        self.program.push(RamInstruction::ReadPtr(arg[1..].parse().unwrap()));
                    } else {
                        self.program.push(RamInstruction::ReadReg(arg.parse().unwrap()));
                    }
                },
                "STORE" => {
                    if arg.starts_with("^") {
                        self.program.push(RamInstruction::StorePtr(arg[1..].parse().unwrap()));
                    } else {
                        self.program.push(RamInstruction::StoreReg(arg.parse().unwrap()));
                    }
                },
                "LOAD" => {
                    if arg.starts_with("^") {
                        self.program.push(RamInstruction::LoadPtr(arg[1..].parse().unwrap()));
                    } else if arg.starts_with("=") {
                        self.program.push(RamInstruction::LoadConst(arg[1..].parse().unwrap()));
                    } else {
                        self.program.push(RamInstruction::LoadReg(arg.parse().unwrap()));
                    }
                },
                "ADD" => {
                    if arg.starts_with("^") {
                        self.program.push(RamInstruction::AddPtr(arg[1..].parse().unwrap()));
                    } else if arg.starts_with("=") {
                        self.program.push(RamInstruction::AddConst(arg[1..].parse().unwrap()));
                    } else {
                        self.program.push(RamInstruction::AddReg(arg.parse().unwrap()));
                    }
                },
                "SUB" => {
                    if arg.starts_with("^") {
                        self.program.push(RamInstruction::SubPtr(arg[1..].parse().unwrap()));
                    } else if arg.starts_with("=") {
                        self.program.push(RamInstruction::SubConst(arg[1..].parse().unwrap()));
                    } else {
                        self.program.push(RamInstruction::SubReg(arg.parse().unwrap()));
                    }
                },
                "HALF" => {
                    self.program.push(RamInstruction::Half);
                },
                "JUMP" => {
                    if arg.starts_with("=") {
                        self.program.push(RamInstruction::Jump(arg[1..].parse().unwrap()));
                    } else {
                        panic!("Argument for JUMP needs to be a constant, e.g. JUMP =5 (invalid input given: JUMP {})", arg);
                    }
                },
                "JPOS" => {
                    if arg.starts_with("=") {
                        self.program.push(RamInstruction::Jpos(arg[1..].parse().unwrap()));
                    } else {
                        panic!("Argument for JPOS needs to be a constant, e.g. JPOS =5 (invalid input given: JPOS {})", arg);
                    }
                },
                "JZERO" => {
                    if arg.starts_with("=") {
                        self.program.push(RamInstruction::Jzero(arg[1..].parse().unwrap()));
                    } else {
                        panic!("Argument for JZERO needs to be a constant, e.g. JZERO =5 (invalid input given: JZERO {})", arg);
                    }
                },
                "JNEG" => {
                    if arg.starts_with("=") {
                        self.program.push(RamInstruction::Jneg(arg[1..].parse().unwrap()));
                    } else {
                        panic!("Argument for JNEG needs to be a constant, e.g. JNEG =5 (invalid input given: JNEG {})", arg);
                    }
                },
                "HALT" => {
                    self.program.push(RamInstruction::Halt);
                },
                _ => panic!("Invalid instruction: {}", instruction),
            }
        }
    }

    pub fn execute(&mut self, input: &Vec<i64>) -> i64 {
        self.program_counter = 0;
        loop {
            let instruction = &self.program[self.program_counter];
            match instruction {
                RamInstruction::ReadReg(reg) => {
                    let value = input[*reg as usize - 1];
                    self.registers.insert(0, value);
                },
                RamInstruction::ReadPtr(ptr) => {
                    let reg = *self.registers.get(ptr).unwrap_or(&0);
                    let value = input[reg as usize - 1];
                    self.registers.insert(0, value);
                },
                
                RamInstruction::StoreReg(reg) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(*reg, r0);
                },
                RamInstruction::StorePtr(ptr) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    let reg = *self.registers.get(ptr).unwrap_or(&0) as usize;
                    self.registers.insert(reg, r0);
                },
                
                RamInstruction::LoadReg(reg) => {
                    let value = *self.registers.get(reg).unwrap_or(&0);
                    self.registers.insert(0, value);
                },
                RamInstruction::LoadPtr(ptr) => {
                    let reg = *self.registers.get(ptr).unwrap_or(&0) as usize;
                    let value = *self.registers.get(&reg).unwrap_or(&0);
                    self.registers.insert(0, value);
                },
                RamInstruction::LoadConst(value) => {
                    self.registers.insert(0, *value);
                },

                RamInstruction::AddReg(reg) => {
                    let value = *self.registers.get(reg).unwrap_or(&0);
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 + value);
                },
                RamInstruction::AddPtr(ptr) => {
                    let reg = *self.registers.get(ptr).unwrap_or(&0) as usize;
                    let value = *self.registers.get(&reg).unwrap_or(&0);
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 + value);
                },
                RamInstruction::AddConst(value) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 + value);
                },

                RamInstruction::SubReg(reg) => {
                    let value = *self.registers.get(reg).unwrap_or(&0);
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 - value);
                },
                RamInstruction::SubPtr(ptr) => {
                    let reg = *self.registers.get(ptr).unwrap_or(&0) as usize;
                    let value = *self.registers.get(&reg).unwrap_or(&0);
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 - value);
                },
                RamInstruction::SubConst(value) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 - value);
                },

                RamInstruction::Half => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    self.registers.insert(0, r0 / 2);
                },

                RamInstruction::Jump(addr) => {
                    self.program_counter = *addr as usize - 1;
                    continue;
                },
                RamInstruction::Jpos(addr) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    if r0 > 0 {
                        self.program_counter = *addr as usize - 1;
                        continue;
                    }
                },
                RamInstruction::Jzero(addr) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    if r0 == 0 {
                        self.program_counter = *addr as usize - 1;
                        continue;
                    }
                },
                RamInstruction::Jneg(addr) => {
                    let r0 = *self.registers.get(&0).unwrap_or(&0);
                    if r0 < 0 {
                        self.program_counter = *addr as usize - 1;
                        continue;
                    }
                },

                RamInstruction::Halt => {
                    break;
                },
            }
            println!("({}, {:?})", self.program_counter + 1, self.registers);
            self.program_counter += 1;
        }
        println!("({}, {:?})", self.program_counter + 1, self.registers);
        *self.registers.get(&0).unwrap_or(&0)
    }
}