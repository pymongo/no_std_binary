#![no_std]
#![no_main]
#![feature(lang_items,asm)]
 
/**
entry_point/start_address of process, since the linker looks for a function named `_start` by default
compile: 
> rustc -C link-arg=-nostartfiles main.rs
or add this to .cargo/config.toml
```
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]
```
*/
#[no_mangle]
extern "C" fn _start() -> ! {
    exit(0); // macOS: illegal hardware instruction
}

fn exit(code: isize) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // exit
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
