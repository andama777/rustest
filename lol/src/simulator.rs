use crate::cpu::{Cpu, Status};
//use anyhow::Result;
pub struct Simulator{
    cpu : Cpu,
}

impl Simulator{

    pub fn new() -> Simulator{
        println!("sim new");
        Self { cpu: Cpu::new() }
    }

    pub fn start(&self){
        println!("sim start");

        //self.cpu.init_memory
        /*
        loop{
            match self.cpu.run(){
                Ok(status) => match status{
                    Status::Processiong => {}
                    Status::Finished => {
                        break;
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(())
        */
    }
}