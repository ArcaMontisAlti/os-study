#![no_std]
#![no_main]
#![feature(offset_of)]
use core::arch::asm;
use core::fmt::Write; 
use core::panic::PanicInfo;
use core::writeln;
// ここから wasabiモジュールのうち, 画面描画関連のものをインポート
use wasabi::graphics::draw_test_pattern;
use wasabi::graphics::fill_rect;
use wasabi::graphics::Bitmap;
// ここまで 

// ここから wasabiモジュールのうち, UEFI関連のものをインポート
use wasabi::uefi::exit_from_efi_boot_services;
use wasabi::uefi::init_vram;
use wasabi::uefi::EfiHandle;
use wasabi::uefi::EfiMemoryType;
use wasabi::uefi::EfiSystemTable;
use wasabi::uefi::MemoryMapHolder;
use wasabi::uefi::VramTextWriter;
// ここまで

pub fn hlt() {
    unsafe { asm!("hlt") }
}

#[no_mangle] // p.56 より, これをつけることで、コンパイラが関数名を変更しないようにする
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000, 0, 0, vw, vh).expect("fill_rect failed");
    draw_test_pattern(&mut vram);
    let mut w = VramTextWriter::new(&mut vram);
    for i in 0..4 {
        writeln!(w, "i ={i}").unwrap();
    }
    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services()
        .get_memory_map(&mut memory_map);
    writeln!(w, "{status:?}").unwrap();
    let mut total_memory_pages = 0; // 通常のDRAMとして使える領域をカウントする
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue; // 通常のDRAMとして使える領域でなければスキップ
        }
        total_memory_pages += e.number_of_pages(); // ページ数を加算
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_size_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_size_mib} MiB"
    )
    .unwrap();
    exit_from_efi_boot_services(
        image_handle,
        efi_system_table,
        &mut memory_map,
    );
    writeln!(w, "Hello, Non-UEFI world!").unwrap();
    loop {
        hlt() // CPU を割り込みが来るまで休ませる
    }

#[panic_handler] // panic_handler を定義することで、パニック時の挙動を定義する
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt() // パニック時も CPU を休ませる
    }
}

}