// Advent of Code Day TODAY
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use grid::*;
use std::io::Error;
// use std::process;

#[derive(Debug, Copy, Clone)]
pub struct DataStruct {
    x: i32,
    y: i32,
    z: i32,
}

impl DataStruct {
    pub fn new() -> Result<DataStruct, &'static str> {
        let x = 0;
        let y = 0;
        let z = 0;
        Ok(DataStruct { x, y, z })
    }
    pub fn export(self) -> Result<DataStruct, &'static str> {
        // return Ok(Coordinates{self.x,self.y,self.z});
        println!("export method called returning struct {:?}", self);
        Ok(self)
    }
}

pub fn day9_challenge1(config: &Config) -> Result<i128, Error> {
    Ok(0)
}

pub fn day9_challenge2(config: &Config) -> Result<i128, Error> {
    Ok(0)
}
