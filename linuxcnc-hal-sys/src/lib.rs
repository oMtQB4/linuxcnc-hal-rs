//! This crate uses [`bindgen`] to create bindings to the LinuxCNC HAL module.
//!
//! The high level, safe interface at [`linuxcnc-hal`] is recommended for user code.
//!
//! # Examples
//!
//! All functions exported from this crate are `unsafe`, hence each example is wrapped in a big
//! `unsafe` block for clarity.
//!
//! The LinuxCNC HAL requires a certain setup procedure to operate correctly. The basic program
//! structure should be roughly as follows
//!
//! 1. Call [`hal_init`] to create a new HAL component
//! 1. Register `SIGTERM` and `SIGINT` signals, likely with the [`signal_hook`] crate. LinuxCNC will
//! hang if these signals are not registered.
//! 1. Register pins with [`hal_pin_float_new`], [`hal_pin_u32_new`], etc
//! 1. Call [`hal_ready`] to signal to LinuxCNC that the component is ready
//! 1. Enter an infinite loop to continuously update input/output pin values and perform component
//! logic
//!
//! ## Create an input pin
//!
//! This example registers a pin that accepts a floating point value using [`hal_pin_float_new`].
//! Each HAL pin requires some memory allocated to store its value which is performed with
//! [`hal_malloc`].
//!
//! **Note that there is no error handling in this example for brevity.**
//!
//! ```rust,no_run
//! use linuxcnc_hal_sys::*;
//! use signal_hook::iterator::Signals;
//! use std::ffi::CString;
//! use std::mem;
//! use std::thread;
//! use std::time::Duration;
//!
//! unsafe {
//!     let id = hal_init(CString::new("pins").unwrap().as_ptr() as *const i8);
//!
//!     println!("ID {}", id);
//!
//!     let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT]).unwrap();
//!
//!     let storage = hal_malloc(mem::size_of::<f64>() as i64) as *mut *mut f64;
//!
//!     println!("Storage {:?}", storage);
//!
//!     let pin_name = CString::new("pins.input_1").unwrap();
//!
//!     let ret = hal_pin_float_new(
//!         pin_name.as_ptr() as *const i8,
//!         hal_pin_dir_t_HAL_IN,
//!         storage,
//!         id,
//!     );
//!
//!     println!("Pin init {}", ret);
//!
//!     let ret = hal_ready(id);
//!
//!     println!("Ready {}", ret);
//!
//!     while !signals.pending().any(|signal| match signal {
//!         signal_hook::SIGTERM | signal_hook::SIGINT | signal_hook::SIGKILL => true,
//!         _ => false,
//!     }) {
//!         println!("Input {:?}", **storage);
//!
//!         thread::sleep(Duration::from_millis(500));
//!     }
//! }
//! ```
//!
//! [`linuxcnc-hal`]: https://docs.rs/linuxcnc-hal
//! [`bindgen`]: https://docs.rs/bindgen
//! [`signal_hook`]: https://docs.rs/signal_hook

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
