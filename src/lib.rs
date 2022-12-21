// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

use std::ffi::{c_void, CString};

use wokwi_chip_ll::{
    bufferWrite, debugPrint, framebufferInit, timerInit, timerStart, BufferId, TimerConfig,
};

const MS: u32 = 1000; // micros

// Colors:
const DEEP_GREEN: u32 = 0xff003000; // ARGB
const PURPLE: u32 = 0xffff00ff; // ARGB

struct Chip {
    frame_buffer: BufferId,
    width: u32,
    height: u32,
    current_row: u32,
}

fn draw_line(chip: &Chip, row: u32, color: u32) {
    let color_bytes_ptr = &color as *const u32 as *const u8;

    let offset = chip.width * 4 * row;
    for x in (0..chip.width * 4).step_by(4) {
        unsafe {
            bufferWrite(chip.frame_buffer, offset + x, color_bytes_ptr, 4);
        }
    }
}

pub unsafe fn on_timer_fired(user_data: *const c_void) {
    let chip = &mut *(user_data as *mut Chip);

    if chip.current_row == 0 {
        debugPrint(
            CString::new("First row!")
                .unwrap()
                .into_raw(),
        );
    }

    draw_line(chip, chip.current_row, DEEP_GREEN);

    chip.current_row = (chip.current_row + 1) % chip.height;
    draw_line(chip, chip.current_row, PURPLE);
}

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    debugPrint(
        CString::new("Hello from Framebuffer Chip!")
            .unwrap()
            .into_raw(),
    );

    let mut width = 0;
    let mut height = 0;
    let frame_buffer = framebufferInit(&mut width, &mut height);

    let chip = Chip {
        frame_buffer: frame_buffer,
        width: width,
        height: height,
        current_row: 0,
    };

    let timer_config = TimerConfig {
        user_data: &chip as *const _ as *const c_void,
        callback: on_timer_fired as *const c_void,
    };

    let timer = timerInit(&timer_config);
    timerStart(timer, 10 * MS, true);
}
