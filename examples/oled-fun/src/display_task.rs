// file: display_task.rs
// desc: task for oled display handling

// Import defmt macros
use defmt::{info, error};

// OLED and graphics imports
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
    image::Image,
};
use tinybmp::Bmp;


use embassy_sync::pipe::{Reader};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

// Import from crate root
use crate::setup_devices::Display;
use crate::nooo::{FRAMES, frame_count};
use embassy_time::Timer;


// Helper function to display a specific frame of an animation
async fn display_frame(
    display: &mut Display, 
    current_animation_num: u8,
    frame_index: usize) {
    
    // Create text style
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    // Clear the display
    display.clear(BinaryColor::Off).unwrap();
    
    // Draw title in the top section
    let title_text = match current_animation_num {
        1 => "Animation #: 1",
        2 => "Animation #: 2", 
        3 => "Animation #: 3",
        4 => "Animation #: 4",
        _ => "Animation #: ?",
    };
    Text::new(title_text, Point::new(0, 10), text_style)
        .draw(display)
        .unwrap();
    
    // Get current frame data
    let current_frame_data = FRAMES[frame_index];
    
    // Parse current frame as BMP
    match Bmp::from_slice(current_frame_data) {
        Ok(bmp) => {
            // Draw the current frame centered
            let image = Image::new(&bmp, Point::new(40, 16)); // Centered for 48x48 image
            match image.draw(display) {
                Ok(_) => {},
                Err(_) => error!("Failed to draw frame {}", frame_index),
            }
        },
        Err(_) => {
            error!("Failed to parse frame {} BMP", frame_index);
        }
    }
    
    // Update display
    match display.flush() {
        Ok(_) => info!("Displayed frame {}/{}", frame_index + 1, frame_count()),
        Err(_) => error!("Display flush failed"),
    }
}

fn try_update_animation_num(
    current_animation_num: u8,
    pipe_reader: &Reader<'static, CriticalSectionRawMutex, 1>,
) -> u8 {
    let mut buffer = [0u8; 1];
    match pipe_reader.try_read(&mut buffer) {
        Ok(bytes_read) if bytes_read > 0 => {
            let new_animation_num = buffer[0];
            if new_animation_num >= 1 && new_animation_num <= 4 {
                info!("Animation changed to: {}", new_animation_num);
                new_animation_num
            } else {
                error!("Invalid animation number: {}", new_animation_num);
                current_animation_num
            }
        },
        _ => current_animation_num, // No data or error, keep current
    }
}

#[embassy_executor::task]
pub async fn display_task(
    mut display: Display,
    mut pipe_reader: Reader<'static, CriticalSectionRawMutex, 1>,
) {
    info!("Starting animation with {} frames", frame_count());
    let mut frame_index = 0usize;
    let mut current_animation_num: u8 = 1;
    
    loop {
        // Check for animation changes (non-blocking)
        current_animation_num = try_update_animation_num(current_animation_num, &mut pipe_reader);
        
        // Display current frame
        display_frame(&mut display, current_animation_num, frame_index).await;
        
        // Advance to next frame
        frame_index = (frame_index + 1) % frame_count();
        
        Timer::after_millis(200).await; // Animation speed
    }
}