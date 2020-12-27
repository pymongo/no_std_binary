#![no_std]
#![no_main]
#![feature(lang_items,asm)]

// entry_point/start_address of process, since the linker looks for a function named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // mac: illegal hardware instruction
    exit(0);
    // loop {}
}

pub fn exit(code: isize) -> ! {
    let syscall_number: usize = 60;
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") code,
            options(noreturn)
        );
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
