// file: main.rs
// desc: OLED Animated BMP display with clean module organization
#![no_std]
#![no_main]

use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{self, Config};
use embassy_rp::peripherals::{DMA_CH0, PIO0, PIN_23, PIN_24, PIN_25, PIN_29, I2C0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{Peri};
use embassy_time::Timer;
use static_cell::StaticCell;

// OLED and graphics imports
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
    image::Image,
};
use tinybmp::Bmp;
use {defmt_rtt as _, panic_probe as _};

mod setup_devices;
use setup_devices::{setup_display, setup_wifi};

// Import frames modules
mod nooo; 
use nooo::{FRAMES, frame_count};

// Program metadata for `picotool info`.
const PROGRAM_NAME: &core::ffi::CStr = c"Pico 2W: Animated BMP";
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 3] = [
    embassy_rp::binary_info::rp_program_name!(PROGRAM_NAME),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());
    
    // Setup individual components
    let mut display = setup_display(p.I2C0, p.PIN_0, p.PIN_1).await;
    
    let mut wifi_controller = setup_wifi(
        p.PIO0,
        p.PIN_23,
        p.PIN_25,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
        &spawner
    ).await;
    
    // Turn on WiFi LED
    wifi_controller.gpio_set(0, true).await;
    info!("WiFi LED enabled");
    
    // Create text style
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    info!("System initialization complete!");
    info!("Starting animation with {} frames", frame_count());
    
    let mut frame_index = 0usize;
    
    // Main animation loop
    loop {
        // Clear the display
        display.clear(BinaryColor::Off).unwrap();
        
        // Draw title in the top section
        Text::new("Animation #: ", Point::new(0, 10), text_style)
            .draw(&mut display)
            .unwrap();
        
        // Get current frame data
        let current_frame_data = FRAMES[frame_index];
        
        // Parse current frame as BMP
        match Bmp::from_slice(current_frame_data) {
            Ok(bmp) => {
                // Draw the current frame centered
                let image = Image::new(&bmp, Point::new(40, 16)); // Centered for 48x48 image
                match image.draw(&mut display) {
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
        
        // Move to next frame - automatically cycles based on array length
        frame_index = (frame_index + 1) % frame_count();
        
        // Animation speed - adjust as needed
        Timer::after_millis(200).await; // 5 FPS
    }
}