#![no_std]
#![no_main]

mod io;
mod sbi;

use core::{arch::asm, mem, panic::PanicInfo, ptr};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    static mut __bss: u64;
    static mut __bss_end: u64;
    static mut __stack_top: u64;
}

unsafe fn zero_bss(mut bss: *mut u64, bss_end: *mut u64) {
    while bss < bss_end {
        // NOTE(volatile) to prevent this from being transformed into `memclr`
        unsafe { ptr::write_volatile(bss, mem::zeroed()) };
        bss = unsafe { bss.offset(1) };
    }
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> ! {
    unsafe {
        zero_bss(&raw mut __bss, &raw mut __bss_end);

        for _ in 0..3 {
            println!();
        }
        println!("Hello world from WasOS!");

        loop {
            asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub extern "C" fn boot() {
    unsafe {
        asm!(
            "mv sp, {stack_top}",
            "j kernel_main",
            stack_top = in(reg) __stack_top
        );
    }
}
