#![no_std]
#![allow(non_camel_case_types)]

#[derive(Debug)]
pub enum Never {}

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires one of the following device features enabled:
        stm32h742
        stm32h743
        stm32h753
        stm32h750
        stm32h742v
        stm32h743v
        stm32h753v
        stm32h750v
"
);

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(any(
    feature = "stm32h742",
    feature = "stm32h743",
    feature = "stm32h750",
))]
pub use stm32h7::stm32h743 as stm32;
#[cfg(any(
    feature = "stm32h753",
))]
pub use stm32h7::stm32h753 as stm32;
#[cfg(any(
    feature = "stm32h742v",
    feature = "stm32h743v",
    feature = "stm32h750v",
))]
pub use stm32h7::stm32h743v as stm32;
#[cfg(any(
    feature = "stm32h753v",
))]
pub use stm32h7::stm32h753v as stm32;


#[cfg(feature = "device-selected")]
pub use crate::stm32 as pac;
#[cfg(feature = "device-selected")]
pub use crate::stm32 as device;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::stm32::interrupt;

#[cfg(feature = "device-selected")]
pub mod delay;
#[cfg(feature = "device-selected")]
pub mod flash;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod pwm;
#[cfg(feature = "device-selected")]
pub mod pwr;
#[cfg(feature = "device-selected")]
pub mod qei;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
pub mod rng;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod signature;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timer;
#[cfg(feature = "device-selected")]
pub mod watchdog;
#[cfg(feature = "device-selected")]
pub mod adc;
