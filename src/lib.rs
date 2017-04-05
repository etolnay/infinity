//! The Infinity OS Kernel is a µkernel with focus on performance, multi-core system, security, and
//! no legacy components.

#![feature(alloc, collections)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![feature(heap_api)]
#![no_std]

/// Architecture specific items (x86_64)
#[cfg(all(not(test), target_arch = "x86_64"))]
#[macro_use]
extern crate arch_x86_64 as arch;

extern crate alloc;
#[macro_use]
extern crate collections;
extern crate spin;

use arch::memory::MemoryController;
use spin::Mutex;

#[macro_use]
pub mod common;

pub mod context;

static MEMORY_CONTROLLER: Mutex<Option<&'static mut MemoryController>> = Mutex::new(None);

pub extern fn userspace_init() {
    println!("Hello from a context!");
    loop {}
}

/// This is the kernel entry point for the primary CPU. The arch crate is responsible for calling
/// this.
#[no_mangle]
pub extern fn kmain(memory_controller: &'static mut MemoryController) -> ! {
    // save the memory controller
    *MEMORY_CONTROLLER.lock() = Some(memory_controller);

    // initialize the context sub-system
    context::init();

    // Spawn a context
    match context::contexts_mut().spawn(userspace_init) {
        Ok(context_lock) => {
            let mut context = context_lock.write();
            context.status = context::Status::Runnable;
        },
        Err(error) => {
            panic!("failed to spawn userspace_init: {}", error);
        }
    }

    println!("On kernel!");

    loop {

    }
}
