#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!("./bindings.rs");

use std::ffi::CString;
use std::os::raw::c_char;

use anyhow::{anyhow, Result};

pub fn setup(platform: &str) -> Result<()> {
    let c_platform = CString::new(platform).unwrap();
    let result = unsafe {
        let platform_ptr = c_platform.into_raw() as *mut c_char;
        let res = wiringXSetup(platform_ptr, None);
        let _ = CString::from_raw(platform_ptr);
        res
    };
    if result < 0 {
        Err(anyhow!("Setup failed"))
    } else {
        Ok(())
    }
}

pub fn pin_mode(gpio: i32, mode: u32) {
    unsafe {
        pinMode(gpio, mode);
    }
}

pub fn digital_write(gpio: i32, value: u32) {
    unsafe {
        digitalWrite(gpio, value);
    }
}

pub fn digital_read(gpio: i32) -> i32 {
    unsafe { digitalRead(gpio) }
}

pub fn valid_gpio(gpio: i32) -> bool {
    unsafe { wiringXValidGPIO(gpio) == 0 }
}

pub fn gc() {
    unsafe {
        wiringXGC();
    }
}

pub fn platform() -> String {
    unsafe {
        let c_str = wiringXPlatform();
        std::ffi::CStr::from_ptr(c_str)
            .to_string_lossy()
            .into_owned()
    }
}

pub fn setup_i2c(path: &str, device: i32) -> i32 {
    let c_path = CString::new(path).unwrap();
    unsafe { wiringXI2CSetup(c_path.as_ptr(), device) }
}

pub fn i2c_read(fd: i32) -> i32 {
    unsafe { wiringXI2CRead(fd) }
}

pub fn i2c_read_reg8(fd: i32, reg: i32) -> i32 {
    unsafe { wiringXI2CReadReg8(fd, reg) }
}

pub fn i2c_read_reg16(fd: i32, reg: i32) -> i32 {
    unsafe { wiringXI2CReadReg16(fd, reg) }
}

pub fn i2c_write(fd: i32, data: i32) -> bool {
    unsafe { wiringXI2CWrite(fd, data) == 0 }
}

pub fn i2c_write_reg8(fd: i32, reg: i32, data: i32) -> bool {
    unsafe { wiringXI2CWriteReg8(fd, reg, data) == 0 }
}

pub fn i2c_write_reg16(fd: i32, reg: i32, data: i32) -> bool {
    unsafe { wiringXI2CWriteReg16(fd, reg, data) == 0 }
}

pub fn spi_get_fd(channel: i32) -> i32 {
    unsafe { wiringXSPIGetFd(channel) }
}

pub fn spi_data_rw(channel: i32, data: &mut [u8]) -> Result<Vec<u8>> {
    let len = data.len() as i32;
    let result = unsafe { wiringXSPIDataRW(channel, data.as_mut_ptr(), len) };
    if result < 0 {
        Err(anyhow!("SPI Data RW error"))
    } else {
        Ok(data.to_vec())
    }
}

pub fn setup_spi(channel: i32, speed: i32) -> i32 {
    unsafe { wiringXSPISetup(channel, speed) }
}
