#![no_std] // 標準ライブラリを使わないよ, ということを宣言する
#![no_main] // main関数は使わないよ, ということを宣言する

#[no_mangle] // p.56 より, これをつけることで、コンパイラが関数名を変更しないようにする

fn efi_main() {
    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
