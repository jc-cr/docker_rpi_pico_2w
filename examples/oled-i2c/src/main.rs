// file: main.rs
// desc: simple OLED i2c verification
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
};

use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
const PROGRAM_NAME: &core::ffi::CStr = c"Pico 2W: i2c OLED";
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 3] = [
    embassy_rp::binary_info::rp_program_name!(PROGRAM_NAME),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

async fn setup_wifi(
    pio0: Peri<'static, PIO0>,
    pin_23: Peri<'static, PIN_23>,
    pin_25: Peri<'static, PIN_25>, 
    pin_24: Peri<'static, PIN_24>,
    pin_29: Peri<'static, PIN_29>,
    dma_ch0: Peri<'static, DMA_CH0>,
    spawner: &Spawner
) -> cyw43::Control<'static> {
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    
    let pwr = Output::new(pin_23, Level::Low);
    let cs = Output::new(pin_25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        pin_24,
        pin_29,
        dma_ch0,
    );
    
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut wifi_controller, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));
    wifi_controller.init(clm).await;
    wifi_controller.gpio_set(0, false).await;
    info!("WiFi initialized!");
    
    wifi_controller
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Init
    let p = embassy_rp::init(Default::default());
    
    // Setup i2c
    let sda = p.PIN_0;
    let scl = p.PIN_1;
    info!("Setting up i2c on pins SDA=0, SCL=1");
    let i2c = i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, Config::default());
    
    // Setup OLED display
    info!("Initializing OLED display at address 0x3C");
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    
    // Initialize display
    match display.init() {
        Ok(_) => info!("OLED display initialized successfully"),
        Err(_) => {
            error!("Failed to initialize OLED display");
            loop {
                Timer::after_secs(1).await;
            }
        }
    }
    
    // Clear the display
    display.clear(BinaryColor::Off).unwrap();
    
    // Create text style
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    // Draw verification text
    Text::new("OLED Test", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();
    
    Text::new("I2C Working!", Point::new(0, 25), text_style)
        .draw(&mut display)
        .unwrap();
    
    Text::new("Pico 2W Ready", Point::new(0, 40), text_style)
        .draw(&mut display)
        .unwrap();
    
    Text::new("Address: 0x3C", Point::new(0, 55), text_style)
        .draw(&mut display)
        .unwrap();
    
    // Flush to display
    match display.flush() {
        Ok(_) => info!("Display updated successfully"),
        Err(_) => error!("Failed to update display"),
    }
    
    // Initialize WiFi (optional for OLED test)
    let mut wifi_controller = setup_wifi(
        p.PIO0,
        p.PIN_23,
        p.PIN_25,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
        &spawner
    ).await;
    
    wifi_controller.gpio_set(0, true).await;
    
    let mut counter = 0u32;
    
    // Simple counter display loop
    loop {
        // Update counter display
        display.clear(BinaryColor::Off).unwrap();
        
        Text::new("OLED Working!", Point::new(0, 10), text_style)
            .draw(&mut display)
            .unwrap();
        
        // Simple counter display without formatting
        let counter_text = match counter % 10 {
            0 => "Count: 0",
            1 => "Count: 1", 
            2 => "Count: 2",
            3 => "Count: 3",
            4 => "Count: 4",
            5 => "Count: 5",
            6 => "Count: 6", 
            7 => "Count: 7",
            8 => "Count: 8",
            9 => "Count: 9",
            _ => "Count: ?",
        };
        
        Text::new(counter_text, Point::new(0, 30), text_style)
            .draw(&mut display)
            .unwrap();
        
        Text::new("I2C: SDA=0 SCL=1", Point::new(0, 50), text_style)
            .draw(&mut display)
            .unwrap();
        
        // Update display
        match display.flush() {
            Ok(_) => info!("Counter: {}", counter),
            Err(_) => error!("Display flush failed"),
        }
        
        counter += 1;
        Timer::after_secs(1).await;
    }
}