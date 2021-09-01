#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// `panic_handler` 属性定义了在发生 panic 时编译器应该调用的函数。
/// 标准库提供了自己的 panic_handler 函数，
/// 但在 `no_std` 环境中，我们需要自己定义它。
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// `no_mangle` 禁用名称改写
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 入口函数，链接器默认寻找的函数
    loop {}
}
