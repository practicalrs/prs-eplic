use crate::{interp_err::InterpErr, Result};
use std::io::{stdin, Read};

pub struct Tp {
    mem: Vec<u32>,
    ptr: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TpDbg {
    mem: u32,
    ptr: usize,
}

impl Tp {
    pub fn new() -> Self {
        Self {
            mem: vec![0; 2048],
            ptr: 1024,
        }
    }

    pub fn dbg(&self) -> TpDbg {
        let mem = self.mem[self.ptr];
        let ptr = self.ptr;

        TpDbg { mem, ptr }
    }

    pub fn dec(&mut self) -> Result<()> {
        if self.is_ptr_vld() {
            let op = self.mem[self.ptr]
                .checked_sub(1)
                .ok_or(InterpErr::SubOvflw)?;

            self.mem[self.ptr] = op;
        }

        Ok(())
    }

    pub fn dec_ptr(&mut self) -> Result<()> {
        let op = self.ptr.checked_sub(1).ok_or(InterpErr::SubOvflw)?;

        self.ptr = op;

        Ok(())
    }

    pub fn get(&self) -> u32 {
        self.mem[self.ptr]
    }

    pub fn inc(&mut self) -> Result<()> {
        if self.is_ptr_vld() {
            let op = self.mem[self.ptr]
                .checked_add(1)
                .ok_or(InterpErr::AddOvflw)?;

            self.mem[self.ptr] = op;
        }

        Ok(())
    }

    pub fn inc_ptr(&mut self) -> Result<()> {
        let op = self.ptr.checked_add(1).ok_or(InterpErr::AddOvflw)?;

        self.ptr = op;

        Ok(())
    }

    pub fn print(&self) -> Result<()> {
        if self.mem[self.ptr] <= 255 {
            let ch = self.mem[self.ptr] as u8;
            print!("{}", ch as char);
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<()> {
        let mut inpt = [0; 1];
        stdin().read_exact(&mut inpt)?;
        self.mem[self.ptr] = inpt[0] as u32;

        Ok(())
    }

    pub fn is_ptr_vld(&self) -> bool {
        if self.mem.len() > self.ptr {
            return true;
        }

        false
    }
}
