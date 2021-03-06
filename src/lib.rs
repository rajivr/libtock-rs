#![feature(
    asm,
    alloc,
    alloc_error_handler,
    core_intrinsics,
    lang_items,
    naked_functions
)]
#![no_std]

extern crate alloc;

mod callback;

pub mod adc;
pub mod ble_composer;
pub mod ble_parser;
pub mod buttons;
pub mod console;
pub mod debug;
pub mod electronics;
pub mod gpio;
pub mod led;
pub mod result;
pub mod sensors;
pub mod shared_memory;
pub mod simple_ble;
pub mod temperature;
pub mod timer;
pub mod unwind_symbols;

#[cfg(target_arch = "arm")]
pub mod entry_point;

#[cfg(target_arch = "arm")]
mod lang_items;

#[cfg(target_arch = "arm")]
pub mod syscalls;

#[cfg(not(target_arch = "arm"))]
#[path = "syscalls_mock.rs"]
mod syscalls;

#[cfg(target_arch = "arm")]
#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();
