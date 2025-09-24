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
use embassy_time::Timer;

// Import from crate root
use crate::setup_devices::Display;
use crate::nooo::{FRAMES as NOOO_FRAMES, frame_count as nooo_frame_count};
use crate::giga::{FRAMES as GIGA_FRAMES, frame_count as giga_frame_count};
use crate::no_shake::{FRAMES as NO_SHAKE_FRAMES, frame_count as no_shake_frame_count};
use crate::reaction::{FRAMES as REACTION_FRAMES, frame_count as reaction_frame_count};


fn get_animation_data(animation_num: u8) -> (&'static [&'static [u8]], usize) {
    match animation_num {
        1 => (NOOO_FRAMES, nooo_frame_count()),
        2 => (GIGA_FRAMES, giga_frame_count()),
        3 => (NO_SHAKE_FRAMES, no_shake_frame_count()),
        4 => (REACTION_FRAMES, reaction_frame_count()),
        _ => (NOOO_FRAMES, nooo_frame_count()), // Default to first animation
    }
}


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
    
    // Get the correct animation data
    let (frames, frame_count) = get_animation_data(current_animation_num);
    
    // Make sure frame_index is valid for this animation
    let safe_frame_index = frame_index % frame_count;
    
    // Get current frame data
    let current_frame_data = frames[safe_frame_index];
    
    // Parse current frame as BMP
    match Bmp::from_slice(current_frame_data) {
        Ok(bmp) => {
            // Draw the current frame centered
            let image = Image::new(&bmp, Point::new(40, 16)); // Centered for 48x48 image
            match image.draw(display) {
                Ok(_) => {},
                Err(_) => error!("Failed to draw frame {}", safe_frame_index),
            }
        },
        Err(_) => {
            error!("Failed to parse frame {} BMP", safe_frame_index);
        }
    }
    
    // Update display
    match display.flush() {
        Ok(_) => info!("Displayed frame {}/{}", safe_frame_index + 1, frame_count),
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
    let mut frame_index = 0usize;
    let mut current_animation_num: u8 = 1;
    let mut previous_animation_num: u8 = 0; // Track animation changes
    
    // Get initial animation info
    let (_, initial_frame_count) = get_animation_data(current_animation_num);
    info!("Starting display task with animation {} ({} frames)", current_animation_num, initial_frame_count);
    
    loop {
        // Check for animation changes (non-blocking)
        current_animation_num = try_update_animation_num(current_animation_num, &mut pipe_reader);
        
        // Reset frame index when animation changes
        if current_animation_num != previous_animation_num {
            frame_index = 0;
            previous_animation_num = current_animation_num;
            let (_, frame_count) = get_animation_data(current_animation_num);
            info!("Switched to animation {} with {} frames", current_animation_num, frame_count);
        }
        
        // Display current frame
        display_frame(&mut display, current_animation_num, frame_index).await;
        
        // Advance to next frame (with bounds checking for current animation)
        let (_, frame_count) = get_animation_data(current_animation_num);
        frame_index = (frame_index + 1) % frame_count;
        
        Timer::after_millis(100).await; // Animation speed
    }
}