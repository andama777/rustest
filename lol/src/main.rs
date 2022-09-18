use anyhow::{Context, Result};
use lol::simulator::Simulator;

fn main(){
    let mut sim = Simulator::new();
    sim.start()
}