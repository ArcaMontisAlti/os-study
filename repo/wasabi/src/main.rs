#![no_std] // no_std を使うための宣言
#![no_main] // no_main を使うための宣言
#![feature(offset_of)]
// core クレート内にある型を使うために use 宣言を行う
use core::arch::asm;
use core::cmp::min; // 四角形を描画するために追加した
use core::mem::offset_of;
use core::mem::size_of;
use core::panic::PanicInfo;
use core::ptr::null_mut;

type EfiVoid = u8;
type EfiHandle = u64;
type Result<T> = core::result::Result<T, &'static str>;

// EfiGuid の実装部分
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct EfiGuid {
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3: [u8; 8],
}

const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data0: 0x9042a9de,
    data1: 0x23dc,
    data2: 0x4a38,
    data3: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[must_use]
#[repr(u64)]
enum EfiStatus {
    Success = 0,
}

#[repr(C)]
struct EfiBootServicesTable {
    _reserved0: [u64; 40],
    locate_protocol: extern "win64" fn(
        protocol: *const EfiGuid,
        registration: *const EfiVoid,
        interface: *mut *mut EfiVoid,
    ) -> EfiStatus,
}
const _: () = assert!(offset_of!(EfiBootServicesTable, locate_protocol) == 320);

// EfiGraphicsProtocol の中の mode というメンバの実装
#[repr(C)]
struct EfiSystemTable {
    _reserved0: [u64; 12],
    pub boot_services: &'static EfiBootServicesTable,
}
const _: () = assert!(offset_of!(EfiSystemTable, boot_services) == 96);

// メンバ info を辿ることで, より詳しい画面モードを取得するための構造体の実装
#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolPixelInfo {
    version: u32,
    pub horizontal_resolution: u32, // 水平方向の画素数. u32 というのは, 32bit の符号なし整数
    pub vertical_resolution: u32,   // 垂直方向の画素数
    _padding0: [u32; 5],
    pub pixels_per_scan_line: u32, // 水平方向のデータに含まれる画素数. horizontal_resolution と同じ値になるとは限らないため必要
}
const _: () = assert!(size_of::<EfiGraphicsOutputProtocolPixelInfo>() == 36);

// mode が指す先の構造体の実装
#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocolMode<'a> {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'a EfiGraphicsOutputProtocolPixelInfo,
    pub size_of_info: u64,
    pub frame_buffer_base: usize,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Debug)]
struct EfiGraphicsOutputProtocol<'a> {
    reserved: [u64; 3],
    pub mode: &'a EfiGraphicsOutputProtocolMode<'a>,
}

fn locate_graphic_protocol<'a>(
    efi_system_table: &EfiSystemTable,
) -> Result<&'a EfiGraphicsOutputProtocol<'a>> {
    let mut graphic_output_protocol = null_mut::<EfiGraphicsOutputProtocol>();
    let status = (efi_system_table.boot_services.locate_protocol)(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        null_mut::<EfiVoid>(),
        &mut graphic_output_protocol as *mut *mut EfiGraphicsOutputProtocol as *mut *mut EfiVoid,
    );
    if status != EfiStatus::Success {
        return Err("Failed to locate graphics output protocol");
    }
    Ok(unsafe { &*graphic_output_protocol })
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}

#[no_mangle] // p.56 より, これをつけることで、コンパイラが関数名を変更しないようにする
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    for y in 0..vram.height {
        for x in 0..vram.width {
            if let Some(pixel) = vram.pixel_at_mut(x, y) {
                *pixel = 0x00ff00; // 緑色で塗りつぶす
            }
        }
    }
    for y in 0..vram.height / 2 {
        for x in 0..vram.width / 2 {
            if let Some(pixel) = vram.pixel_at_mut(x, y) {
                *pixel = 0xff0000;
            }
        }
    }
    // println!("Hello, world!");
    loop {
        hlt() // CPU を割り込みが来るまで休ませる
    }
}

#[panic_handler] // panic_handler を定義することで、パニック時の挙動を定義する
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt() // パニック時も CPU を休ませる
    }
}

trait Bitmap {
    fn bytes_per_pixel(&self) -> i64; // 1ピクセルあたりのバイト数を返す i64 とは, 64bit の符号付き整数
    fn pixels_per_line(&self) -> i64; // 1行あたりのピクセル数を返す 
    fn width(&self) -> i64; // 画像の幅を返す
    fn height(&self) -> i64; // 画像の高さを返す
    fn buf_mut(&mut self) -> *mut u8;
    /// # Safety
    /// 
    /// Returned pointer is valid as long as the given coordinates are valid
    /// which means that passion is_in_ * _range tests.
    unsafe fn unchecked_pixel_at_mut(&mut self, x: i64, y: i64) -> *mut u32 {
        self.buf_mut().add(
            ((y * self.pixels_per_line() + x) * self.bytes_per_pixel())
                    as usize,
        ) as *mut u32
    }
    fn pixel_at_mut(&mut self, x: i64, y: i64) -> Option<&mut u32> {
        if self.is_in_x_range(x) && self.is_in_y_range(y) {
            // SAFETY: (x, y) is always validated by the checks above.
            unsafe { Some(&mut *(self.unchecked_pixel_at_mut(x, y))) }
        } else {
            None
        }

    }
    fn is_in_x_range(&self, px: i64) -> bool {
        0 <= px && px < min(self.width(), self.pixels_per_line())
    }
    fn is_in_y_range(&self, py: i64) -> bool {
        0 <= py && py < self.height()
    }
}

#[derive(Clone, Copy)]
struct VramBufferInfo {
    buf: *mut u8,
    width: i64,
    height: i64,
    pixels_per_line: i64,
}

impl Bitmap for VramBufferInfo {
    fn bytes_per_pixel(&self) -> i64 {
        4
    }
    fn pixels_per_line(&self) -> i64 {
        self.pixels_per_line
    }
    fn width(&self) -> i64 {
        self.width
    }
    fn height(&self) -> i64 {
        self.height
    }
    fn buf_mut(&mut self) -> *mut u8 {
        self.buf
    }
}

fn init_vram(efi_system_table: &EfiSystemTable) -> Result<VramBufferInfo> {
    let gp = locate_graphic_protocol(efi_system_table)?;
    Ok(VramBufferInfo {
        buf: gp.mode.frame_buffer_base as *mut u8,
        width: gp.mode.info.horizontal_resolution as i64,
        height: gp.mode.info.vertical_resolution as i64,
        pixels_per_line: gp.mode.info.pixels_per_scan_line as i64,
    })
}