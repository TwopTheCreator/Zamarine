#![no_std]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

#[macro_use]
extern crate alloc;

use core::panic::PanicInfo;
use alloc::boxed::Box;
use log::{error, info, warn};

pub mod process;
pub mod syscalls;
pub mod interrupts;
pub mod time;
pub mod sync;

#[global_allocator]
static ALLOCATOR: spin::Mutex<linked_list_allocator::LockedHeap> = 
    spin::Mutex::new(linked_list_allocator::LockedHeap::empty());

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("Kernel panic: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    init_heap();
    init_logger();
    
    info!("Kernel initialized successfully");
    
    loop {
 
    }
}

fn init_heap() {
    const HEAP_SIZE: usize = 0x100000;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    
    unsafe {
        let heap_start = HEAP.as_ptr() as usize;
        let heap_end = heap_start + HEAP_SIZE;
        ALLOCATOR.lock().init(heap_start, heap_end - heap_start);
    }
}

fn init_logger() {
    use log::LevelFilter;
    use log::Level;
    
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("kernel", LevelFilter::Trace)
        .init()
        .unwrap();
}
