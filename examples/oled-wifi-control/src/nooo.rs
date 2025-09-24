// file: nooo.rs

// All animation frames
const FRAME_0: &[u8] = include_bytes!("../include/nooo/frame-0.bmp");
const FRAME_1: &[u8] = include_bytes!("../include/nooo/frame-1.bmp");
const FRAME_2: &[u8] = include_bytes!("../include/nooo/frame-2.bmp");
const FRAME_3: &[u8] = include_bytes!("../include/nooo/frame-3.bmp");
const FRAME_4: &[u8] = include_bytes!("../include/nooo/frame-4.bmp");
const FRAME_5: &[u8] = include_bytes!("../include/nooo/frame-5.bmp");
const FRAME_6: &[u8] = include_bytes!("../include/nooo/frame-6.bmp");
const FRAME_7: &[u8] = include_bytes!("../include/nooo/frame-7.bmp");
const FRAME_8: &[u8] = include_bytes!("../include/nooo/frame-8.bmp");
const FRAME_9: &[u8] = include_bytes!("../include/nooo/frame-9.bmp");
const FRAME_10: &[u8] = include_bytes!("../include/nooo/frame-10.bmp");
const FRAME_11: &[u8] = include_bytes!("../include/nooo/frame-11.bmp");
const FRAME_12: &[u8] = include_bytes!("../include/nooo/frame-12.bmp");
const FRAME_13: &[u8] = include_bytes!("../include/nooo/frame-13.bmp");
const FRAME_14: &[u8] = include_bytes!("../include/nooo/frame-14.bmp");

// Public array of all frames - count is automatically derived
pub const FRAMES: &[&[u8]] = &[
    FRAME_0, FRAME_1, FRAME_2, FRAME_3, FRAME_4,
    FRAME_5, FRAME_6, FRAME_7, FRAME_8, FRAME_9,
    FRAME_10, FRAME_11, FRAME_12, FRAME_13, FRAME_14
];

// Helper function to get frame count
pub const fn frame_count() -> usize {
    FRAMES.len()
}