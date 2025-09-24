// Auto-generated animation: no-shake
// Generated from: include/no-shake
// Frame count: 6

const FRAME_0: &[u8] = include_bytes!("../include/no-shake/frame-0.bmp");
const FRAME_1: &[u8] = include_bytes!("../include/no-shake/frame-1.bmp");
const FRAME_2: &[u8] = include_bytes!("../include/no-shake/frame-2.bmp");
const FRAME_3: &[u8] = include_bytes!("../include/no-shake/frame-3.bmp");
const FRAME_4: &[u8] = include_bytes!("../include/no-shake/frame-4.bmp");
const FRAME_5: &[u8] = include_bytes!("../include/no-shake/frame-5.bmp");

// Public array of all frames - count is automatically derived
pub const FRAMES: &[&[u8]] = &[
    FRAME_0, FRAME_1, FRAME_2, FRAME_3, FRAME_4,
    FRAME_5
];

// Helper function to get frame count
pub const fn frame_count() -> usize {
    FRAMES.len()
}
