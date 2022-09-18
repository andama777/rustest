#[derive(Debug)]
pub struct Registers{
    data : [u32; 32],
}

const REGISTERS_COUNT : u32 = 32;

impl Registers{
    pub fn new() -> Self{
        println!("regs new");
        let data = [0; REGISTERS_COUNT as usize];
        Registers { data }
    }

    pub fn read(&self, addr : u32) -> u32{
        assert!(addr < 32);

        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u32, value: u32){
        assert!(addr < 32);

        if addr != 0{
            self.data[addr as usize] = value;
        }
    }
}