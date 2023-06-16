#![no_std]

use core::{
    convert::Infallible,
    ptr::{read_volatile, write_volatile},
};

#[repr(usize)]
enum Reg {
    Data0 = 0xE00000F4,
    Data1 = 0xE00000F8,
}

pub struct Debugger;

impl Debugger {
    pub unsafe fn steal() -> Debugger {
        let mut this = Debugger;

        // Clear out the sending flag
        this.set(Reg::Data1, 0x00);
        this.set(Reg::Data0, 0x80);

        this
    }

    /// Blocks until a compatible debugger attaches itself and is ready to
    /// receive data
    pub fn wait_for_debugger(&mut self) {
        while self.get(Reg::Data0) & 0x80 != 0 {}
    }

    fn get(&self, reg: Reg) -> u32 {
        unsafe { read_volatile(reg as usize as _) }
    }

    fn set(&mut self, reg: Reg, val: u32) {
        unsafe { write_volatile(reg as usize as _, val) }
    }
}

impl embedded_hal::serial::ErrorType for Debugger {
    type Error = Infallible;
}

impl embedded_hal::serial::Write for Debugger {
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        for chunk in buf.chunks(7) {
            // TODO: Timeout
            self.wait_for_debugger();

            let status: u8 = 0x80 | (chunk.len() as u8 + 4);

            // Use data1 if data0 overflowed
            if chunk.len() > 3 {
                self.set(
                    Reg::Data1,
                    u32::from_le_bytes([
                        *chunk.get(3).unwrap_or(&0),
                        *chunk.get(4).unwrap_or(&0),
                        *chunk.get(5).unwrap_or(&0),
                        *chunk.get(6).unwrap_or(&0),
                    ]),
                );
            }

            self.set(
                Reg::Data0,
                u32::from_le_bytes([
                    status,
                    *chunk.get(0).unwrap_or(&0),
                    *chunk.get(1).unwrap_or(&0),
                    *chunk.get(2).unwrap_or(&0),
                ]),
            );
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wait_for_debugger();
        Ok(())
    }
}

impl core::fmt::Write for Debugger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Infallible
        embedded_hal::serial::Write::write(self, s.as_bytes()).unwrap();
        Ok(())
    }
}
