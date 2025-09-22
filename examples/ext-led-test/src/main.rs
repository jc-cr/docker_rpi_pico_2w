// file: main.rs
// desc: toggle LED with button

#![no_std]
#![no_main]
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0, PIN_23, PIN_24, PIN_25, PIN_29};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{Peri}; // Add this import
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _}; // Fixed the typos here

// Program metadata for `picotool info`.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 3] = [
    embassy_rp::binary_info::rp_program_name!(c"Pico 2W External LED"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

// Correct function signature using Peri wrappers
async fn setup_wifi(
    pio0: Peri<'static, PIO0>,
    pin_23: Peri<'static, PIN_23>,
    pin_25: Peri<'static, PIN_25>, 
    pin_24: Peri<'static, PIN_24>,
    pin_29: Peri<'static, PIN_29>,
    dma_ch0: Peri<'static, DMA_CH0>,
    spawner: &Spawner
) -> cyw43::Control<'static> {
    // Include the WiFi firmware and CLM.
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
    
    // Extract the GPIO pin we need for main
    let mut gpio_led = Output::new(p.PIN_15, Level::Low);
    let mut led_on:bool = false;

    // Button
    let mut button = Input::new(p.PIN_14, Pull::None);
    
    // Pass the peripherals (which are Peri<'_, T> types)
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
    gpio_led.set_low();

    loop {
        button.wait_for_high().await;  // Wait for button press
        
        // Toggle the LED
        if !led_on {
            led_on = true; 
            gpio_led.set_high(); 
        } else {
            led_on = false;
            gpio_led.set_low();
        }
        
        button.wait_for_low().await;   // Wait for button release
        Timer::after(Duration::from_millis(50)).await;  // Debounce delay
    }
}