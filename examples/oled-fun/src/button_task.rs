// file: button_task.rs
// desc: Handles button press updates - FIXED VERSION

use defmt::info;
use embassy_time::{Duration, Timer};
use embassy_sync::pipe::{Writer};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_futures::select::{select4, Either4};

use crate::setup_devices::Buttons;

#[embassy_executor::task]
pub async fn button_task(
    mut buttons: Buttons,
    pipe_writer: Writer<'static, CriticalSectionRawMutex, 1>
) {
    info!("Button task started");
    
    loop {
        // Wait for ANY button to be pressed using select4
        // With pull-up resistors, buttons go LOW when pressed
        match select4(
            buttons.button_1.wait_for_low(),
            buttons.button_2.wait_for_low(), 
            buttons.button_3.wait_for_low(),
            buttons.button_4.wait_for_low()
        ).await {
            Either4::First(_) => {
                info!("Button 1 pressed");
                let _ = pipe_writer.write(&[1]).await;
                buttons.button_1.wait_for_high().await;  // Wait for release
            },
            Either4::Second(_) => {
                info!("Button 2 pressed");  
                let _ = pipe_writer.write(&[2]).await;
                buttons.button_2.wait_for_high().await;  // Wait for release
            },
            Either4::Third(_) => {
                info!("Button 3 pressed");
                let _ = pipe_writer.write(&[3]).await;
                buttons.button_3.wait_for_high().await;  // Wait for release
            },
            Either4::Fourth(_) => {
                info!("Button 4 pressed");
                let _ = pipe_writer.write(&[4]).await;
                buttons.button_4.wait_for_high().await;  // Wait for release
            }
        }
        
        // Debounce delay
        Timer::after(Duration::from_millis(50)).await;
    }
}