use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::registers::Registers;
use crate::serial::{Serial, SERIAL_ADDRESS};
use anyhow::Result;

pub struct Cpu{
    pub pc : u32,
    pub registers : Registers,
    memory : Memory,
    serial : Serial,
    nop_count : u8,
}

pub enum Status{
    Processing,
    Finished,
}

impl Cpu{
    pub fn new() -> Self{
        println!("cpu new");
        Cpu{
            pc : 0,
            memory : Memory::new(),
            registers : Registers::new(),
            serial : Serial::new(),
            nop_count : 0,
        }
    }

    pub fn init_memory(&mut self, data: Vec<u8>){
        self.memory.initialize(data);
    }

    pub fn fetch(&self) -> u32{
        self.memory.read(self.pc)
    }

    pub fn run(&mut self) -> Result<Status>{
        let raw_inst = self.fetch();

        let inst = Instruction::decode(raw_inst)?;

        if let Instruction::Addi{
            rd : 0,
            rs1 : 0,
            imm : 0,
        } = inst
        {
            self.nop_count += 1;
        }else {
            self.nop_count = 0;
        }

        if self.nop_count >= 5{
            return Ok(Status::Finished);
        }
        self.execute(inst)?;

        Ok(Status::Processing)
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<()> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => {
                let value = self
                    .registers
                    .read(rs1)
                    .wrapping_add(self.registers.read(rs2));
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let value = self
                    .registers
                    .read(rs1)
                    .wrapping_sub(self.registers.read(rs2));
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let value = self.registers.read(rs1) | self.registers.read(rs2);
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::And { rd, rs1, rs2 } => {
                let value = self.registers.read(rs1) & self.registers.read(rs2);
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Addi { rd, rs1, imm } => {
                // imm は 12bit の signed int なので 12bit 目が 0 なら正、1なら負
                let num = match (imm & 0x80) == 0 {
                    true => imm,
                    false => 0xfffff000 | imm, // 13bit目以降を 1 で埋める
                };
                let value = self.registers.read(rs1).wrapping_add(num);
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Slli { rd, rs1, imm } => {
                let value = self.registers.read(rs1) << (imm & 0b11111);
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Beq { rs1, rs2, imm } => {
                // imm は 13bit の signed int なので 13bit 目が 0 なら正、1なら負
                let num = match (imm & 0xc0) == 0 {
                    true => imm,
                    false => 0xfffff000 | imm, // 13bit目以降を 1 で埋める
                };

                if self.registers.read(rs1) == self.registers.read(rs2) {
                    self.pc = self.pc.wrapping_add(num);
                } else {
                    self.pc += 4;
                }
                Ok(())
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.registers.read(rs1) + imm;
                let value = if addr == SERIAL_ADDRESS {
                    self.serial.read()?
                } else {
                    self.memory.read(addr)
                };
                self.registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.registers.read(rs1) + imm;
                let value = self.registers.read(rs2);
                if addr == SERIAL_ADDRESS {
                    //self.serial.write(value)?;
                } else {
                    self.memory.write(addr, value);
                }
                self.pc += 4;
                Ok(())
            }
        }
    }


}
